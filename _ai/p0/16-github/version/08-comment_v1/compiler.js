const fs = require('fs');

// =========================================================
// 1. 詞彙標記 (Tokens) 與 中間碼 (Quadruples) 定義
// =========================================================

// 利用 Object 模擬 Enum 來定義 Token 種類
const TokenType = {
    TK_FUNC: 'TK_FUNC', TK_RETURN: 'TK_RETURN', TK_IF: 'TK_IF', TK_ID: 'TK_ID', TK_NUM: 'TK_NUM',
    TK_LPAREN: 'TK_LPAREN', TK_RPAREN: 'TK_RPAREN', TK_LBRACE: 'TK_LBRACE', TK_RBRACE: 'TK_RBRACE',
    TK_COMMA: 'TK_COMMA', TK_SEMICOLON: 'TK_SEMICOLON',
    TK_ASSIGN: 'TK_ASSIGN', TK_PLUS: 'TK_PLUS', TK_MINUS: 'TK_MINUS', TK_MUL: 'TK_MUL', TK_DIV: 'TK_DIV',
    TK_EQ: 'TK_EQ', TK_LT: 'TK_LT', TK_GT: 'TK_GT', TK_EOF: 'TK_EOF'
};

class Token {
    constructor(type, text) {
        this.type = type;
        this.text = text;
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
// 原理：將字串切分成標記。透過 this.pos 索引游標前進。
// =========================================================

// 輔助函式：判斷字元類型
const isSpace = (ch) => /\s/.test(ch);
const isDigit = (ch) => /^[0-9]$/.test(ch);
const isAlpha = (ch) => /^[a-zA-Z]$/.test(ch);
const isAlnum = (ch) => /^[a-zA-Z0-9]$/.test(ch);

class Lexer {
    constructor(src) {
        this.src = src;
        this.pos = 0;
        this.cur_token = null;
        this.nextToken(); // 預讀第一個 Token
    }

    nextToken() {
        while (true) {
            // 忽略空格、換行
            while (this.pos < this.src.length && isSpace(this.src[this.pos])) {
                this.pos++;
            }
            
            // 處理結尾
            if (this.pos >= this.src.length) {
                this.cur_token = new Token(TokenType.TK_EOF, "");
                return;
            }

            // 處理註解
            if (this.src[this.pos] === '/') {
                // 單行註解 //
                if (this.pos + 1 < this.src.length && this.src[this.pos + 1] === '/') {
                    this.pos += 2;
                    while (this.pos < this.src.length && this.src[this.pos] !== '\n') {
                        this.pos++;
                    }
                    continue;
                }
                // 多行註解 /* ... */
                else if (this.pos + 1 < this.src.length && this.src[this.pos + 1] === '*') {
                    this.pos += 2;
                    while (this.pos + 1 < this.src.length && !(this.src[this.pos] === '*' && this.src[this.pos + 1] === '/')) {
                        this.pos++;
                    }
                    if (this.pos + 1 < this.src.length) this.pos += 2; // 跳過 */
                    continue;
                }
            }
            break;
        }

        let start = this.pos;

        // 辨識數字 (NUM)
        if (isDigit(this.src[this.pos])) {
            while (this.pos < this.src.length && isDigit(this.src[this.pos])) {
                this.pos++;
            }
            this.cur_token = new Token(TokenType.TK_NUM, this.src.substring(start, this.pos));
            return;
        }

        // 辨識識別碼 (ID) 與 關鍵字 (Keyword)
        if (isAlpha(this.src[this.pos]) || this.src[this.pos] === '_') {
            while (this.pos < this.src.length && (isAlnum(this.src[this.pos]) || this.src[this.pos] === '_')) {
                this.pos++;
            }
            let text = this.src.substring(start, this.pos);
            
            const keywords = {
                "func": TokenType.TK_FUNC,
                "return": TokenType.TK_RETURN,
                "if": TokenType.TK_IF
            };
            this.cur_token = new Token(keywords[text] || TokenType.TK_ID, text);
            return;
        }

        // 辨識運算符與符號
        let ch = this.src[this.pos++];
        const symbols = {
            '(': TokenType.TK_LPAREN, ')': TokenType.TK_RPAREN,
            '{': TokenType.TK_LBRACE, '}': TokenType.TK_RBRACE,
            '+': TokenType.TK_PLUS,   '-': TokenType.TK_MINUS,
            '*': TokenType.TK_MUL,    '/': TokenType.TK_DIV,
            ',': TokenType.TK_COMMA,  ';': TokenType.TK_SEMICOLON,
            '<': TokenType.TK_LT,     '>': TokenType.TK_GT
        };
        
        if (symbols[ch]) {
            this.cur_token = new Token(symbols[ch], ch);
        } else if (ch === '=') {
            if (this.pos < this.src.length && this.src[this.pos] === '=') {
                this.pos++;
                this.cur_token = new Token(TokenType.TK_EQ, "==");
            } else {
                this.cur_token = new Token(TokenType.TK_ASSIGN, "=");
            }
        } else {
            throw new Error(`未知的字元: ${ch}`);
        }
    }
}

// =========================================================
// 3. 語法解析 (Parser) - 遞迴下降法
// =========================================================
class Parser {
    constructor(lexer) {
        this.lexer = lexer;
        this.quads =[];
        this.t_idx = 0;
    }

    get cur() {
        return this.lexer.cur_token;
    }

    consume() {
        this.lexer.nextToken();
    }

    newT() {
        this.t_idx++;
        return `t${this.t_idx}`;
    }

    emit(op, a1, a2, res) {
        this.quads.push(new Quad(op, a1, a2, res));
        // 使用 padStart 和 padEnd 進行排版，讓終端機輸出對齊整齊
        let idx = String(this.quads.length - 1).padStart(3, '0');
        console.log(`${idx}: ${op.padEnd(10)} ${a1.padEnd(10)} ${a2.padEnd(10)} ${res.padEnd(10)}`);
    }

    factor() {
        let res = "";
        if (this.cur.type === TokenType.TK_NUM) {
            res = this.newT();
            this.emit("IMM", this.cur.text, "-", res);
            this.consume();
        } else if (this.cur.type === TokenType.TK_ID) {
            let name = this.cur.text;
            this.consume();
            if (this.cur.type === TokenType.TK_LPAREN) { // 函數呼叫
                this.consume();
                let count = 0;
                while (this.cur.type !== TokenType.TK_RPAREN) {
                    let arg = this.expression();
                    this.emit("PARAM", arg, "-", "-");
                    count++;
                    if (this.cur.type === TokenType.TK_COMMA) this.consume();
                }
                this.consume();
                res = this.newT();
                this.emit("CALL", name, String(count), res);
            } else {
                res = name; // 單純變數
            }
        } else if (this.cur.type === TokenType.TK_LPAREN) {
            this.consume();
            res = this.expression();
            this.consume();
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
            let op;
            if (this.cur.type === TokenType.TK_EQ) op = "CMP_EQ";
            else if (this.cur.type === TokenType.TK_LT) op = "CMP_LT";
            else op = "CMP_GT";
            
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
            this.consume(); this.consume(); // if, (
            let cond = this.expression();
            this.consume(); this.consume(); // ), {
            
            let jmp_idx = this.quads.length;
            this.emit("JMP_F", cond, "-", "?"); // Backpatching 預留空位
            
            while (this.cur.type !== TokenType.TK_RBRACE) {
                this.statement();
            }
            this.consume(); // }
            
            // 回填真實跳轉地址
            this.quads[jmp_idx].result = String(this.quads.length);
            
        } else if (this.cur.type === TokenType.TK_ID) { // 賦值語句
            let name = this.cur.text;
            this.consume();
            if (this.cur.type === TokenType.TK_ASSIGN) {
                this.consume();
                let res = this.expression();
                this.emit("STORE", res, "-", name);
                if (this.cur.type === TokenType.TK_SEMICOLON) this.consume();
            }
        } else if (this.cur.type === TokenType.TK_RETURN) { // 回傳語句
            this.consume();
            let res = this.expression();
            this.emit("RET_VAL", res, "-", "-");
            if (this.cur.type === TokenType.TK_SEMICOLON) this.consume();
        }
    }

    parseProgram() {
        while (this.cur.type !== TokenType.TK_EOF) {
            if (this.cur.type === TokenType.TK_FUNC) {
                this.consume();
                let f_name = this.cur.text;
                this.emit("FUNC_BEG", f_name, "-", "-");
                this.consume(); this.consume(); // name, (
                
                while (this.cur.type === TokenType.TK_ID) {
                    this.emit("FORMAL", this.cur.text, "-", "-");
                    this.consume();
                    if (this.cur.type === TokenType.TK_COMMA) this.consume();
                }
                this.consume(); this.consume(); // ), {
                
                while (this.cur.type !== TokenType.TK_RBRACE) {
                    this.statement();
                }
                this.emit("FUNC_END", f_name, "-", "-");
                this.consume(); // }
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
    constructor(ret_pc = 0, ret_var = "") {
        this.vars = new Map(); // 使用 Map 保留變數存入的順序
        this.ret_pc = ret_pc;
        this.ret_var = ret_var;
        this.incoming_args =[];
        this.formal_idx = 0;
    }
}

class VM {
    constructor(quads) {
        this.quads = quads;
        this.stack = [new Frame()]; // 初始化全域環境
        this.sp = 0;
    }

    getVar(name) {
        // 使用 Regex 判斷字串是否為整數
        if (/^-?\d+$/.test(name)) return parseInt(name, 10);
        if (name === "-") return 0;
        return this.stack[this.sp].vars.get(name) || 0; // 若找不到預設為 0
    }

    setVar(name, val) {
        this.stack[this.sp].vars.set(name, val);
    }

    run() {
        let pc = 0;
        let param_stack =[];
        
        // 預掃描：記錄函數進入點 (FUNC_BEG 的下一行)
        let func_map = new Map();
        this.quads.forEach((q, i) => {
            if (q.op === "FUNC_BEG") {
                func_map.set(q.arg1, i + 1);
            }
        });

        console.log("\n=== VM 執行開始 ===");

        while (pc < this.quads.length) {
            let q = this.quads[pc];

            switch (q.op) {
                case "FUNC_BEG":
                    // 遇到函數宣告，直接跳過直到 FUNC_END
                    while (this.quads[pc].op !== "FUNC_END") pc++;
                    break;
                case "IMM": this.setVar(q.result, parseInt(q.arg1, 10)); break;
                case "ADD": this.setVar(q.result, this.getVar(q.arg1) + this.getVar(q.arg2)); break;
                case "SUB": this.setVar(q.result, this.getVar(q.arg1) - this.getVar(q.arg2)); break;
                case "MUL": this.setVar(q.result, this.getVar(q.arg1) * this.getVar(q.arg2)); break;
                case "DIV": 
                    // JS 使用 Math.trunc 確保像 C 語言一樣丟棄小數，並防止除以 0
                    let divisor = this.getVar(q.arg2);
                    this.setVar(q.result, Math.trunc(this.getVar(q.arg1) / (divisor === 0 ? 1 : divisor))); 
                    break;
                case "CMP_EQ": this.setVar(q.result, this.getVar(q.arg1) === this.getVar(q.arg2) ? 1 : 0); break;
                case "CMP_LT": this.setVar(q.result, this.getVar(q.arg1) < this.getVar(q.arg2) ? 1 : 0); break;
                case "CMP_GT": this.setVar(q.result, this.getVar(q.arg1) > this.getVar(q.arg2) ? 1 : 0); break;
                case "STORE": this.setVar(q.result, this.getVar(q.arg1)); break;
                case "JMP_F":
                    if (this.getVar(q.arg1) === 0) {
                        pc = parseInt(q.result, 10) - 1;
                    }
                    break;
                case "PARAM":
                    param_stack.push(this.getVar(q.arg1));
                    break;
                case "CALL":
                    let p_count = parseInt(q.arg2, 10);
                    let target_pc = func_map.get(q.arg1);
                    
                    let new_frame = new Frame(pc + 1, q.result);
                    
                    // 從參數暫存區取得參數，利用 splice 一次取出並從原陣列刪除
                    if (p_count > 0) {
                        new_frame.incoming_args = param_stack.splice(-p_count);
                    }
                    
                    this.stack.push(new_frame);
                    this.sp++;
                    pc = target_pc;
                    continue; // 直接跳入目標位置，不執行 pc++
                case "FORMAL":
                    let frame = this.stack[this.sp];
                    this.setVar(q.arg1, frame.incoming_args[frame.formal_idx]);
                    frame.formal_idx++;
                    break;
                case "RET_VAL":
                    let ret_val = this.getVar(q.arg1);
                    let ret_address = this.stack[this.sp].ret_pc;
                    let target_var = this.stack[this.sp].ret_var;
                    
                    this.stack.pop(); // 銷毀當前堆疊幀
                    this.sp--;        // 回到 Caller
                    
                    this.setVar(target_var, ret_val); // 將回傳值寫入母函數的變數域中
                    pc = ret_address;
                    continue;
            }
            pc++;
        }

        console.log("=== VM 執行完畢 ===\n\n全域變數結果:");
        // 迭代全域作用域 Map (保證插入順序)
        for (let [name, val] of this.stack[0].vars.entries()) {
            if (!name.startsWith('t')) { // 忽略內部產生的 t1, t2 暫存變數
                console.log(`>> ${name} = ${val}`);
            }
        }
    }
}

// =========================================================
// 讀取檔案與主程式
// =========================================================
function main() {
    const args = process.argv.slice(2);
    if (args.length < 1) {
        console.log(`用法: node compiler.js <source_file>`);
        process.exit(1);
    }

    let sourceCode = '';
    try {
        // 同步讀取檔案內容
        sourceCode = fs.readFileSync(args[0], 'utf-8');
    } catch (err) {
        console.error(`無法開啟檔案: ${err.message}`);
        process.exit(1);
    }

    console.log("編譯器生成的中間碼 (PC: Quadruples):");
    console.log("--------------------------------------------");
    
    // 初始化解析流程
    const lexer = new Lexer(sourceCode);
    const parser = new Parser(lexer);
    
    // 進行語法解析並產生 Quadruples
    parser.parseProgram();
    
    // 傳給虛擬機執行
    const vm = new VM(parser.quads);
    vm.run();
}

// 執行主程式
if (require.main === module) {
    main();
}