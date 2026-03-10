use std::cell::RefCell;
use std::collections::HashMap;
use std::env;
use std::fmt;
use std::fs;
use std::process;
use std::rc::Rc;

// =========================================================
// 錯誤回報工具
// =========================================================
fn report_error(src: &str, pos: usize, msg: &str) -> ! {
    let lines: Vec<&str> = src.split('\n').collect();
    let mut current_pos = 0;
    let mut line_idx = 0;

    for (i, l) in lines.iter().enumerate() {
        let chars_count = l.chars().count();
        if current_pos + chars_count + 1 > pos {
            line_idx = i;
            break;
        }
        current_pos += chars_count + 1;
    }

    let mut col_idx = if pos >= current_pos { pos - current_pos } else { 0 };
    if line_idx >= lines.len() {
        line_idx = lines.len() - 1;
        col_idx = lines[line_idx].chars().count();
    }

    println!("\n❌ [語法錯誤] 第 {} 行, 第 {} 字元: {}", line_idx + 1, col_idx + 1, msg);
    let line_str = lines[line_idx];
    println!("  {}", line_str);

    let mut indicator = String::new();
    for (i, ch) in line_str.chars().enumerate() {
        if i >= col_idx { break; }
        indicator.push(if ch == '\t' { '\t' } else { ' ' });
    }
    indicator.push('^');
    println!("  {}", indicator);

    process::exit(1);
}

// =========================================================
// 1. 詞彙標記與中間碼
// =========================================================
#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
enum TokenType {
    TK_FUNC, TK_RETURN, TK_IF, TK_PRINT,
    TK_WHILE, TK_FOR, TK_BREAK, TK_CONTINUE,
    TK_ID, TK_NUM, TK_STRING,
    TK_LPAREN, TK_RPAREN, TK_LBRACE, TK_RBRACE, TK_LBRACKET, TK_RBRACKET,
    TK_DOT, TK_COLON, TK_COMMA, TK_SEMICOLON,
    TK_ASSIGN, TK_PLUS, TK_MINUS, TK_MUL, TK_DIV,
    TK_EQ, TK_LT, TK_GT,
    TK_EOF,
}

#[derive(Debug, Clone)]
struct Token {
    t_type: TokenType,
    text: String,
    pos: usize,
}

#[derive(Debug, Clone)]
struct Quad {
    op: String,
    arg1: String,
    arg2: String,
    result: String,
}

// =========================================================
// 2. 詞法分析 (Lexer)
// =========================================================
struct Lexer {
    src_chars: Vec<char>,
    src_str: String,
    pos: usize,
    cur_token: Option<Token>,
}

impl Lexer {
    fn new(src: String) -> Self {
        let src_chars: Vec<char> = src.chars().collect();
        let mut lexer = Lexer { src_chars, src_str: src, pos: 0, cur_token: None };
        lexer.next_token();
        lexer
    }

    fn next_token(&mut self) {
        loop {
            while self.pos < self.src_chars.len() && self.src_chars[self.pos].is_whitespace() {
                self.pos += 1;
            }
            if self.pos >= self.src_chars.len() {
                self.cur_token = Some(Token { t_type: TokenType::TK_EOF, text: "".to_string(), pos: self.pos });
                return;
            }

            if self.src_chars[self.pos] == '/' {
                if self.pos + 1 < self.src_chars.len() && self.src_chars[self.pos + 1] == '/' {
                    self.pos += 2;
                    while self.pos < self.src_chars.len() && self.src_chars[self.pos] != '\n' { self.pos += 1; }
                    continue;
                } else if self.pos + 1 < self.src_chars.len() && self.src_chars[self.pos + 1] == '*' {
                    self.pos += 2;
                    while self.pos + 1 < self.src_chars.len() && !(self.src_chars[self.pos] == '*' && self.src_chars[self.pos + 1] == '/') {
                        self.pos += 1;
                    }
                    if self.pos + 1 < self.src_chars.len() { self.pos += 2; }
                    continue;
                }
            }
            break;
        }

        let start = self.pos;
        let ch = self.src_chars[self.pos];

        // String
        if ch == '"' {
            self.pos += 1;
            let start_str = self.pos;
            while self.pos < self.src_chars.len() && self.src_chars[self.pos] != '"' { self.pos += 1; }
            if self.pos >= self.src_chars.len() {
                report_error(&self.src_str, start, "字串缺少結尾的雙引號 '\"'");
            }
            let text: String = self.src_chars[start_str..self.pos].iter().collect();
            self.pos += 1;
            self.cur_token = Some(Token { t_type: TokenType::TK_STRING, text, pos: start });
            return;
        }

        // Number
        if ch.is_ascii_digit() {
            while self.pos < self.src_chars.len() && self.src_chars[self.pos].is_ascii_digit() { self.pos += 1; }
            let text: String = self.src_chars[start..self.pos].iter().collect();
            self.cur_token = Some(Token { t_type: TokenType::TK_NUM, text, pos: start });
            return;
        }

        // Identifier / Keyword
        if ch.is_alphabetic() || ch == '_' {
            while self.pos < self.src_chars.len() && (self.src_chars[self.pos].is_alphanumeric() || self.src_chars[self.pos] == '_') { self.pos += 1; }
            let text: String = self.src_chars[start..self.pos].iter().collect();
            let mut keywords = HashMap::new();
            keywords.insert("func", TokenType::TK_FUNC); keywords.insert("return", TokenType::TK_RETURN);
            keywords.insert("if", TokenType::TK_IF); keywords.insert("print", TokenType::TK_PRINT);
            keywords.insert("while", TokenType::TK_WHILE); keywords.insert("for", TokenType::TK_FOR);
            keywords.insert("break", TokenType::TK_BREAK); keywords.insert("continue", TokenType::TK_CONTINUE);

            let t_type = keywords.get(text.as_str()).cloned().unwrap_or(TokenType::TK_ID);
            self.cur_token = Some(Token { t_type, text, pos: start });
            return;
        }

        // Symbols
        let mut text = ch.to_string();
        self.pos += 1;
        let t_type = match ch {
            '(' => TokenType::TK_LPAREN, ')' => TokenType::TK_RPAREN,
            '{' => TokenType::TK_LBRACE, '}' => TokenType::TK_RBRACE,
            '[' => TokenType::TK_LBRACKET, ']' => TokenType::TK_RBRACKET,
            '.' => TokenType::TK_DOT, ':' => TokenType::TK_COLON,
            '+' => TokenType::TK_PLUS, '-' => TokenType::TK_MINUS,
            '*' => TokenType::TK_MUL, '/' => TokenType::TK_DIV,
            ',' => TokenType::TK_COMMA, ';' => TokenType::TK_SEMICOLON,
            '<' => TokenType::TK_LT, '>' => TokenType::TK_GT,
            '=' => {
                if self.pos < self.src_chars.len() && self.src_chars[self.pos] == '=' {
                    self.pos += 1;
                    text = "==".to_string();
                    TokenType::TK_EQ
                } else {
                    TokenType::TK_ASSIGN
                }
            }
            _ => { report_error(&self.src_str, start, &format!("無法辨識的字元: '{}'", ch)); }
        };
        self.cur_token = Some(Token { t_type, text, pos: start });
    }
}

// =========================================================
// 3. 語法解析 (Parser)
// =========================================================
struct LoopCtx {
    breaks: Vec<usize>,
    continue_target: usize,
}

struct Parser {
    lexer: Lexer,
    quads: Vec<Quad>,
    string_pool: Vec<String>,
    loop_stack: Vec<LoopCtx>,
    t_idx: usize,
}

impl Parser {
    fn new(lexer: Lexer) -> Self {
        Parser { lexer, quads: Vec::new(), string_pool: Vec::new(), loop_stack: Vec::new(), t_idx: 0 }
    }

    fn cur(&self) -> &Token {
        self.lexer.cur_token.as_ref().unwrap()
    }

    fn consume(&mut self) {
        self.lexer.next_token();
    }

    fn error(&self, msg: &str) -> ! {
        let cur = self.cur();
        report_error(&self.lexer.src_str, cur.pos, &format!("{} (目前讀到: '{}')", msg, cur.text));
    }

    fn expect(&mut self, expected_type: TokenType, err_msg: &str) {
        if self.cur().t_type == expected_type {
            self.consume();
        } else {
            self.error(err_msg);
        }
    }

    fn new_t(&mut self) -> String {
        self.t_idx += 1;
        format!("t{}", self.t_idx)
    }

    fn emit(&mut self, op: &str, a1: &str, a2: &str, res: &str) -> usize {
        let idx = self.quads.len();
        self.quads.push(Quad { op: op.to_string(), arg1: a1.to_string(), arg2: a2.to_string(), result: res.to_string() });
        println!("{:03}: {:<12} {:<10} {:<10} {:<10}", idx, op, a1, a2, res);
        idx
    }

    // ================= 處理賦值與鏈式呼叫 =================
    fn expr_or_assign(&mut self) {
        let name = self.cur().text.clone();
        self.consume();
        let mut obj = name.clone();
        let mut path: Vec<String> = Vec::new();

        while vec![TokenType::TK_LBRACKET, TokenType::TK_DOT, TokenType::TK_LPAREN].contains(&self.cur().t_type) {
            match self.cur().t_type {
                TokenType::TK_LBRACKET => {
                    self.consume();
                    let idx = self.expression();
                    self.expect(TokenType::TK_RBRACKET, "預期 ']'");
                    path.push(idx);
                }
                TokenType::TK_DOT => {
                    self.consume();
                    if self.cur().t_type != TokenType::TK_ID { self.error("預期屬性名稱"); }
                    let key_str = self.cur().text.clone();
                    self.consume();
                    let k = self.new_t();
                    let pool_idx = self.string_pool.len();
                    self.string_pool.push(key_str);
                    self.emit("LOAD_STR", &pool_idx.to_string(), "-", &k);
                    path.push(k);
                }
                TokenType::TK_LPAREN => {
                    for p in &path {
                        let t = self.new_t();
                        self.emit("GET_ITEM", &obj, p, &t);
                        obj = t;
                    }
                    path.clear();
                    self.consume();
                    let mut count = 0;
                    if self.cur().t_type != TokenType::TK_RPAREN {
                        loop {
                            let arg = self.expression();
                            self.emit("PARAM", &arg, "-", "-");
                            count += 1;
                            if self.cur().t_type == TokenType::TK_COMMA {
                                self.consume();
                            } else { break; }
                        }
                    }
                    self.expect(TokenType::TK_RPAREN, "預期 ')'");
                    let t = self.new_t();
                    self.emit("CALL", &obj, &count.to_string(), &t);
                    obj = t;
                }
                _ => break,
            }
        }

        if self.cur().t_type == TokenType::TK_ASSIGN {
            self.consume();
            let val = self.expression();
            if path.is_empty() {
                self.emit("STORE", &val, "-", &obj);
            } else {
                for p in &path[0..path.len() - 1] {
                    let t = self.new_t();
                    self.emit("GET_ITEM", &obj, p, &t);
                    obj = t;
                }
                self.emit("SET_ITEM", &obj, path.last().unwrap(), &val);
            }
        }
    }

    // ================= 基本表達式元 =================
    fn primary(&mut self) -> String {
        match self.cur().t_type {
            TokenType::TK_NUM => {
                let t = self.new_t();
                self.emit("IMM", &self.cur().text.clone(), "-", &t);
                self.consume();
                t
            }
            TokenType::TK_STRING => {
                let t = self.new_t();
                let pool_idx = self.string_pool.len();
                self.string_pool.push(self.cur().text.clone());
                self.emit("LOAD_STR", &pool_idx.to_string(), "-", &t);
                self.consume();
                t
            }
            TokenType::TK_ID => {
                let name = self.cur().text.clone();
                self.consume();
                name
            }
            TokenType::TK_LBRACKET => {
                self.consume();
                let t = self.new_t();
                self.emit("NEW_ARR", "-", "-", &t);
                if self.cur().t_type != TokenType::TK_RBRACKET {
                    loop {
                        let val = self.expression();
                        self.emit("APPEND_ITEM", &t, "-", &val);
                        if self.cur().t_type == TokenType::TK_COMMA {
                            self.consume();
                        } else { break; }
                    }
                }
                self.expect(TokenType::TK_RBRACKET, "陣列預期要有 ']' 結尾");
                t
            }
            TokenType::TK_LBRACE => {
                self.consume();
                let t = self.new_t();
                self.emit("NEW_DICT", "-", "-", &t);
                if self.cur().t_type != TokenType::TK_RBRACE {
                    loop {
                        let k = if self.cur().t_type == TokenType::TK_ID {
                            let key_str = self.cur().text.clone();
                            self.consume();
                            let k = self.new_t();
                            let pool_idx = self.string_pool.len();
                            self.string_pool.push(key_str);
                            self.emit("LOAD_STR", &pool_idx.to_string(), "-", &k);
                            k
                        } else if self.cur().t_type == TokenType::TK_STRING {
                            self.primary()
                        } else {
                            self.error("字典的鍵必須是字串或識別碼");
                        };
                        self.expect(TokenType::TK_COLON, "字典預期要有 ':'");
                        let val = self.expression();
                        self.emit("SET_ITEM", &t, &k, &val);
                        if self.cur().t_type == TokenType::TK_COMMA {
                            self.consume();
                        } else { break; }
                    }
                }
                self.expect(TokenType::TK_RBRACE, "字典預期要有 '}' 結尾");
                t
            }
            TokenType::TK_LPAREN => {
                self.consume();
                let res = self.expression();
                self.expect(TokenType::TK_RPAREN, "預期要有 ')'");
                res
            }
            _ => self.error("表達式中出現預期外的語法結構"),
        }
    }

    fn factor(&mut self) -> String {
        let mut res = self.primary();
        while vec![TokenType::TK_LBRACKET, TokenType::TK_DOT, TokenType::TK_LPAREN].contains(&self.cur().t_type) {
            match self.cur().t_type {
                TokenType::TK_LBRACKET => {
                    self.consume();
                    let idx = self.expression();
                    self.expect(TokenType::TK_RBRACKET, "預期 ']'");
                    let t = self.new_t();
                    self.emit("GET_ITEM", &res, &idx, &t);
                    res = t;
                }
                TokenType::TK_DOT => {
                    self.consume();
                    let key_str = self.cur().text.clone();
                    self.consume();
                    let k = self.new_t();
                    let pool_idx = self.string_pool.len();
                    self.string_pool.push(key_str);
                    self.emit("LOAD_STR", &pool_idx.to_string(), "-", &k);
                    let t = self.new_t();
                    self.emit("GET_ITEM", &res, &k, &t);
                    res = t;
                }
                TokenType::TK_LPAREN => {
                    self.consume();
                    let mut count = 0;
                    if self.cur().t_type != TokenType::TK_RPAREN {
                        loop {
                            let arg = self.expression();
                            self.emit("PARAM", &arg, "-", "-");
                            count += 1;
                            if self.cur().t_type == TokenType::TK_COMMA {
                                self.consume();
                            } else { break; }
                        }
                    }
                    self.expect(TokenType::TK_RPAREN, "預期 ')'");
                    let t = self.new_t();
                    self.emit("CALL", &res, &count.to_string(), &t);
                    res = t;
                }
                _ => break,
            }
        }
        res
    }

    fn term(&mut self) -> String {
        let mut l = self.factor();
        while vec![TokenType::TK_MUL, TokenType::TK_DIV].contains(&self.cur().t_type) {
            let op = if self.cur().t_type == TokenType::TK_MUL { "MUL" } else { "DIV" };
            self.consume();
            let r = self.factor();
            let t = self.new_t();
            self.emit(op, &l, &r, &t);
            l = t;
        }
        l
    }

    fn arith_expr(&mut self) -> String {
        let mut l = self.term();
        while vec![TokenType::TK_PLUS, TokenType::TK_MINUS].contains(&self.cur().t_type) {
            let op = if self.cur().t_type == TokenType::TK_PLUS { "ADD" } else { "SUB" };
            self.consume();
            let r = self.term();
            let t = self.new_t();
            self.emit(op, &l, &r, &t);
            l = t;
        }
        l
    }

    fn expression(&mut self) -> String {
        let l = self.arith_expr();
        if vec![TokenType::TK_EQ, TokenType::TK_LT, TokenType::TK_GT].contains(&self.cur().t_type) {
            let op = match self.cur().t_type {
                TokenType::TK_EQ => "CMP_EQ",
                TokenType::TK_LT => "CMP_LT",
                _ => "CMP_GT",
            };
            self.consume();
            let r = self.arith_expr();
            let t = self.new_t();
            self.emit(op, &l, &r, &t);
            return t;
        }
        l
    }

    // ================= 陳述句 =================
    fn statement(&mut self) {
        match self.cur().t_type {
            TokenType::TK_IF => {
                self.consume(); self.expect(TokenType::TK_LPAREN, "預期 '('");
                let cond = self.expression();
                self.expect(TokenType::TK_RPAREN, "預期 ')'"); self.expect(TokenType::TK_LBRACE, "預期 '{'");
                
                let jmp_f_idx = self.emit("JMP_F", &cond, "-", "?");
                while self.cur().t_type != TokenType::TK_RBRACE && self.cur().t_type != TokenType::TK_EOF {
                    self.statement();
                }
                self.expect(TokenType::TK_RBRACE, "預期 '}'");
                let end_idx = self.quads.len().to_string();
                self.quads[jmp_f_idx].result = end_idx;
            }
            TokenType::TK_WHILE => {
                self.consume(); self.expect(TokenType::TK_LPAREN, "預期 '('");
                let cond_idx = self.quads.len();
                let cond = self.expression();
                self.expect(TokenType::TK_RPAREN, "預期 ')'"); self.expect(TokenType::TK_LBRACE, "預期 '{'");
                
                let jmp_f_idx = self.emit("JMP_F", &cond, "-", "?");
                self.loop_stack.push(LoopCtx { breaks: Vec::new(), continue_target: cond_idx });
                
                while self.cur().t_type != TokenType::TK_RBRACE && self.cur().t_type != TokenType::TK_EOF {
                    self.statement();
                }
                self.emit("JMP", "-", "-", &cond_idx.to_string());
                self.expect(TokenType::TK_RBRACE, "預期 '}'");
                
                let end_idx = self.quads.len().to_string();
                self.quads[jmp_f_idx].result = end_idx.clone();
                let loop_ctx = self.loop_stack.pop().unwrap();
                for b_idx in loop_ctx.breaks {
                    self.quads[b_idx].result = end_idx.clone();
                }
            }
            TokenType::TK_FOR => {
                self.consume(); self.expect(TokenType::TK_LPAREN, "預期 '('");
                
                if self.cur().t_type != TokenType::TK_SEMICOLON { self.expr_or_assign(); }
                self.expect(TokenType::TK_SEMICOLON, "預期 ';'");
                
                let cond_idx = self.quads.len();
                let cond = if self.cur().t_type != TokenType::TK_SEMICOLON {
                    self.expression()
                } else {
                    let c = self.new_t();
                    self.emit("IMM", "1", "-", &c);
                    c
                };
                
                let jmp_f_idx = self.emit("JMP_F", &cond, "-", "?");
                let jmp_body_idx = self.emit("JMP", "-", "-", "?");
                
                self.expect(TokenType::TK_SEMICOLON, "預期 ';'");
                
                let step_idx = self.quads.len();
                if self.cur().t_type != TokenType::TK_RPAREN { self.expr_or_assign(); }
                self.emit("JMP", "-", "-", &cond_idx.to_string());
                
                self.expect(TokenType::TK_RPAREN, "預期 ')'"); self.expect(TokenType::TK_LBRACE, "預期 '{'");
                
                let len = self.quads.len().to_string();
                self.quads[jmp_body_idx].result = len;
                self.loop_stack.push(LoopCtx { breaks: Vec::new(), continue_target: step_idx });
                
                while self.cur().t_type != TokenType::TK_RBRACE && self.cur().t_type != TokenType::TK_EOF {
                    self.statement();
                }
                self.emit("JMP", "-", "-", &step_idx.to_string());
                self.expect(TokenType::TK_RBRACE, "預期 '}'");
                
                let end_idx = self.quads.len().to_string();
                self.quads[jmp_f_idx].result = end_idx.clone();
                let loop_ctx = self.loop_stack.pop().unwrap();
                for b_idx in loop_ctx.breaks {
                    self.quads[b_idx].result = end_idx.clone();
                }
            }
            TokenType::TK_BREAK => {
                self.consume();
                if self.loop_stack.is_empty() { self.error("break 必須在迴圈內部使用"); }
                let b_idx = self.emit("JMP", "-", "-", "?");
                self.loop_stack.last_mut().unwrap().breaks.push(b_idx);
                self.expect(TokenType::TK_SEMICOLON, "預期 ';'");
            }
            TokenType::TK_CONTINUE => {
                self.consume();
                if self.loop_stack.is_empty() { self.error("continue 必須在迴圈內部使用"); }
                let c_target = self.loop_stack.last().unwrap().continue_target.to_string();
                self.emit("JMP", "-", "-", &c_target);
                self.expect(TokenType::TK_SEMICOLON, "預期 ';'");
            }
            TokenType::TK_ID => {
                self.expr_or_assign(); self.expect(TokenType::TK_SEMICOLON, "預期 ';'");
            }
            TokenType::TK_RETURN => {
                self.consume(); let res = self.expression(); self.emit("RET_VAL", &res, "-", "-");
                self.expect(TokenType::TK_SEMICOLON, "預期 ';'");
            }
            TokenType::TK_PRINT => {
                self.consume(); self.expect(TokenType::TK_LPAREN, "預期 '('");
                if self.cur().t_type != TokenType::TK_RPAREN {
                    loop {
                        let val = self.expression(); self.emit("PRINT_VAL", &val, "-", "-");
                        if self.cur().t_type == TokenType::TK_COMMA { self.consume(); }
                        else { break; }
                    }
                }
                self.emit("PRINT_NL", "-", "-", "-");
                self.expect(TokenType::TK_RPAREN, "預期 ')'"); self.expect(TokenType::TK_SEMICOLON, "預期 ';'");
            }
            _ => self.error("無法辨識的陳述句或語法結構"),
        }
    }

    fn parse_program(&mut self) {
        while self.cur().t_type != TokenType::TK_EOF {
            if self.cur().t_type == TokenType::TK_FUNC {
                self.consume();
                let f_name = self.cur().text.clone(); self.consume();
                self.emit("FUNC_BEG", &f_name, "-", "-");
                self.expect(TokenType::TK_LPAREN, "預期 '('");
                if self.cur().t_type != TokenType::TK_RPAREN {
                    loop {
                        self.emit("FORMAL", &self.cur().text.clone(), "-", "-"); self.consume();
                        if self.cur().t_type == TokenType::TK_COMMA { self.consume(); }
                        else { break; }
                    }
                }
                self.expect(TokenType::TK_RPAREN, "預期 ')'"); self.expect(TokenType::TK_LBRACE, "預期 '{'");
                while self.cur().t_type != TokenType::TK_RBRACE && self.cur().t_type != TokenType::TK_EOF {
                    self.statement();
                }
                self.emit("FUNC_END", &f_name, "-", "-");
                self.expect(TokenType::TK_RBRACE, "預期 '}'");
            } else {
                self.statement();
            }
        }
    }
}

// =========================================================
// 4. 虛擬機 (Virtual Machine)
// =========================================================
#[derive(Clone)]
enum Value {
    Int(i64),
    Str(String),
    Array(Rc<RefCell<Vec<Value>>>),
    Dict(Rc<RefCell<HashMap<String, Value>>>),
    Nil,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(i) => write!(f, "{}", i),
            Value::Str(s) => write!(f, "{}", s),
            Value::Array(a) => {
                let vec = a.borrow();
                let strs: Vec<String> = vec.iter().map(|v| v.to_string()).collect();
                write!(f, "[{}]", strs.join(", "))
            }
            Value::Dict(d) => {
                let map = d.borrow();
                let mut strs: Vec<String> = map.iter()
                    .map(|(k, v)| format!("'{}': {}", k, v))
                    .collect();
                strs.sort(); // 保持輸出穩定
                write!(f, "{{{}}}", strs.join(", "))
            }
            Value::Nil => write!(f, "0"),
        }
    }
}

impl Value {
    fn as_i64(&self) -> i64 {
        match self {
            Value::Int(i) => *i,
            _ => 0,
        }
    }
    
    fn as_str(&self) -> String {
        match self {
            Value::Str(s) => s.clone(),
            _ => "".to_string(),
        }
    }
}

struct Frame {
    vars: HashMap<String, Value>,
    ret_pc: usize,
    ret_var: String,
    incoming_args: Vec<Value>,
    formal_idx: usize,
}

struct VM {
    quads: Vec<Quad>,
    string_pool: Vec<String>,
    stack: Vec<Frame>,
    print_buf: Vec<String>,
}

impl VM {
    fn new(quads: Vec<Quad>, string_pool: Vec<String>) -> Self {
        VM {
            quads, string_pool, print_buf: Vec::new(),
            stack: vec![Frame { vars: HashMap::new(), ret_pc: 0, ret_var: "".to_string(), incoming_args: Vec::new(), formal_idx: 0 }],
        }
    }

    fn get_var(&self, name: &str) -> Value {
        if let Ok(n) = name.parse::<i64>() { return Value::Int(n); }
        if name.starts_with('-') && name[1..].parse::<i64>().is_ok() {
            return Value::Int(name.parse::<i64>().unwrap());
        }
        if name == "-" { return Value::Int(0); }
        self.stack.last().unwrap().vars.get(name).cloned().unwrap_or(Value::Nil)
    }

    fn set_var(&mut self, name: &str, val: Value) {
        self.stack.last_mut().unwrap().vars.insert(name.to_string(), val);
    }

    fn run(&mut self) {
        let mut pc = 0;
        let mut param_stack: Vec<Value> = Vec::new();
        let mut func_map = HashMap::new();

        for (i, q) in self.quads.iter().enumerate() {
            if q.op == "FUNC_BEG" {
                func_map.insert(q.arg1.clone(), i + 1);
            }
        }

        println!("\n=== VM 執行開始 ===");
        while pc < self.quads.len() {
            let q = &self.quads[pc].clone();
            let op = q.op.as_str();

            match op {
                "FUNC_BEG" => {
                    while self.quads[pc].op != "FUNC_END" { pc += 1; }
                }
                "IMM" => { self.set_var(&q.result, Value::Int(q.arg1.parse().unwrap())); }
                "LOAD_STR" => {
                    let idx: usize = q.arg1.parse().unwrap();
                    self.set_var(&q.result, Value::Str(self.string_pool[idx].clone()));
                }
                "ADD" => { self.set_var(&q.result, Value::Int(self.get_var(&q.arg1).as_i64() + self.get_var(&q.arg2).as_i64())); }
                "SUB" => { self.set_var(&q.result, Value::Int(self.get_var(&q.arg1).as_i64() - self.get_var(&q.arg2).as_i64())); }
                "MUL" => { self.set_var(&q.result, Value::Int(self.get_var(&q.arg1).as_i64() * self.get_var(&q.arg2).as_i64())); }
                "DIV" => {
                    let d = self.get_var(&q.arg2).as_i64();
                    self.set_var(&q.result, Value::Int(self.get_var(&q.arg1).as_i64() / if d == 0 { 1 } else { d }));
                }
                "CMP_EQ" => { self.set_var(&q.result, Value::Int(if self.get_var(&q.arg1).as_i64() == self.get_var(&q.arg2).as_i64() { 1 } else { 0 })); }
                "CMP_LT" => { self.set_var(&q.result, Value::Int(if self.get_var(&q.arg1).as_i64() < self.get_var(&q.arg2).as_i64() { 1 } else { 0 })); }
                "CMP_GT" => { self.set_var(&q.result, Value::Int(if self.get_var(&q.arg1).as_i64() > self.get_var(&q.arg2).as_i64() { 1 } else { 0 })); }
                "STORE" => { self.set_var(&q.result, self.get_var(&q.arg1)); }
                
                "NEW_ARR" => { self.set_var(&q.result, Value::Array(Rc::new(RefCell::new(Vec::new())))); }
                "NEW_DICT" => { self.set_var(&q.result, Value::Dict(Rc::new(RefCell::new(HashMap::new())))); }
                "APPEND_ITEM" => {
                    if let Value::Array(arr) = self.get_var(&q.arg1) {
                        arr.borrow_mut().push(self.get_var(&q.result));
                    }
                }
                "SET_ITEM" => {
                    let obj = self.get_var(&q.arg1);
                    let val = self.get_var(&q.result);
                    match obj {
                        Value::Array(arr) => {
                            let idx = self.get_var(&q.arg2).as_i64() as usize;
                            arr.borrow_mut()[idx] = val;
                        }
                        Value::Dict(dict) => {
                            let key = self.get_var(&q.arg2).as_str();
                            dict.borrow_mut().insert(key, val);
                        }
                        _ => {}
                    }
                }
                "GET_ITEM" => {
                    let obj = self.get_var(&q.arg1);
                    let val = match obj {
                        Value::Array(arr) => {
                            let idx = self.get_var(&q.arg2).as_i64() as usize;
                            arr.borrow()[idx].clone()
                        }
                        Value::Dict(dict) => {
                            let key = self.get_var(&q.arg2).as_str();
                            dict.borrow().get(&key).cloned().unwrap_or(Value::Nil)
                        }
                        _ => Value::Nil,
                    };
                    self.set_var(&q.result, val);
                }
                
                "JMP" => { pc = q.result.parse::<usize>().unwrap() - 1; }
                "JMP_F" => {
                    if self.get_var(&q.arg1).as_i64() == 0 {
                        pc = q.result.parse::<usize>().unwrap() - 1;
                    }
                }
                "PRINT_VAL" => { self.print_buf.push(self.get_var(&q.arg1).to_string()); }
                "PRINT_NL" => {
                    println!("[程式輸出] >> {}", self.print_buf.join(" "));
                    self.print_buf.clear();
                }
                "PARAM" => { param_stack.push(self.get_var(&q.arg1)); }
                "CALL" => {
                    let p_count: usize = q.arg2.parse().unwrap();
                    let f_name = if let Value::Str(s) = self.get_var(&q.arg1) { s } else { q.arg1.clone() };
                    
                    if let Some(&target_pc) = func_map.get(&f_name) {
                        let mut new_frame = Frame {
                            vars: HashMap::new(),
                            ret_pc: pc + 1,
                            ret_var: q.result.clone(),
                            incoming_args: Vec::new(),
                            formal_idx: 0,
                        };
                        if p_count > 0 {
                            let start = param_stack.len() - p_count;
                            new_frame.incoming_args = param_stack.drain(start..).collect();
                        }
                        self.stack.push(new_frame);
                        pc = target_pc;
                        continue;
                    } else {
                        panic!("找不到函數 '{}'", f_name);
                    }
                }
                "FORMAL" => {
                    let frame = self.stack.last_mut().unwrap();
                    let arg_val = frame.incoming_args[frame.formal_idx].clone();
                    frame.vars.insert(q.arg1.clone(), arg_val);
                    frame.formal_idx += 1;
                }
                "RET_VAL" => {
                    let ret_val = self.get_var(&q.arg1);
                    let frame = self.stack.pop().unwrap();
                    let ret_address = frame.ret_pc;
                    let target_var = frame.ret_var;
                    self.set_var(&target_var, ret_val);
                    pc = ret_address;
                    continue;
                }
                "FUNC_END" => {
                    if self.stack.len() > 1 {
                        let frame = self.stack.pop().unwrap();
                        let ret_address = frame.ret_pc;
                        let target_var = frame.ret_var;
                        self.set_var(&target_var, Value::Nil);
                        pc = ret_address;
                        continue;
                    }
                }
                _ => panic!("未知的指令: {}", op),
            }
            pc += 1;
        }

        println!("=== VM 執行完畢 ===\n\n記憶體狀態 (全域變數):");
        for (name, val) in &self.stack[0].vars {
            if !name.starts_with('t') {
                println!("[{}] = {}", name, val);
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("用法: {} <source_file>", args[0]);
        process::exit(1);
    }

    let source_code = match fs::read_to_string(&args[1]) {
        Ok(code) => code,
        Err(e) => {
            println!("無法開啟檔案: {}", e);
            process::exit(1);
        }
    };

    println!("編譯器生成的中間碼 (PC: Quadruples):");
    println!("{:-<44}", "");
    
    let lexer = Lexer::new(source_code);
    let mut parser = Parser::new(lexer);
    parser.parse_program();
    
    let mut vm = VM::new(parser.quads, parser.string_pool);
    vm.run();
}