const fs = require('fs');
const path = require('path');

// =========================================================
// 錯誤回報工具
// =========================================================
function reportError(src, pos, msg) {
    const lines = src.split('\n');
    let currentPos = 0;
    let lineIdx = 0;
    let colIdx = 0;

    for (let i = 0; i < lines.length; i++) {
        if (currentPos + lines[i].length + 1 > pos) {
            lineIdx = i;
            colIdx = pos - currentPos;
            break;
        }
        currentPos += lines[i].length + 1;
    }

    console.error(`\n❌ [語法錯誤] 第 ${lineIdx + 1} 行, 第 ${colIdx + 1} 字元: ${msg}`);
    const lineStr = lines[lineIdx];
    console.error(`  ${lineStr}`);
    
    let indicator = "";
    for (let i = 0; i < colIdx; i++) {
        indicator += (lineStr[i] === '\t' ? '\t' : ' ');
    }
    console.error(`  ${indicator}^`);
    process.exit(1);
}

// =========================================================
// 1. 詞彙標記與中間碼
// =========================================================
const TokenType = {
    TK_FUNC: 'TK_FUNC', TK_RETURN: 'TK_RETURN', TK_IF: 'TK_IF', TK_PRINT: 'TK_PRINT',
    TK_WHILE: 'TK_WHILE', TK_FOR: 'TK_FOR', TK_BREAK: 'TK_BREAK', TK_CONTINUE: 'TK_CONTINUE',
    TK_ID: 'TK_ID', TK_NUM: 'TK_NUM', TK_STRING: 'TK_STRING',
    TK_LPAREN: 'TK_LPAREN', TK_RPAREN: 'TK_RPAREN',
    TK_LBRACE: 'TK_LBRACE', TK_RBRACE: 'TK_RBRACE',
    TK_LBRACKET: 'TK_LBRACKET', TK_RBRACKET: 'TK_RBRACKET',
    TK_DOT: 'TK_DOT', TK_COLON: 'TK_COLON',
    TK_COMMA: 'TK_COMMA', TK_SEMICOLON: 'TK_SEMICOLON',
    TK_ASSIGN: 'TK_ASSIGN', TK_PLUS: 'TK_PLUS', TK_MINUS: 'TK_MINUS', 
    TK_MUL: 'TK_MUL', TK_DIV: 'TK_DIV',
    TK_EQ: 'TK_EQ', TK_LT: 'TK_LT', TK_GT: 'TK_GT',
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
// 2. 詞法分析 (Lexer)
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
            while (this.pos < this.src.length && /\s/.test(this.src[this.pos])) this.pos++;
            
            if (this.pos >= this.src.length) {
                this.curToken = new Token(TokenType.TK_EOF, "", this.pos);
                return;
            }

            // 處理註解
            if (this.src[this.pos] === '/') {
                if (this.pos + 1 < this.src.length && this.src[this.pos + 1] === '/') {
                    this.pos += 2;
                    while (this.pos < this.src.length && this.src[this.pos] !== '\n') this.pos++;
                    continue;
                } else if (this.pos + 1 < this.src.length && this.src[this.pos + 1] === '*') {
                    this.pos += 2;
                    while (this.pos + 1 < this.src.length && !(this.src[this.pos] === '*' && this.src[this.pos + 1] === '/')) this.pos++;
                    if (this.pos + 1 < this.src.length) this.pos += 2;
                    continue;
                }
            }
            break;
        }

        let start = this.pos;

        // 處理字串
        if (this.src[this.pos] === '"') {
            this.pos++;
            let startStr = this.pos;
            while (this.pos < this.src.length && this.src[this.pos] !== '"') this.pos++;
            if (this.pos >= this.src.length) reportError(this.src, start, "字串缺少結尾的雙引號 '\"'");
            let text = this.src.substring(startStr, this.pos);
            this.pos++;
            this.curToken = new Token(TokenType.TK_STRING, text, start);
            return;
        }

        // 處理數字
        if (/\d/.test(this.src[this.pos])) {
            while (this.pos < this.src.length && /\d/.test(this.src[this.pos])) this.pos++;
            this.curToken = new Token(TokenType.TK_NUM, this.src.substring(start, this.pos), start);
            return;
        }

        // 處理識別碼與關鍵字
        if (/[a-zA-Z_]/.test(this.src[this.pos])) {
            while (this.pos < this.src.length && /[a-zA-Z0-9_]/.test(this.src[this.pos])) this.pos++;
            let text = this.src.substring(start, this.pos);
            const keywords = {
                "func": TokenType.TK_FUNC, "return": TokenType.TK_RETURN,
                "if": TokenType.TK_IF, "print": TokenType.TK_PRINT,
                "while": TokenType.TK_WHILE, "for": TokenType.TK_FOR,
                "break": TokenType.TK_BREAK, "continue": TokenType.TK_CONTINUE
            };
            this.curToken = new Token(keywords[text] || TokenType.TK_ID, text, start);
            return;
        }

        // 符號
        let ch = this.src[this.pos];
        this.pos++;
        const symbols = {
            '(': TokenType.TK_LPAREN, ')': TokenType.TK_RPAREN, '{': TokenType.TK_LBRACE, '}': TokenType.TK_RBRACE,
            '[': TokenType.TK_LBRACKET, ']': TokenType.TK_RBRACKET, '.': TokenType.TK_DOT, ':': TokenType.TK_COLON,
            '+': TokenType.TK_PLUS, '-': TokenType.TK_MINUS, '*': TokenType.TK_MUL, '/': TokenType.TK_DIV,
            ',': TokenType.TK_COMMA, ';': TokenType.TK_SEMICOLON, '<': TokenType.TK_LT, '>': TokenType.TK_GT
        };

        if (symbols[ch]) {
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
// 3. 語法解析 (Parser)
// =========================================================
class Parser {
    constructor(lexer) {
        this.lexer = lexer;
        this.quads = [];
        this.stringPool = [];
        this.loopStack = [];
        this.tIdx = 0;
    }

    get cur() { return this.lexer.curToken; }

    consume() { this.lexer.nextToken(); }

    error(msg) {
        reportError(this.lexer.src, this.cur.pos, `${msg} (目前讀到: '${this.cur.text}')`);
    }

    expect(type, msg) {
        if (this.cur.type === type) this.consume();
        else this.error(msg);
    }

    newT() {
        return `t${++this.tIdx}`;
    }

    emit(op, a1, a2, res) {
        let idx = this.quads.length;
        this.quads.push(new Quad(op, a1, a2, res));
        console.log(`${idx.toString().padStart(3, '0')}: ${op.padEnd(12)} ${a1.padEnd(10)} ${a2.padEnd(10)} ${res.padEnd(10)}`);
        return idx;
    }

    exprOrAssign() {
        let name = this.cur.text;
        this.consume();
        let obj = name;
        let pathList = [];

        while ([TokenType.TK_LBRACKET, TokenType.TK_DOT, TokenType.TK_LPAREN].includes(this.cur.type)) {
            if (this.cur.type === TokenType.TK_LBRACKET) {
                this.consume();
                let idxExpr = this.expression();
                this.expect(TokenType.TK_RBRACKET, "預期 ']'");
                pathList.push(idxExpr);
            } else if (this.cur.type === TokenType.TK_DOT) {
                this.consume();
                if (this.cur.type !== TokenType.TK_ID) this.error("預期屬性名稱");
                let keyStr = this.cur.text;
                this.consume();
                let k = this.newT();
                let poolIdx = this.stringPool.length;
                this.stringPool.push(keyStr);
                this.emit("LOAD_STR", poolIdx.toString(), "-", k);
                pathList.push(k);
            } else if (this.cur.type === TokenType.TK_LPAREN) {
                for (let p of pathList) {
                    let t = this.newT();
                    this.emit("GET_ITEM", obj, p, t);
                    obj = t;
                }
                pathList = [];
                this.consume();
                let count = 0;
                if (this.cur.type !== TokenType.TK_RPAREN) {
                    while (true) {
                        let arg = this.expression();
                        this.emit("PARAM", arg, "-", "-");
                        count++;
                        if (this.cur.type === TokenType.TK_COMMA) this.consume();
                        else break;
                    }
                }
                this.expect(TokenType.TK_RPAREN, "預期 ')'");
                let tCall = this.newT();
                this.emit("CALL", obj, count.toString(), tCall);
                obj = tCall;
            }
        }

        if (this.cur.type === TokenType.TK_ASSIGN) {
            this.consume();
            let val = this.expression();
            if (pathList.length === 0) {
                this.emit("STORE", val, "-", obj);
            } else {
                for (let i = 0; i < pathList.length - 1; i++) {
                    let t = this.newT();
                    this.emit("GET_ITEM", obj, pathList[i], t);
                    obj = t;
                }
                this.emit("SET_ITEM", obj, pathList[pathList.length - 1], val);
            }
        }
    }

    primary() {
        if (this.cur.type === TokenType.TK_NUM) {
            let t = this.newT();
            this.emit("IMM", this.cur.text, "-", t);
            this.consume();
            return t;
        } else if (this.cur.type === TokenType.TK_STRING) {
            let t = this.newT();
            let poolIdx = this.stringPool.length;
            this.stringPool.push(this.cur.text);
            this.emit("LOAD_STR", poolIdx.toString(), "-", t);
            this.consume();
            return t;
        } else if (this.cur.type === TokenType.TK_ID) {
            let name = this.cur.text;
            this.consume();
            return name;
        } else if (this.cur.type === TokenType.TK_LBRACKET) {
            this.consume();
            let tArr = this.newT();
            this.emit("NEW_ARR", "-", "-", tArr);
            if (this.cur.type !== TokenType.TK_RBRACKET) {
                while (true) {
                    let val = this.expression();
                    this.emit("APPEND_ITEM", tArr, "-", val);
                    if (this.cur.type === TokenType.TK_COMMA) this.consume();
                    else break;
                }
            }
            this.expect(TokenType.TK_RBRACKET, "陣列預期要有 ']' 結尾");
            return tArr;
        } else if (this.cur.type === TokenType.TK_LBRACE) {
            this.consume();
            let tDict = this.newT();
            this.emit("NEW_DICT", "-", "-", tDict);
            if (this.cur.type !== TokenType.TK_RBRACE) {
                while (true) {
                    let k;
                    if (this.cur.type === TokenType.TK_ID) {
                        let keyStr = this.cur.text;
                        this.consume();
                        k = this.newT();
                        let poolIdx = this.stringPool.length;
                        this.stringPool.push(keyStr);
                        this.emit("LOAD_STR", poolIdx.toString(), "-", k);
                    } else if (this.cur.type === TokenType.TK_STRING) {
                        k = this.primary();
                    } else {
                        this.error("字典的鍵(Key)必須是字串或識別碼");
                    }
                    this.expect(TokenType.TK_COLON, "字典預期要有 ':' 分隔鍵值");
                    let val = this.expression();
                    this.emit("SET_ITEM", tDict, k, val);
                    if (this.cur.type === TokenType.TK_COMMA) this.consume();
                    else break;
                }
            }
            this.expect(TokenType.TK_RBRACE, "字典預期要有 '}' 結尾");
            return tDict;
        } else if (this.cur.type === TokenType.TK_LPAREN) {
            this.consume();
            let res = this.expression();
            this.expect(TokenType.TK_RPAREN, "括號表達式結尾預期要有 ')'");
            return res;
        } else {
            this.error("表達式中出現預期外的語法結構");
        }
    }

    factor() {
        let res = this.primary();
        while ([TokenType.TK_LBRACKET, TokenType.TK_DOT, TokenType.TK_LPAREN].includes(this.cur.type)) {
            if (this.cur.type === TokenType.TK_LBRACKET) {
                this.consume();
                let idxExpr = this.expression();
                this.expect(TokenType.TK_RBRACKET, "預期 ']'");
                let t = this.newT();
                this.emit("GET_ITEM", res, idxExpr, t);
                res = t;
            } else if (this.cur.type === TokenType.TK_DOT) {
                this.consume();
                let keyStr = this.cur.text;
                this.consume();
                let k = this.newT();
                let poolIdx = this.stringPool.length;
                this.stringPool.push(keyStr);
                this.emit("LOAD_STR", poolIdx.toString(), "-", k);
                let t = this.newT();
                this.emit("GET_ITEM", res, k, t);
                res = t;
            } else if (this.cur.type === TokenType.TK_LPAREN) {
                this.consume();
                let count = 0;
                if (this.cur.type !== TokenType.TK_RPAREN) {
                    while (true) {
                        let arg = this.expression();
                        this.emit("PARAM", arg, "-", "-");
                        count++;
                        if (this.cur.type === TokenType.TK_COMMA) this.consume();
                        else break;
                    }
                }
                this.expect(TokenType.TK_RPAREN, "預期 ')'");
                let tCall = this.newT();
                this.emit("CALL", res, count.toString(), tCall);
                res = tCall;
            }
        }
        return res;
    }

    term() {
        let l = this.factor();
        while (this.cur.type === TokenType.TK_MUL || this.cur.type === TokenType.TK_DIV) {
            let op = this.cur.type === TokenType.TK_MUL ? "MUL" : "DIV";
            this.consume();
            let r = this.factor();
            let t = this.newT();
            this.emit(op, l, r, t);
            l = t;
        }
        return l;
    }

    arithExpr() {
        let l = this.term();
        while (this.cur.type === TokenType.TK_PLUS || this.cur.type === TokenType.TK_MINUS) {
            let op = this.cur.type === TokenType.TK_PLUS ? "ADD" : "SUB";
            this.consume();
            let r = this.term();
            let t = this.newT();
            this.emit(op, l, r, t);
            l = t;
        }
        return l;
    }

    expression() {
        let l = this.arithExpr();
        if ([TokenType.TK_EQ, TokenType.TK_LT, TokenType.TK_GT].includes(this.cur.type)) {
            let op = this.cur.type === TokenType.TK_EQ ? "CMP_EQ" : (this.cur.type === TokenType.TK_LT ? "CMP_LT" : "CMP_GT");
            this.consume();
            let r = this.arithExpr();
            let t = this.newT();
            this.emit(op, l, r, t);
            return t;
        }
        return l;
    }

    statement() {
        if (this.cur.type === TokenType.TK_IF) {
            this.consume();
            this.expect(TokenType.TK_LPAREN, "預期 '('");
            let cond = this.expression();
            this.expect(TokenType.TK_RPAREN, "預期 ')'");
            this.expect(TokenType.TK_LBRACE, "預期 '{'");
            let jmpFIdx = this.emit("JMP_F", cond, "-", "?");
            while (this.cur.type !== TokenType.TK_RBRACE && this.cur.type !== TokenType.TK_EOF) this.statement();
            this.expect(TokenType.TK_RBRACE, "預期 '}'");
            this.quads[jmpFIdx].result = this.quads.length.toString();
        } else if (this.cur.type === TokenType.TK_WHILE) {
            this.consume();
            this.expect(TokenType.TK_LPAREN, "預期 '('");
            let condIdx = this.quads.length;
            let cond = this.expression();
            this.expect(TokenType.TK_RPAREN, "預期 ')'");
            this.expect(TokenType.TK_LBRACE, "預期 '{'");
            let jmpFIdx = this.emit("JMP_F", cond, "-", "?");
            this.loopStack.push({ break: [], continue: condIdx });
            while (this.cur.type !== TokenType.TK_RBRACE && this.cur.type !== TokenType.TK_EOF) this.statement();
            this.emit("JMP", "-", "-", condIdx.toString());
            this.expect(TokenType.TK_RBRACE, "預期 '}'");
            let endIdx = this.quads.length;
            this.quads[jmpFIdx].result = endIdx.toString();
            let loopCtx = this.loopStack.pop();
            for (let bIdx of loopCtx.break) this.quads[bIdx].result = endIdx.toString();
        } else if (this.cur.type === TokenType.TK_FOR) {
            this.consume();
            this.expect(TokenType.TK_LPAREN, "預期 '('");
            if (this.cur.type !== TokenType.TK_SEMICOLON) this.exprOrAssign();
            this.expect(TokenType.TK_SEMICOLON, "預期 ';'");
            let condIdx = this.quads.length;
            let cond;
            if (this.cur.type !== TokenType.TK_SEMICOLON) {
                cond = this.expression();
            } else {
                cond = this.newT();
                this.emit("IMM", "1", "-", cond);
            }
            let jmpFIdx = this.emit("JMP_F", cond, "-", "?");
            let jmpBodyIdx = this.emit("JMP", "-", "-", "?");
            this.expect(TokenType.TK_SEMICOLON, "預期 ';'");
            let stepIdx = this.quads.length;
            if (this.cur.type !== TokenType.TK_RPAREN) this.exprOrAssign();
            this.emit("JMP", "-", "-", condIdx.toString());
            this.expect(TokenType.TK_RPAREN, "預期 ')'");
            this.expect(TokenType.TK_LBRACE, "預期 '{'");
            this.quads[jmpBodyIdx].result = this.quads.length.toString();
            this.loopStack.push({ break: [], continue: stepIdx });
            while (this.cur.type !== TokenType.TK_RBRACE && this.cur.type !== TokenType.TK_EOF) this.statement();
            this.emit("JMP", "-", "-", stepIdx.toString());
            this.expect(TokenType.TK_RBRACE, "預期 '}'");
            let endIdx = this.quads.length;
            this.quads[jmpFIdx].result = endIdx.toString();
            let loopCtx = this.loopStack.pop();
            for (let bIdx of loopCtx.break) this.quads[bIdx].result = endIdx.toString();
        } else if (this.cur.type === TokenType.TK_BREAK) {
            this.consume();
            if (this.loopStack.length === 0) this.error("break 必須在迴圈內部使用");
            let bIdx = this.emit("JMP", "-", "-", "?");
            this.loopStack[this.loopStack.length - 1].break.push(bIdx);
            this.expect(TokenType.TK_SEMICOLON, "預期 ';'");
        } else if (this.cur.type === TokenType.TK_CONTINUE) {
            this.consume();
            if (this.loopStack.length === 0) this.error("continue 必須在迴圈內部使用");
            let cTarget = this.loopStack[this.loopStack.length - 1].continue;
            this.emit("JMP", "-", "-", cTarget.toString());
            this.expect(TokenType.TK_SEMICOLON, "預期 ';'");
        } else if (this.cur.type === TokenType.TK_ID) {
            this.exprOrAssign();
            this.expect(TokenType.TK_SEMICOLON, "預期 ';'");
        } else if (this.cur.type === TokenType.TK_RETURN) {
            this.consume();
            let res = this.expression();
            this.emit("RET_VAL", res, "-", "-");
            this.expect(TokenType.TK_SEMICOLON, "預期 ';'");
        } else if (this.cur.type === TokenType.TK_PRINT) {
            this.consume();
            this.expect(TokenType.TK_LPAREN, "預期 '('");
            if (this.cur.type !== TokenType.TK_RPAREN) {
                while (true) {
                    let val = this.expression();
                    this.emit("PRINT_VAL", val, "-", "-");
                    if (this.cur.type === TokenType.TK_COMMA) this.consume();
                    else break;
                }
            }
            this.emit("PRINT_NL", "-", "-", "-");
            this.expect(TokenType.TK_RPAREN, "預期 ')'");
            this.expect(TokenType.TK_SEMICOLON, "預期 ';'");
        } else {
            this.error("無法辨識的陳述句或語法結構");
        }
    }

    parseProgram() {
        while (this.cur.type !== TokenType.TK_EOF) {
            if (this.cur.type === TokenType.TK_FUNC) {
                this.consume();
                let fName = this.cur.text;
                this.consume();
                this.emit("FUNC_BEG", fName, "-", "-");
                this.expect(TokenType.TK_LPAREN, "預期 '('");
                if (this.cur.type !== TokenType.TK_RPAREN) {
                    while (true) {
                        this.emit("FORMAL", this.cur.text, "-", "-");
                        this.consume();
                        if (this.cur.type === TokenType.TK_COMMA) this.consume();
                        else break;
                    }
                }
                this.expect(TokenType.TK_RPAREN, "預期 ')'");
                this.expect(TokenType.TK_LBRACE, "預期 '{'");
                while (this.cur.type !== TokenType.TK_RBRACE && this.cur.type !== TokenType.TK_EOF) this.statement();
                this.emit("FUNC_END", fName, "-", "-");
                this.expect(TokenType.TK_RBRACE, "預期 '}'");
            } else {
                this.statement();
            }
        }
    }
}

// =========================================================
// 4. 虛擬機 (Virtual Machine)
// =========================================================
class Frame {
    constructor(retPc = 0, retVar = "") {
        this.vars = {};
        this.retPc = retPc;
        this.retVar = retVar;
        this.incomingArgs = [];
        this.formalIdx = 0;
    }
}

class VM {
    constructor(quads, stringPool) {
        this.quads = quads;
        this.stringPool = stringPool;
        this.stack = [new Frame()];
        this.sp = 0;
        this.printBuf = [];
    }

    getVar(name) {
        if (/^-?\d+$/.test(name)) return parseInt(name);
        if (name === "-") return 0;
        let val = this.stack[this.sp].vars[name];
        return val === undefined ? 0 : val;
    }

    setVar(name, val) {
        this.stack[this.sp].vars[name] = val;
    }

    run() {
        let pc = 0;
        let paramStack = [];
        let funcMap = {};
        this.quads.forEach((q, i) => {
            if (q.op === "FUNC_BEG") funcMap[q.arg1] = i + 1;
        });

        console.log("\n=== VM 執行開始 ===");
        while (pc < this.quads.length) {
            let q = this.quads[pc];
            try {
                if (q.op === "FUNC_BEG") {
                    while (this.quads[pc].op !== "FUNC_END") pc++;
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
                    let d = this.getVar(q.arg2);
                    this.setVar(q.result, Math.floor(this.getVar(q.arg1) / (d === 0 ? 1 : d)));
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
                    if (this.getVar(q.arg1) === 0) pc = parseInt(q.result) - 1;
                } else if (q.op === "PRINT_VAL") {
                    this.printBuf.push(this.getVar(q.arg1).toString());
                } else if (q.op === "PRINT_NL") {
                    console.log("[程式輸出] >> " + this.printBuf.join(" "));
                    this.printBuf = [];
                } else if (q.op === "PARAM") {
                    paramStack.push(this.getVar(q.arg1));
                } else if (q.op === "CALL") {
                    let pCount = parseInt(q.arg2);
                    let valArg1 = this.getVar(q.arg1);
                    let fName = (typeof valArg1 === 'string') ? valArg1 : q.arg1;
                    let targetPc = funcMap[fName];
                    if (targetPc === undefined) throw new Error(`找不到函數 '${fName}'`);

                    let newFrame = new Frame(pc + 1, q.result);
                    if (pCount > 0) {
                        newFrame.incomingArgs = paramStack.slice(-pCount);
                        paramStack.splice(-pCount);
                    }
                    this.stack.push(newFrame);
                    this.sp++;
                    pc = targetPc;
                    continue;
                } else if (q.op === "FORMAL") {
                    let frame = this.stack[this.sp];
                    this.setVar(q.arg1, frame.incomingArgs[frame.formalIdx++]);
                } else if (q.op === "RET_VAL") {
                    let retVal = this.getVar(q.arg1);
                    let currentFrame = this.stack.pop();
                    this.sp--;
                    this.setVar(currentFrame.retVar, retVal);
                    pc = currentFrame.retPc;
                    continue;
                } else if (q.op === "FUNC_END") {
                    if (this.sp > 0) {
                        let currentFrame = this.stack.pop();
                        this.sp--;
                        this.setVar(currentFrame.retVar, 0);
                        pc = currentFrame.retPc;
                        continue;
                    }
                }
            } catch (e) {
                console.error(`\n[VM 執行時期錯誤] 發生在指令列 ${pc.toString().padStart(3, '0')} (${q.op}): ${e.message}`);
                process.exit(1);
            }
            pc++;
        }

        console.log("=== VM 執行完畢 ===\n\n記憶體狀態 (全域變數):");
        Object.entries(this.stack[0].vars).forEach(([name, val]) => {
            if (!name.startsWith('t')) console.log(`[${name}] = ${JSON.stringify(val)}`);
        });
    }
}

// =========================================================
// 主程式入口
// =========================================================
function main() {
    const args = process.argv.slice(2);
    if (args.length < 1) {
        console.log(`用法: node ${path.basename(__filename)} <source_file>`);
        process.exit(1);
    }

    let sourceCode;
    try {
        sourceCode = fs.readFileSync(args[0], 'utf-8');
    } catch (e) {
        console.error(`無法開啟檔案: ${e.message}`);
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

main();