const fs = require('fs');
const { spawnSync } = require('child_process');

// =========================================================
// 程式語言 EBNF (Extended Backus-Naur Form) 語法定義
// =========================================================
// program ::= ( function_def | statement )*
//
// function_def ::= "func" id "(" [ id ("," id)* ] ")" "{" statement* "}"
//
// statement ::= "if" "(" expression ")" "{" statement* "}"[ "else" "{" statement* "}" ]
//             | "while" "(" expression ")" "{" statement* "}"
//             | "for" "(" [expr_or_assign] ";" [expression] ";"[expr_or_assign] ")" "{" statement* "}"
//             | "break" ";"
//             | "continue" ";"
//             | "return" expression ";"
//             | expr_or_assign ";"
//
// expr_or_assign ::= id { "[" expression "]" | "." id | "(" [ expression ("," expression)* ] ")" }[ "=" expression ]
//
// expression ::= arith_expr[ ("==" | "<" | ">") arith_expr ]
//
// arith_expr ::= term ( ("+" | "-") term )*
//
// term ::= factor ( ("*" | "/") factor )*
//
// factor ::= primary { "[" expression "]" | "." id | "(" [ expression ("," expression)* ] ")" }
//
// primary ::= num 
//           | string 
//           | id 
//           | "[" [ expression (";" expression | ("," expression)* ) ] "]" 
//           | "{"[ (id|string) ":" expression ("," (id|string) ":" expression)* ] "}" 
//           | "(" expression ")"
// =========================================================


// =========================================================
// 錯誤回報工具：定位原始碼位置並輸出指標 ^ 符號
// =========================================================
function reportError(src, pos, msg) {
    const lines = src.split('\n');
    let currentPos = 0;
    let lineIdx = 0;
    
    for (let i = 0; i < lines.length; i++) {
        const l = lines[i];
        if (currentPos + l.length + 1 > pos) {
            lineIdx = i;
            break;
        }
        currentPos += l.length + 1;
    }
    
    let colIdx = pos - currentPos;
    if (lineIdx >= lines.length) {
        lineIdx = lines.length - 1;
        colIdx = lines[lineIdx].length;
    }
    
    console.log(`\n❌ [語法錯誤] 第 ${lineIdx + 1} 行, 第 ${colIdx + 1} 字元: ${msg}`);
    const lineStr = lines[lineIdx];
    console.log(`  ${lineStr}`);
    
    // 根據 Tab 或空格對齊指標
    let indicator = '';
    for (let i = 0; i < colIdx; i++) {
        indicator += (i < lineStr.length && lineStr[i] === '\t') ? '\t' : ' ';
    }
    indicator += '^';
    console.log(`  ${indicator}`);
    process.exit(1);
}


// =========================================================
// 1. 詞彙標記與中間碼 (Intermediate Representation)
// =========================================================
const TokenType = {
    // 關鍵字
    TK_FUNC: 'TK_FUNC',
    TK_RETURN: 'TK_RETURN',
    TK_IF: 'TK_IF',
    TK_ELSE: 'TK_ELSE',
    TK_WHILE: 'TK_WHILE',
    TK_FOR: 'TK_FOR',
    TK_BREAK: 'TK_BREAK',
    TK_CONTINUE: 'TK_CONTINUE',
    // 識別碼、常數
    TK_ID: 'TK_ID',
    TK_NUM: 'TK_NUM',
    TK_STRING: 'TK_STRING',
    // 符號
    TK_LPAREN: 'TK_LPAREN',
    TK_RPAREN: 'TK_RPAREN',
    TK_LBRACE: 'TK_LBRACE',
    TK_RBRACE: 'TK_RBRACE',
    TK_LBRACKET: 'TK_LBRACKET',
    TK_RBRACKET: 'TK_RBRACKET',
    TK_DOT: 'TK_DOT',
    TK_COLON: 'TK_COLON',
    TK_COMMA: 'TK_COMMA',
    TK_SEMICOLON: 'TK_SEMICOLON',
    // 運算子
    TK_ASSIGN: 'TK_ASSIGN',
    TK_PLUS: 'TK_PLUS',
    TK_MINUS: 'TK_MINUS',
    TK_MUL: 'TK_MUL',
    TK_DIV: 'TK_DIV',
    TK_EQ: 'TK_EQ',
    TK_LT: 'TK_LT',
    TK_GT: 'TK_GT',
    TK_EOF: 'TK_EOF'
};

class Token {
    constructor(type, text, pos) {
        this.type = type;
        this.text = text;
        this.pos = pos;
    }
}

class Quad {
    constructor(op, arg1, arg2, result) {
        this.op = op;
        this.arg1 = arg1;
        this.arg2 = arg2;
        this.result = result;
    }
}


// =========================================================
// 2. 詞法分析 (Lexer)：將原始字串轉為 Token 流
// =========================================================
class Lexer {
    constructor(src) {
        this.src = src;
        this.pos = 0;
        this.curToken = null;
        this.nextToken();
    }
    
    nextToken() {
        while (true) {
            // 跳過空格
            while (this.pos < this.src.length && /\s/.test(this.src[this.pos])) {
                this.pos++;
            }
            
            if (this.pos >= this.src.length) {
                this.curToken = new Token(TokenType.TK_EOF, "", this.pos);
                return;
            }
            
            // 處理單行註解 (//) 或多行註解 (/* */)
            if (this.src[this.pos] === '/') {
                if (this.pos + 1 < this.src.length && this.src[this.pos + 1] === '/') {
                    this.pos += 2;
                    while (this.pos < this.src.length && this.src[this.pos] !== '\n') {
                        this.pos++;
                    }
                    continue;
                } else if (this.pos + 1 < this.src.length && this.src[this.pos + 1] === '*') {
                    this.pos += 2;
                    while (this.pos + 1 < this.src.length && 
                           !(this.src[this.pos] === '*' && this.src[this.pos + 1] === '/')) {
                        this.pos++;
                    }
                    if (this.pos + 1 < this.src.length) {
                        this.pos += 2;
                    }
                    continue;
                }
            }
            break;
        }
        
        const start = this.pos;
        
        // 處理字串常數 "..."
        if (this.src[this.pos] === '"') {
            this.pos++;
            const startStr = this.pos;
            while (this.pos < this.src.length && this.src[this.pos] !== '"') {
                this.pos++;
            }
            if (this.pos >= this.src.length) {
                reportError(this.src, start, "字串缺少結尾的雙引號 '\"'");
            }
            const text = this.src.substring(startStr, this.pos);
            this.pos++;
            this.curToken = new Token(TokenType.TK_STRING, text, start);
            return;
        }
        
        // 處理數字
        if (/\d/.test(this.src[this.pos])) {
            while (this.pos < this.src.length && /\d/.test(this.src[this.pos])) {
                this.pos++;
            }
            this.curToken = new Token(TokenType.TK_NUM, this.src.substring(start, this.pos), start);
            return;
        }
        
        // 處理關鍵字與變數名稱 (Identifier)
        if (/[a-zA-Z_]/.test(this.src[this.pos])) {
            while (this.pos < this.src.length && /[a-zA-Z0-9_]/.test(this.src[this.pos])) {
                this.pos++;
            }
            const text = this.src.substring(start, this.pos);
            const keywords = {
                "func": TokenType.TK_FUNC,
                "return": TokenType.TK_RETURN,
                "if": TokenType.TK_IF,
                "else": TokenType.TK_ELSE,
                "while": TokenType.TK_WHILE,
                "for": TokenType.TK_FOR,
                "break": TokenType.TK_BREAK,
                "continue": TokenType.TK_CONTINUE
            };
            this.curToken = new Token(keywords[text] || TokenType.TK_ID, text, start);
            return;
        }
        
        // 處理單/雙字元符號與運算子
        const ch = this.src[this.pos];
        this.pos++;
        
        const symbols = {
            '(': TokenType.TK_LPAREN,
            ')': TokenType.TK_RPAREN,
            '{': TokenType.TK_LBRACE,
            '}': TokenType.TK_RBRACE,
            '[': TokenType.TK_LBRACKET,
            ']': TokenType.TK_RBRACKET,
            '.': TokenType.TK_DOT,
            ':': TokenType.TK_COLON,
            '+': TokenType.TK_PLUS,
            '-': TokenType.TK_MINUS,
            '*': TokenType.TK_MUL,
            '/': TokenType.TK_DIV,
            ',': TokenType.TK_COMMA,
            ';': TokenType.TK_SEMICOLON,
            '<': TokenType.TK_LT,
            '>': TokenType.TK_GT
        };
        
        if (ch in symbols) {
            this.curToken = new Token(symbols[ch], ch, start);
        } else if (ch === '=') {
            if (this.pos < this.src.length && this.src[this.pos] === '=') {
                this.pos++;
                this.curToken = new Token(TokenType.TK_EQ, "==", start);
            } else {
                this.curToken = new Token(TokenType.TK_ASSIGN, "=", start);
            }
        } else {
            reportError(this.src, start, `無法辨識的字元: '${ch}'`);
        }
    }
}


// =========================================================
// 3. 語法解析 (Parser)：將 Token 流轉為四位組 (Quads)
// =========================================================
class Parser {
    constructor(lexer) {
        this.lexer = lexer;
        this.quads = [];
        this.stringPool = [];
        this.tempCount = 0;
        this.loopStack = [];
    }
    
    newTemp() {
        return `t${this.tempCount++}`;
    }
    
    emit(op, arg1 = "", arg2 = "", result = "") {
        this.quads.push(new Quad(op, arg1, arg2, result));
        console.log(`${String(this.quads.length - 1).padStart(3, '0')}: ${op.padEnd(12)} ${String(arg1).padEnd(8)} ${String(arg2).padEnd(8)} ${result}`);
    }
    
    expect(tokenType) {
        if (this.lexer.curToken.type !== tokenType) {
            reportError(this.lexer.src, this.lexer.curToken.pos, 
                `預期 ${tokenType}，實際收到 ${this.lexer.curToken.type}`);
        }
        const tok = this.lexer.curToken;
        this.lexer.nextToken();
        return tok;
    }
    
    match(tokenType) {
        return this.lexer.curToken.type === tokenType;
    }
    
    parseProgram() {
        while (!this.match(TokenType.TK_EOF)) {
            if (this.match(TokenType.TK_FUNC)) {
                this.parseFunctionDef();
            } else {
                this.parseStatement();
            }
        }
    }
    
    parseFunctionDef() {
        this.expect(TokenType.TK_FUNC);
        const funcName = this.expect(TokenType.TK_ID).text;
        this.expect(TokenType.TK_LPAREN);
        
        const params = [];
        if (!this.match(TokenType.TK_RPAREN)) {
            params.push(this.expect(TokenType.TK_ID).text);
            while (this.match(TokenType.TK_COMMA)) {
                this.expect(TokenType.TK_COMMA);
                params.push(this.expect(TokenType.TK_ID).text);
            }
        }
        this.expect(TokenType.TK_RPAREN);
        
        this.emit("FUNC_BEG", funcName, "", "");
        for (const p of params) {
            this.emit("FORMAL", p, "", "");
        }
        
        this.expect(TokenType.TK_LBRACE);
        while (!this.match(TokenType.TK_RBRACE)) {
            this.parseStatement();
        }
        this.expect(TokenType.TK_RBRACE);
        this.emit("FUNC_END", "", "", "");
    }
    
    parseStatement() {
        if (this.match(TokenType.TK_IF)) {
            this.parseIfStatement();
        } else if (this.match(TokenType.TK_WHILE)) {
            this.parseWhileStatement();
        } else if (this.match(TokenType.TK_FOR)) {
            this.parseForStatement();
        } else if (this.match(TokenType.TK_BREAK)) {
            this.parseBreakStatement();
        } else if (this.match(TokenType.TK_CONTINUE)) {
            this.parseContinueStatement();
        } else if (this.match(TokenType.TK_RETURN)) {
            this.parseReturnStatement();
        } else {
            this.parseExprOrAssign();
            this.expect(TokenType.TK_SEMICOLON);
        }
    }
    
    parseIfStatement() {
        this.expect(TokenType.TK_IF);
        this.expect(TokenType.TK_LPAREN);
        const cond = this.parseExpression();
        this.expect(TokenType.TK_RPAREN);
        
        const holeJmpF = this.quads.length;
        this.emit("JMP_F", cond, "", "?");
        
        this.expect(TokenType.TK_LBRACE);
        while (!this.match(TokenType.TK_RBRACE)) {
            this.parseStatement();
        }
        this.expect(TokenType.TK_RBRACE);
        
        if (this.match(TokenType.TK_ELSE)) {
            const holeJmp = this.quads.length;
            this.emit("JMP", "", "", "?");
            this.quads[holeJmpF].result = String(this.quads.length);
            
            this.expect(TokenType.TK_ELSE);
            this.expect(TokenType.TK_LBRACE);
            while (!this.match(TokenType.TK_RBRACE)) {
                this.parseStatement();
            }
            this.expect(TokenType.TK_RBRACE);
            this.quads[holeJmp].result = String(this.quads.length);
        } else {
            this.quads[holeJmpF].result = String(this.quads.length);
        }
    }
    
    parseWhileStatement() {
        this.expect(TokenType.TK_WHILE);
        const loopStart = this.quads.length;
        
        this.expect(TokenType.TK_LPAREN);
        const cond = this.parseExpression();
        this.expect(TokenType.TK_RPAREN);
        
        const holeJmpF = this.quads.length;
        this.emit("JMP_F", cond, "", "?");
        
        this.loopStack.push({ breaks: [], continues: [], start: loopStart });
        
        this.expect(TokenType.TK_LBRACE);
        while (!this.match(TokenType.TK_RBRACE)) {
            this.parseStatement();
        }
        this.expect(TokenType.TK_RBRACE);
        
        this.emit("JMP", "", "", String(loopStart));
        const loopEnd = this.quads.length;
        this.quads[holeJmpF].result = String(loopEnd);
        
        const loopInfo = this.loopStack.pop();
        for (const idx of loopInfo.breaks) {
            this.quads[idx].result = String(loopEnd);
        }
        for (const idx of loopInfo.continues) {
            this.quads[idx].result = String(loopStart);
        }
    }
    
    parseForStatement() {
        this.expect(TokenType.TK_FOR);
        this.expect(TokenType.TK_LPAREN);
        
        if (!this.match(TokenType.TK_SEMICOLON)) {
            this.parseExprOrAssign();
        }
        this.expect(TokenType.TK_SEMICOLON);
        
        const loopStart = this.quads.length;
        let cond = null;
        if (!this.match(TokenType.TK_SEMICOLON)) {
            cond = this.parseExpression();
        }
        this.expect(TokenType.TK_SEMICOLON);
        
        let holeJmpF = -1;
        if (cond !== null) {
            holeJmpF = this.quads.length;
            this.emit("JMP_F", cond, "", "?");
        }
        
        const holeJmpBody = this.quads.length;
        this.emit("JMP", "", "", "?");
        
        const incrementStart = this.quads.length;
        if (!this.match(TokenType.TK_RPAREN)) {
            this.parseExprOrAssign();
        }
        this.expect(TokenType.TK_RPAREN);
        this.emit("JMP", "", "", String(loopStart));
        
        this.quads[holeJmpBody].result = String(this.quads.length);
        this.loopStack.push({ breaks: [], continues: [], start: incrementStart });
        
        this.expect(TokenType.TK_LBRACE);
        while (!this.match(TokenType.TK_RBRACE)) {
            this.parseStatement();
        }
        this.expect(TokenType.TK_RBRACE);
        
        this.emit("JMP", "", "", String(incrementStart));
        const loopEnd = this.quads.length;
        
        if (holeJmpF !== -1) {
            this.quads[holeJmpF].result = String(loopEnd);
        }
        
        const loopInfo = this.loopStack.pop();
        for (const idx of loopInfo.breaks) {
            this.quads[idx].result = String(loopEnd);
        }
        for (const idx of loopInfo.continues) {
            this.quads[idx].result = String(incrementStart);
        }
    }
    
    parseBreakStatement() {
        this.expect(TokenType.TK_BREAK);
        this.expect(TokenType.TK_SEMICOLON);
        
        if (this.loopStack.length === 0) {
            reportError(this.lexer.src, this.lexer.curToken.pos, "break 只能在迴圈內使用");
        }
        
        const idx = this.quads.length;
        this.emit("JMP", "", "", "?");
        this.loopStack[this.loopStack.length - 1].breaks.push(idx);
    }
    
    parseContinueStatement() {
        this.expect(TokenType.TK_CONTINUE);
        this.expect(TokenType.TK_SEMICOLON);
        
        if (this.loopStack.length === 0) {
            reportError(this.lexer.src, this.lexer.curToken.pos, "continue 只能在迴圈內使用");
        }
        
        const idx = this.quads.length;
        this.emit("JMP", "", "", "?");
        this.loopStack[this.loopStack.length - 1].continues.push(idx);
    }
    
    parseReturnStatement() {
        this.expect(TokenType.TK_RETURN);
        const val = this.parseExpression();
        this.expect(TokenType.TK_SEMICOLON);
        this.emit("RET_VAL", val, "", "");
    }
    
    parseExprOrAssign() {
        const idTok = this.lexer.curToken;
        if (!this.match(TokenType.TK_ID)) {
            this.parseExpression();
            return;
        }
        
        this.expect(TokenType.TK_ID);
        let target = idTok.text;
        let accessChain = [];
        
        while (this.match(TokenType.TK_LBRACKET) || this.match(TokenType.TK_DOT) || this.match(TokenType.TK_LPAREN)) {
            if (this.match(TokenType.TK_LBRACKET)) {
                this.expect(TokenType.TK_LBRACKET);
                const idx = this.parseExpression();
                this.expect(TokenType.TK_RBRACKET);
                accessChain.push({ type: 'index', value: idx });
            } else if (this.match(TokenType.TK_DOT)) {
                this.expect(TokenType.TK_DOT);
                const key = this.expect(TokenType.TK_ID).text;
                const keyIdx = this.stringPool.length;
                this.stringPool.push(key);
                const keyTemp = this.newTemp();
                this.emit("LOAD_STR", String(keyIdx), "", keyTemp);
                accessChain.push({ type: 'index', value: keyTemp });
            } else if (this.match(TokenType.TK_LPAREN)) {
                this.expect(TokenType.TK_LPAREN);
                const args = [];
                if (!this.match(TokenType.TK_RPAREN)) {
                    args.push(this.parseExpression());
                    while (this.match(TokenType.TK_COMMA)) {
                        this.expect(TokenType.TK_COMMA);
                        args.push(this.parseExpression());
                    }
                }
                this.expect(TokenType.TK_RPAREN);
                accessChain.push({ type: 'call', args: args });
            }
        }
        
        if (this.match(TokenType.TK_ASSIGN)) {
            this.expect(TokenType.TK_ASSIGN);
            const rhs = this.parseExpression();
            
            if (accessChain.length === 0) {
                this.emit("STORE", rhs, "", target);
            } else {
                let base = target;
                for (let i = 0; i < accessChain.length - 1; i++) {
                    const access = accessChain[i];
                    if (access.type === 'index') {
                        const temp = this.newTemp();
                        this.emit("GET_ITEM", base, access.value, temp);
                        base = temp;
                    }
                }
                
                const lastAccess = accessChain[accessChain.length - 1];
                if (lastAccess.type === 'index') {
                    this.emit("SET_ITEM", base, lastAccess.value, rhs);
                }
            }
        } else {
            if (accessChain.length > 0) {
                const lastAccess = accessChain[accessChain.length - 1];
                if (lastAccess.type === 'call') {
                    for (const arg of lastAccess.args) {
                        this.emit("PARAM", arg, "", "");
                    }
                    const retTemp = this.newTemp();
                    this.emit("CALL", target, String(lastAccess.args.length), retTemp);
                }
            }
        }
    }
    
    parseExpression() {
        const left = this.parseArithExpr();
        
        if (this.match(TokenType.TK_EQ) || this.match(TokenType.TK_LT) || this.match(TokenType.TK_GT)) {
            const opTok = this.lexer.curToken;
            this.lexer.nextToken();
            const right = this.parseArithExpr();
            const temp = this.newTemp();
            
            const opMap = {
                [TokenType.TK_EQ]: "CMP_EQ",
                [TokenType.TK_LT]: "CMP_LT",
                [TokenType.TK_GT]: "CMP_GT"
            };
            this.emit(opMap[opTok.type], left, right, temp);
            return temp;
        }
        
        return left;
    }
    
    parseArithExpr() {
        let left = this.parseTerm();
        
        while (this.match(TokenType.TK_PLUS) || this.match(TokenType.TK_MINUS)) {
            const opTok = this.lexer.curToken;
            this.lexer.nextToken();
            const right = this.parseTerm();
            const temp = this.newTemp();
            
            if (opTok.type === TokenType.TK_PLUS) {
                this.emit("ADD", left, right, temp);
            } else {
                this.emit("SUB", left, right, temp);
            }
            left = temp;
        }
        
        return left;
    }
    
    parseTerm() {
        let left = this.parseFactor();
        
        while (this.match(TokenType.TK_MUL) || this.match(TokenType.TK_DIV)) {
            const opTok = this.lexer.curToken;
            this.lexer.nextToken();
            const right = this.parseFactor();
            const temp = this.newTemp();
            
            if (opTok.type === TokenType.TK_MUL) {
                this.emit("MUL", left, right, temp);
            } else {
                this.emit("DIV", left, right, temp);
            }
            left = temp;
        }
        
        return left;
    }
    
    parseFactor() {
        let base = this.parsePrimary();
        
        while (this.match(TokenType.TK_LBRACKET) || this.match(TokenType.TK_DOT) || this.match(TokenType.TK_LPAREN)) {
            if (this.match(TokenType.TK_LBRACKET)) {
                this.expect(TokenType.TK_LBRACKET);
                const idx = this.parseExpression();
                this.expect(TokenType.TK_RBRACKET);
                const temp = this.newTemp();
                this.emit("GET_ITEM", base, idx, temp);
                base = temp;
            } else if (this.match(TokenType.TK_DOT)) {
                this.expect(TokenType.TK_DOT);
                const key = this.expect(TokenType.TK_ID).text;
                const keyIdx = this.stringPool.length;
                this.stringPool.push(key);
                const keyTemp = this.newTemp();
                this.emit("LOAD_STR", String(keyIdx), "", keyTemp);
                const temp = this.newTemp();
                this.emit("GET_ITEM", base, keyTemp, temp);
                base = temp;
            } else if (this.match(TokenType.TK_LPAREN)) {
                this.expect(TokenType.TK_LPAREN);
                const args = [];
                if (!this.match(TokenType.TK_RPAREN)) {
                    args.push(this.parseExpression());
                    while (this.match(TokenType.TK_COMMA)) {
                        this.expect(TokenType.TK_COMMA);
                        args.push(this.parseExpression());
                    }
                }
                this.expect(TokenType.TK_RPAREN);
                
                for (const arg of args) {
                    this.emit("PARAM", arg, "", "");
                }
                const retTemp = this.newTemp();
                this.emit("CALL", base, String(args.length), retTemp);
                base = retTemp;
            }
        }
        
        return base;
    }
    
    parsePrimary() {
        if (this.match(TokenType.TK_NUM)) {
            const numTok = this.expect(TokenType.TK_NUM);
            const temp = this.newTemp();
            this.emit("IMM", numTok.text, "", temp);
            return temp;
        } else if (this.match(TokenType.TK_STRING)) {
            const strTok = this.expect(TokenType.TK_STRING);
            const idx = this.stringPool.length;
            this.stringPool.push(strTok.text);
            const temp = this.newTemp();
            this.emit("LOAD_STR", String(idx), "", temp);
            return temp;
        } else if (this.match(TokenType.TK_ID)) {
            const idTok = this.expect(TokenType.TK_ID);
            return idTok.text;
        } else if (this.match(TokenType.TK_LBRACKET)) {
            this.expect(TokenType.TK_LBRACKET);
            
            if (this.match(TokenType.TK_RBRACKET)) {
                this.expect(TokenType.TK_RBRACKET);
                const temp = this.newTemp();
                this.emit("NEW_ARR", "", "", temp);
                return temp;
            }
            
            const first = this.parseExpression();
            
            if (this.match(TokenType.TK_SEMICOLON)) {
                this.expect(TokenType.TK_SEMICOLON);
                const size = this.parseExpression();
                this.expect(TokenType.TK_RBRACKET);
                const temp = this.newTemp();
                this.emit("INIT_ARR", first, size, temp);
                return temp;
            } else {
                const arrTemp = this.newTemp();
                this.emit("NEW_ARR", "", "", arrTemp);
                this.emit("APPEND_ITEM", arrTemp, "", first);
                
                while (this.match(TokenType.TK_COMMA)) {
                    this.expect(TokenType.TK_COMMA);
                    const elem = this.parseExpression();
                    this.emit("APPEND_ITEM", arrTemp, "", elem);
                }
                this.expect(TokenType.TK_RBRACKET);
                return arrTemp;
            }
        } else if (this.match(TokenType.TK_LBRACE)) {
            this.expect(TokenType.TK_LBRACE);
            const dictTemp = this.newTemp();
            this.emit("NEW_DICT", "", "", dictTemp);
            
            if (!this.match(TokenType.TK_RBRACE)) {
                while (true) {
                    let keyTemp;
                    if (this.match(TokenType.TK_ID)) {
                        const keyTok = this.expect(TokenType.TK_ID);
                        const keyIdx = this.stringPool.length;
                        this.stringPool.push(keyTok.text);
                        keyTemp = this.newTemp();
                        this.emit("LOAD_STR", String(keyIdx), "", keyTemp);
                    } else if (this.match(TokenType.TK_STRING)) {
                        const keyTok = this.expect(TokenType.TK_STRING);
                        const keyIdx = this.stringPool.length;
                        this.stringPool.push(keyTok.text);
                        keyTemp = this.newTemp();
                        this.emit("LOAD_STR", String(keyIdx), "", keyTemp);
                    } else {
                        reportError(this.lexer.src, this.lexer.curToken.pos, 
                            "字典的鍵值必須是識別碼或字串");
                    }
                    
                    this.expect(TokenType.TK_COLON);
                    const val = this.parseExpression();
                    this.emit("SET_ITEM", dictTemp, keyTemp, val);
                    
                    if (!this.match(TokenType.TK_COMMA)) break;
                    this.expect(TokenType.TK_COMMA);
                }
            }
            this.expect(TokenType.TK_RBRACE);
            return dictTemp;
        } else if (this.match(TokenType.TK_LPAREN)) {
            this.expect(TokenType.TK_LPAREN);
            const expr = this.parseExpression();
            this.expect(TokenType.TK_RPAREN);
            return expr;
        } else {
            reportError(this.lexer.src, this.lexer.curToken.pos, 
                `無法解析的表達式，當前 Token: ${this.lexer.curToken.type}`);
        }
    }
}


// =========================================================
// 4. 執行時期環境：Frame (函式堆疊框架)
// =========================================================
class Frame {
    constructor(retPc = -1, retVar = "") {
        this.vars = {};
        this.retPc = retPc;
        this.retVar = retVar;
        this.incomingArgs = [];
        this.formalIdx = 0;
    }
}


// =========================================================
// 5. 虛擬機 (VM)：執行四位組中間碼
// =========================================================
class VM {
    constructor(quads, stringPool) {
        this.quads = quads;
        this.stringPool = stringPool;
        this.stack = [new Frame()];
        this.sp = 0;
    }
    
    getVar(name) {
        for (let i = this.sp; i >= 0; i--) {
            if (name in this.stack[i].vars) {
                return this.stack[i].vars[name];
            }
        }
        throw new Error(`未定義變數: ${name}`);
    }
    
    setVar(name, value) {
        this.stack[this.sp].vars[name] = value;
    }
    
    systemCall(fName, args) {
        if (fName === "print") {
            const output = args.map(a => {
                if (Array.isArray(a)) {
                    return '[' + a.join(', ') + ']';
                } else if (typeof a === 'object' && a !== null) {
                    return JSON.stringify(a);
                } else {
                    return String(a);
                }
            }).join(' ');
            console.log(output);
            return [true, 0];
        } else if (fName === "array") {
            if (args.length !== 2) {
                throw new Error("array 需 2 個參數 (長度, 預設值)");
            }
            if (!Number.isInteger(args[0])) {
                throw new Error("array 長度需為整數");
            }
            return [true, new Array(args[0]).fill(args[1])];
        } else if (fName === "len") {
            if (args.length !== 1) {
                throw new Error("len 需 1 個參數");
            }
            let length;
            if (Array.isArray(args[0])) {
                length = args[0].length;
            } else if (typeof args[0] === 'object' && args[0] !== null) {
                length = Object.keys(args[0]).length;
            } else if (typeof args[0] === 'string') {
                length = args[0].length;
            } else {
                length = 0;
            }
            return [true, length];
        } else if (fName === "push") {
            args[0].push(args[1]);
            return [true, args[0]];
        } else if (fName === "pop") {
            return [true, args[0].pop()];
        } else if (fName === "keys") {
            return [true, Object.keys(args[0])];
        } else if (fName === "has_key") {
            return [true, args[1] in args[0] ? 1 : 0];
        } else if (fName === "remove") {
            if (args[1] in args[0]) {
                delete args[0][args[1]];
            }
            return [true, args[0]];
        } else if (fName === "typeof") {
            const val = args[0];
            let tStr;
            if (Number.isInteger(val)) {
                tStr = "int";
            } else if (typeof val === 'string') {
                tStr = "string";
            } else if (Array.isArray(val)) {
                tStr = "array";
            } else if (typeof val === 'object') {
                tStr = "dict";
            } else {
                tStr = "unknown";
            }
            return [true, tStr];
        } else if (fName === "int") {
            return [true, parseInt(args[0])];
        } else if (fName === "str") {
            return [true, String(args[0])];
        } else if (fName === "ord") {
            return [true, args[0].charCodeAt(0)];
        } else if (fName === "chr") {
            return [true, String.fromCharCode(args[0])];
        } else if (fName === "input") {
            const msg = args.length > 0 ? String(args[0]) : "";
            
            // 在非管道模式下使用 shell 的 read 命令實現真正的同步輸入
            if (process.stdin.isTTY) {
                // TTY 模式：使用 bash read 命令
                process.stdout.write(msg);
                try {
                    const result = spawnSync('bash', ['-c', 'read line && echo "$line"'], {
                        stdio: ['inherit', 'pipe', 'inherit'],
                        encoding: 'utf8'
                    });
                    
                    if (result.status === 0 && result.stdout) {
                        return [true, result.stdout.trim()];
                    }
                    return [true, ""];
                } catch (e) {
                    return [true, ""];
                }
            } else {
                // 管道模式：直接從 stdin 讀取
                process.stdout.write(msg);
                const buffer = Buffer.alloc(4096);
                
                try {
                    const bytesRead = fs.readSync(process.stdin.fd, buffer, 0, 4096);
                    if (bytesRead > 0) {
                        const text = buffer.toString('utf8', 0, bytesRead);
                        const lines = text.split('\n');
                        return [true, lines[0].replace(/\r/g, '')];
                    }
                    return [true, ""];
                } catch (e) {
                    return [true, ""];
                }
            }
        } else if (fName === "time") {
            return [true, Date.now() / 1000];
        } else if (fName === "random") {
            return [true, Math.random()];
        } else if (fName === "exit") {
            const code = args.length > 0 ? args[0] : 0;
            process.exit(code);
        }
        
        return [false, null];
    }
    
    run() {
        let pc = 0;
        const paramStack = [];
        
        // 建立函式名稱到入口 PC 的映射表
        const funcMap = {};
        for (let i = 0; i < this.quads.length; i++) {
            if (this.quads[i].op === "FUNC_BEG") {
                funcMap[this.quads[i].arg1] = i + 1;
            }
        }
        
        console.log("\n=== VM 執行開始 ===");
        
        while (pc < this.quads.length) {
            const q = this.quads[pc];
            
            try {
                // 函式定義在主執行緒中只會略過
                if (q.op === "FUNC_BEG") {
                    while (this.quads[pc].op !== "FUNC_END") {
                        pc++;
                    }
                } else if (q.op === "IMM") {
                    this.setVar(q.result, parseInt(q.arg1));
                } else if (q.op === "LOAD_STR") {
                    this.setVar(q.result, this.stringPool[parseInt(q.arg1)]);
                } else if (q.op === "ADD") {
                    this.setVar(q.result, this.getVar(q.arg1) + this.getVar(q.arg2));
                } else if (q.op === "SUB") {
                    this.setVar(q.result, this.getVar(q.arg1) - this.getVar(q.arg2));
                } else if (q.op === "MUL") {
                    this.setVar(q.result, this.getVar(q.arg1) * this.getVar(q.arg2));
                } else if (q.op === "DIV") {
                    this.setVar(q.result, Math.floor(this.getVar(q.arg1) / Math.max(this.getVar(q.arg2), 1)));
                } else if (q.op === "CMP_EQ") {
                    this.setVar(q.result, this.getVar(q.arg1) === this.getVar(q.arg2) ? 1 : 0);
                } else if (q.op === "CMP_LT") {
                    this.setVar(q.result, this.getVar(q.arg1) < this.getVar(q.arg2) ? 1 : 0);
                } else if (q.op === "CMP_GT") {
                    this.setVar(q.result, this.getVar(q.arg1) > this.getVar(q.arg2) ? 1 : 0);
                } else if (q.op === "STORE") {
                    this.setVar(q.result, this.getVar(q.arg1));
                } else if (q.op === "NEW_ARR") {
                    this.setVar(q.result, []);
                } else if (q.op === "INIT_ARR") {
                    const initVal = this.getVar(q.arg1);
                    const arrSize = this.getVar(q.arg2);
                    if (!Number.isInteger(arrSize)) {
                        throw new Error("陣列長度必須是整數");
                    }
                    this.setVar(q.result, new Array(arrSize).fill(initVal));
                } else if (q.op === "NEW_DICT") {
                    this.setVar(q.result, {});
                } else if (q.op === "APPEND_ITEM") {
                    this.getVar(q.arg1).push(this.getVar(q.result));
                } else if (q.op === "SET_ITEM") {
                    this.getVar(q.arg1)[this.getVar(q.arg2)] = this.getVar(q.result);
                } else if (q.op === "GET_ITEM") {
                    this.setVar(q.result, this.getVar(q.arg1)[this.getVar(q.arg2)]);
                } else if (q.op === "JMP") {
                    pc = parseInt(q.result) - 1;
                } else if (q.op === "JMP_F") {
                    if (this.getVar(q.arg1) === 0) {
                        pc = parseInt(q.result) - 1;
                    }
                } else if (q.op === "PARAM") {
                    paramStack.push(this.getVar(q.arg1));
                } else if (q.op === "CALL") {
                    const pCount = parseInt(q.arg2);
                    
                    // 嘗試獲取函數名稱：可能是變數或字串字面量
                    let fName;
                    try {
                        const varVal = this.getVar(q.arg1);
                        fName = typeof varVal === 'string' ? varVal : q.arg1;
                    } catch (e) {
                        // 如果 getVar 失敗，則直接使用 arg1 作為函數名稱
                        fName = q.arg1;
                    }
                    
                    const args = pCount > 0 ? paramStack.slice(-pCount) : [];
                    
                    const [isNative, retVal] = this.systemCall(fName, args);
                    if (isNative) {
                        if (pCount > 0) {
                            paramStack.splice(-pCount);
                        }
                        this.setVar(q.result, retVal);
                        pc++;
                        continue;
                    }
                    
                    const targetPc = funcMap[fName];
                    if (targetPc === undefined) {
                        throw new Error(`找不到函數 '${fName}'`);
                    }
                    
                    const newFrame = new Frame(pc + 1, q.result);
                    if (pCount > 0) {
                        newFrame.incomingArgs = args;
                        paramStack.splice(-pCount);
                    }
                    this.stack.push(newFrame);
                    this.sp++;
                    pc = targetPc;
                    continue;
                } else if (q.op === "FORMAL") {
                    const frame = this.stack[this.sp];
                    this.setVar(q.arg1, frame.incomingArgs[frame.formalIdx]);
                    frame.formalIdx++;
                } else if (q.op === "RET_VAL") {
                    const retVal = this.getVar(q.arg1);
                    const retAddress = this.stack[this.sp].retPc;
                    const targetVar = this.stack[this.sp].retVar;
                    this.stack.pop();
                    this.sp--;
                    this.setVar(targetVar, retVal);
                    pc = retAddress;
                    continue;
                } else if (q.op === "FUNC_END") {
                    if (this.sp > 0) {
                        const retAddress = this.stack[this.sp].retPc;
                        const targetVar = this.stack[this.sp].retVar;
                        this.stack.pop();
                        this.sp--;
                        this.setVar(targetVar, 0);
                        pc = retAddress;
                        continue;
                    }
                }
            } catch (e) {
                console.log(`\n[VM 執行時期錯誤] 發生在指令列 ${String(pc).padStart(3, '0')} (${q.op}): ${e.message}`);
                process.exit(1);
            }
            
            pc++;
        }
        
        console.log("=== VM 執行完畢 ===\n\n記憶體狀態 (全域變數):");
        for (const [name, val] of Object.entries(this.stack[0].vars)) {
            if (!name.startsWith('t')) {
                let displayVal;
                if (Array.isArray(val)) {
                    displayVal = '[' + val.join(', ') + ']';
                } else if (typeof val === 'object' && val !== null) {
                    displayVal = JSON.stringify(val);
                } else {
                    displayVal = val;
                }
                console.log(`[${name}] = ${displayVal}`);
            }
        }
    }
}


// =========================================================
// 主程式進入點
// =========================================================
function main() {
    if (process.argv.length < 3) {
        console.log(`用法: node ${process.argv[1]} <source_file>`);
        process.exit(1);
    }
    
    let sourceCode;
    try {
        sourceCode = fs.readFileSync(process.argv[2], 'utf-8');
    } catch (e) {
        console.log(`無法開啟檔案: ${e.message}`);
        process.exit(1);
    }
    
    console.log("編譯器生成的中間碼 (PC: Quadruples):");
    console.log("-".repeat(44));
    
    const lexer = new Lexer(sourceCode);
    const parser = new Parser(lexer);
    parser.parseProgram();
    
    const vm = new VM(parser.quads, parser.stringPool);
    vm.run();
}

if (require.main === module) {
    main();
}

module.exports = { Lexer, Parser, VM, Token, Quad, TokenType };