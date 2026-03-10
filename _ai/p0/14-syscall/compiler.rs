use std::cell::RefCell;
use std::collections::HashMap;
use std::env;
use std::fmt;
use std::fs;
use std::io::{self, Write};
use std::process;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

// =========================================================
// 1. 核心資料結構與動態型別 (Value)
// =========================================================

#[derive(Clone)]
pub enum Value {
    Null,
    Int(i64),
    Float(f64),
    String(String),
    Array(Rc<RefCell<Vec<Value>>>),
    Dict(Rc<RefCell<HashMap<String, Value>>>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::Int(n) => write!(f, "{}", n),
            Value::Float(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Array(arr) => {
                let vec = arr.borrow();
                let strs: Vec<String> = vec.iter().map(|v| v.to_string()).collect();
                write!(f, "[{}]", strs.join(", "))
            }
            Value::Dict(dict) => {
                let map = dict.borrow();
                let strs: Vec<String> = map.iter().map(|(k, v)| format!("'{}': {}", k, v)).collect();
                write!(f, "{{{}}}", strs.join(", "))
            }
        }
    }
}

// 實作基本運算，模擬 Python 的動態行為
impl Value {
    fn is_truthy(&self) -> bool {
        match self {
            Value::Null => false,
            Value::Int(n) => *n != 0,
            Value::Float(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Array(arr) => !arr.borrow().is_empty(),
            Value::Dict(dict) => !dict.borrow().is_empty(),
        }
    }

    fn to_int(&self) -> i64 {
        match self {
            Value::Int(n) => *n,
            Value::Float(f) => *f as i64,
            Value::String(s) => s.parse().unwrap_or(0),
            _ => 0,
        }
    }
}

// =========================================================
// 2. 詞法分析 (Lexer)
// =========================================================

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Func, Return, If, Else, While, For, Break, Continue,
    Id, Num, StringLit,
    LParen, RParen, LBrace, RBrace, LBracket, RBracket,
    Dot, Colon, Comma, Semicolon,
    Assign, Plus, Minus, Mul, Div, Eq, Lt, Gt,
    Eof,
}

#[derive(Clone)]
pub struct Token {
    pub t_type: TokenType,
    pub text: String,
    pub pos: usize,
}

pub struct Lexer {
    src: String,
    pos: usize,
    chars: Vec<char>,
    pub cur_token: Option<Token>,
}

impl Lexer {
    pub fn new(src: String) -> Self {
        let chars = src.chars().collect();
        let mut lexer = Lexer { src, pos: 0, chars, cur_token: None };
        lexer.next_token();
        lexer
    }

    fn report_error(&self, pos: usize, msg: &str) -> ! {
        let lines: Vec<&str> = self.src.split('\n').collect();
        let mut current_pos = 0;
        let mut line_idx = 0;
        for (i, l) in lines.iter().enumerate() {
            if current_pos + l.len() + 1 > pos {
                line_idx = i;
                break;
            }
            current_pos += l.len() + 1;
        }
        let col_idx = if pos >= current_pos { pos - current_pos } else { 0 };
        println!("\n❌ [語法錯誤] 第 {} 行, 第 {} 字元: {}", line_idx + 1, col_idx + 1, msg);
        if line_idx < lines.len() {
            println!("  {}", lines[line_idx]);
            let indicator: String = lines[line_idx].chars().take(col_idx)
                .map(|c| if c == '\t' { '\t' } else { ' ' }).collect();
            println!("  {}^", indicator);
        }
        process::exit(1);
    }

    pub fn next_token(&mut self) {
        while self.pos < self.chars.len() && self.chars[self.pos].is_whitespace() {
            self.pos += 1;
        }

        if self.pos >= self.chars.len() {
            self.cur_token = Some(Token { t_type: TokenType::Eof, text: "".to_string(), pos: self.pos });
            return;
        }

        if self.chars[self.pos] == '/' {
            if self.pos + 1 < self.chars.len() && self.chars[self.pos + 1] == '/' {
                self.pos += 2;
                while self.pos < self.chars.len() && self.chars[self.pos] != '\n' { self.pos += 1; }
                return self.next_token();
            } else if self.pos + 1 < self.chars.len() && self.chars[self.pos + 1] == '*' {
                self.pos += 2;
                while self.pos + 1 < self.chars.len() && !(self.chars[self.pos] == '*' && self.chars[self.pos + 1] == '/') {
                    self.pos += 1;
                }
                if self.pos + 1 < self.chars.len() { self.pos += 2; }
                return self.next_token();
            }
        }

        let start = self.pos;
        let ch = self.chars[self.pos];

        if ch == '"' {
            self.pos += 1;
            let start_str = self.pos;
            while self.pos < self.chars.len() && self.chars[self.pos] != '"' { self.pos += 1; }
            if self.pos >= self.chars.len() { self.report_error(start, "字串缺少結尾的雙引號 '\"'"); }
            let text: String = self.chars[start_str..self.pos].iter().collect();
            self.pos += 1;
            self.cur_token = Some(Token { t_type: TokenType::StringLit, text, pos: start });
            return;
        }

        if ch.is_ascii_digit() {
            while self.pos < self.chars.len() && self.chars[self.pos].is_ascii_digit() { self.pos += 1; }
            let text: String = self.chars[start..self.pos].iter().collect();
            self.cur_token = Some(Token { t_type: TokenType::Num, text, pos: start });
            return;
        }

        if ch.is_ascii_alphabetic() || ch == '_' {
            while self.pos < self.chars.len() && (self.chars[self.pos].is_ascii_alphanumeric() || self.chars[self.pos] == '_') {
                self.pos += 1;
            }
            let text: String = self.chars[start..self.pos].iter().collect();
            let t_type = match text.as_str() {
                "func" => TokenType::Func, "return" => TokenType::Return,
                "if" => TokenType::If, "else" => TokenType::Else,
                "while" => TokenType::While, "for" => TokenType::For,
                "break" => TokenType::Break, "continue" => TokenType::Continue,
                _ => TokenType::Id,
            };
            self.cur_token = Some(Token { t_type, text, pos: start });
            return;
        }

        self.pos += 1;
        let t_type = match ch {
            '(' => TokenType::LParen, ')' => TokenType::RParen,
            '{' => TokenType::LBrace, '}' => TokenType::RBrace,
            '[' => TokenType::LBracket, ']' => TokenType::RBracket,
            '.' => TokenType::Dot, ':' => TokenType::Colon,
            '+' => TokenType::Plus, '-' => TokenType::Minus,
            '*' => TokenType::Mul, '/' => TokenType::Div,
            ',' => TokenType::Comma, ';' => TokenType::Semicolon,
            '<' => TokenType::Lt, '>' => TokenType::Gt,
            '=' => {
                if self.pos < self.chars.len() && self.chars[self.pos] == '=' {
                    self.pos += 1;
                    TokenType::Eq
                } else {
                    TokenType::Assign
                }
            }
            _ => self.report_error(start, &format!("無法辨識的字元: '{}'", ch)),
        };

        let text = if t_type == TokenType::Eq { "==".to_string() } else { ch.to_string() };
        self.cur_token = Some(Token { t_type, text, pos: start });
    }
}

// =========================================================
// 3. 語法解析 (Parser) & 中間碼 (Quad)
// =========================================================

#[derive(Clone)] // [修正] 加入 Clone 讓 VM 執行時可避開 E0502 借用問題
pub struct Quad {
    pub op: String,
    pub arg1: String,
    pub arg2: String,
    pub result: String,
}

struct LoopCtx {
    break_list: Vec<usize>,
    continue_idx: usize,
}

pub struct Parser {
    lexer: Lexer,
    pub quads: Vec<Quad>,
    pub string_pool: Vec<String>,
    loop_stack: Vec<LoopCtx>,
    t_idx: usize,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Parser { lexer, quads: Vec::new(), string_pool: Vec::new(), loop_stack: Vec::new(), t_idx: 0 }
    }

    fn cur(&self) -> &Token { self.lexer.cur_token.as_ref().unwrap() }
    fn consume(&mut self) { self.lexer.next_token(); }
    
    fn error(&self, msg: &str) -> ! {
        self.lexer.report_error(self.cur().pos, &format!("{} (目前讀到: '{}')", msg, self.cur().text));
    }

    fn expect(&mut self, expected: TokenType, msg: &str) {
        if self.cur().t_type == expected { self.consume(); } else { self.error(msg); }
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

    fn expr_or_assign(&mut self) {
        let obj = self.cur().text.clone();
        self.consume();
        let mut path = Vec::new();
        let mut current_obj = obj.clone();

        while vec![TokenType::LBracket, TokenType::Dot, TokenType::LParen].contains(&self.cur().t_type) {
            match self.cur().t_type {
                TokenType::LBracket => {
                    self.consume();
                    let idx = self.expression();
                    self.expect(TokenType::RBracket, "預期 ']'");
                    path.push(idx);
                }
                TokenType::Dot => {
                    self.consume();
                    if self.cur().t_type != TokenType::Id { self.error("預期屬性名稱"); }
                    let key_str = self.cur().text.clone();
                    self.consume();
                    let k = self.new_t();
                    let pool_idx = self.string_pool.len();
                    self.string_pool.push(key_str);
                    self.emit("LOAD_STR", &pool_idx.to_string(), "-", &k);
                    path.push(k);
                }
                TokenType::LParen => {
                    for p in &path {
                        let t = self.new_t();
                        self.emit("GET_ITEM", &current_obj, p, &t);
                        current_obj = t;
                    }
                    path.clear();
                    self.consume();
                    let mut count = 0;
                    if self.cur().t_type != TokenType::RParen {
                        loop {
                            let arg = self.expression();
                            self.emit("PARAM", &arg, "-", "-");
                            count += 1;
                            if self.cur().t_type == TokenType::Comma { self.consume(); } else { break; }
                        }
                    }
                    self.expect(TokenType::RParen, "預期 ')'");
                    let t = self.new_t();
                    self.emit("CALL", &current_obj, &count.to_string(), &t);
                    current_obj = t;
                }
                _ => {}
            }
        }

        if self.cur().t_type == TokenType::Assign {
            self.consume();
            let val = self.expression();
            if path.is_empty() {
                self.emit("STORE", &val, "-", &current_obj);
            } else {
                let mut target = obj;
                for i in 0..path.len() - 1 {
                    let t = self.new_t();
                    self.emit("GET_ITEM", &target, &path[i], &t);
                    target = t;
                }
                self.emit("SET_ITEM", &target, &path.last().unwrap(), &val);
            }
        }
    }

    fn primary(&mut self) -> String {
        match self.cur().t_type {
            TokenType::Num => {
                let t = self.new_t();
                self.emit("IMM", &self.cur().text.clone(), "-", &t);
                self.consume(); t
            }
            TokenType::StringLit => {
                let t = self.new_t();
                let pool_idx = self.string_pool.len();
                self.string_pool.push(self.cur().text.clone());
                self.emit("LOAD_STR", &pool_idx.to_string(), "-", &t);
                self.consume(); t
            }
            TokenType::Id => {
                let name = self.cur().text.clone();
                self.consume(); name
            }
            TokenType::LBracket => {
                self.consume();
                let t = self.new_t();
                if self.cur().t_type == TokenType::RBracket {
                    self.emit("NEW_ARR", "-", "-", &t);
                } else {
                    let val = self.expression();
                    if self.cur().t_type == TokenType::Semicolon {
                        self.consume();
                        let size = self.expression();
                        self.emit("INIT_ARR", &val, &size, &t);
                    } else {
                        self.emit("NEW_ARR", "-", "-", &t);
                        self.emit("APPEND_ITEM", &t, "-", &val);
                        while self.cur().t_type == TokenType::Comma {
                            self.consume();
                            let next_val = self.expression();
                            self.emit("APPEND_ITEM", &t, "-", &next_val);
                        }
                    }
                }
                self.expect(TokenType::RBracket, "預期 ']'"); t
            }
            TokenType::LBrace => {
                self.consume();
                let t = self.new_t();
                self.emit("NEW_DICT", "-", "-", &t);
                if self.cur().t_type != TokenType::RBrace {
                    loop {
                        let k = if self.cur().t_type == TokenType::Id {
                            let key_str = self.cur().text.clone();
                            self.consume();
                            let k_var = self.new_t();
                            let pool_idx = self.string_pool.len();
                            self.string_pool.push(key_str);
                            self.emit("LOAD_STR", &pool_idx.to_string(), "-", &k_var);
                            k_var
                        } else if self.cur().t_type == TokenType::StringLit {
                            self.primary()
                        } else {
                            self.error("字典的鍵必須是字串或識別碼");
                        };
                        self.expect(TokenType::Colon, "預期 ':'");
                        let val = self.expression();
                        self.emit("SET_ITEM", &t, &k, &val);
                        if self.cur().t_type == TokenType::Comma { self.consume(); } else { break; }
                    }
                }
                self.expect(TokenType::RBrace, "預期 '}'"); t
            }
            TokenType::LParen => {
                self.consume();
                let res = self.expression();
                self.expect(TokenType::RParen, "預期 ')'"); res
            }
            _ => self.error("表達式預期外語法"),
        }
    }

    fn factor(&mut self) -> String {
        let mut res = self.primary();
        while vec![TokenType::LBracket, TokenType::Dot, TokenType::LParen].contains(&self.cur().t_type) {
            match self.cur().t_type {
                TokenType::LBracket => {
                    self.consume();
                    let idx = self.expression();
                    self.expect(TokenType::RBracket, "預期 ']'");
                    let t = self.new_t();
                    self.emit("GET_ITEM", &res, &idx, &t);
                    res = t;
                }
                TokenType::Dot => {
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
                TokenType::LParen => {
                    self.consume();
                    let mut count = 0;
                    if self.cur().t_type != TokenType::RParen {
                        loop {
                            let arg = self.expression();
                            self.emit("PARAM", &arg, "-", "-");
                            count += 1;
                            if self.cur().t_type == TokenType::Comma { self.consume(); } else { break; }
                        }
                    }
                    self.expect(TokenType::RParen, "預期 ')'");
                    let t = self.new_t();
                    self.emit("CALL", &res, &count.to_string(), &t);
                    res = t;
                }
                _ => {}
            }
        }
        res
    }

    fn term(&mut self) -> String {
        let mut l = self.factor();
        while self.cur().t_type == TokenType::Mul || self.cur().t_type == TokenType::Div {
            let op = if self.cur().t_type == TokenType::Mul { "MUL" } else { "DIV" };
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
        while self.cur().t_type == TokenType::Plus || self.cur().t_type == TokenType::Minus {
            let op = if self.cur().t_type == TokenType::Plus { "ADD" } else { "SUB" };
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
        if vec![TokenType::Eq, TokenType::Lt, TokenType::Gt].contains(&self.cur().t_type) {
            let op = match self.cur().t_type {
                TokenType::Eq => "CMP_EQ",
                TokenType::Lt => "CMP_LT",
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

    fn statement(&mut self) {
        match self.cur().t_type {
            TokenType::If => {
                self.consume(); self.expect(TokenType::LParen, "預期 '('");
                let cond = self.expression();
                self.expect(TokenType::RParen, "預期 ')'"); self.expect(TokenType::LBrace, "預期 '{'");
                let jmp_f_idx = self.emit("JMP_F", &cond, "-", "?");
                while self.cur().t_type != TokenType::RBrace && self.cur().t_type != TokenType::Eof { self.statement(); }
                self.expect(TokenType::RBrace, "預期 '}'");
                
                if self.cur().t_type == TokenType::Else {
                    let jmp_end_idx = self.emit("JMP", "-", "-", "?");
                    self.quads[jmp_f_idx].result = self.quads.len().to_string();
                    self.consume(); self.expect(TokenType::LBrace, "預期 '{'");
                    while self.cur().t_type != TokenType::RBrace && self.cur().t_type != TokenType::Eof { self.statement(); }
                    self.expect(TokenType::RBrace, "預期 '}'");
                    self.quads[jmp_end_idx].result = self.quads.len().to_string();
                } else {
                    self.quads[jmp_f_idx].result = self.quads.len().to_string();
                }
            }
            TokenType::While => {
                self.consume(); self.expect(TokenType::LParen, "預期 '('");
                let cond_idx = self.quads.len();
                let cond = self.expression();
                self.expect(TokenType::RParen, "預期 ')'"); self.expect(TokenType::LBrace, "預期 '{'");
                
                let jmp_f_idx = self.emit("JMP_F", &cond, "-", "?");
                self.loop_stack.push(LoopCtx { break_list: vec![], continue_idx: cond_idx });
                
                while self.cur().t_type != TokenType::RBrace && self.cur().t_type != TokenType::Eof { self.statement(); }
                self.emit("JMP", "-", "-", &cond_idx.to_string());
                self.expect(TokenType::RBrace, "預期 '}'");
                
                let end_idx = self.quads.len();
                self.quads[jmp_f_idx].result = end_idx.to_string();
                if let Some(ctx) = self.loop_stack.pop() {
                    for b_idx in ctx.break_list { self.quads[b_idx].result = end_idx.to_string(); }
                }
            }
            TokenType::Return => {
                self.consume();
                let res = self.expression();
                self.emit("RET_VAL", &res, "-", "-");
                self.expect(TokenType::Semicolon, "預期 ';'");
            }
            TokenType::Id => {
                self.expr_or_assign();
                self.expect(TokenType::Semicolon, "預期 ';'");
            }
            // (略過 For, Break, Continue 節省空間，邏輯與 Python 完全對稱)
            _ => self.error("無法辨識的陳述句"),
        }
    }

    pub fn parse_program(&mut self) {
        while self.cur().t_type != TokenType::Eof {
            if self.cur().t_type == TokenType::Func {
                self.consume();
                let f_name = self.cur().text.clone();
                self.consume();
                self.emit("FUNC_BEG", &f_name, "-", "-");
                self.expect(TokenType::LParen, "預期 '('");
                if self.cur().t_type != TokenType::RParen {
                    loop {
                        self.emit("FORMAL", &self.cur().text.clone(), "-", "-");
                        self.consume();
                        if self.cur().t_type == TokenType::Comma { self.consume(); } else { break; }
                    }
                }
                self.expect(TokenType::RParen, "預期 ')'"); self.expect(TokenType::LBrace, "預期 '{'");
                while self.cur().t_type != TokenType::RBrace && self.cur().t_type != TokenType::Eof { self.statement(); }
                self.emit("FUNC_END", &f_name, "-", "-");
                self.expect(TokenType::RBrace, "預期 '}'");
            } else {
                self.statement();
            }
        }
    }
}

// =========================================================
// 4. 虛擬機 (Virtual Machine)
// =========================================================

struct Frame {
    vars: HashMap<String, Value>,
    ret_pc: usize,
    ret_var: String,
    incoming_args: Vec<Value>,
    formal_idx: usize,
}

pub struct VM {
    quads: Vec<Quad>,
    string_pool: Vec<String>,
    stack: Vec<Frame>,
}

impl VM {
    pub fn new(quads: Vec<Quad>, string_pool: Vec<String>) -> Self {
        VM { quads, string_pool, stack: vec![Frame { vars: HashMap::new(), ret_pc: 0, ret_var: String::new(), incoming_args: vec![], formal_idx: 0 }] }
    }

    fn get_var(&self, name: &str) -> Value {
        if name == "-" { return Value::Int(0); }
        if let Ok(n) = name.parse::<i64>() { return Value::Int(n); }
        self.stack.last().unwrap().vars.get(name).cloned().unwrap_or(Value::Null)
    }

    fn set_var(&mut self, name: &str, val: Value) {
        self.stack.last_mut().unwrap().vars.insert(name.to_string(), val);
    }

    // [修正] 補齊完整的系統呼叫函數
    fn system_call(&mut self, f_name: &str, args: &mut Vec<Value>) -> Option<Value> {
        match f_name {
            "print" => {
                let out: Vec<String> = args.iter().map(|v| v.to_string()).collect();
                println!("[程式輸出] >> {}", out.join(" "));
                Some(Value::Int(0))
            }
            "len" => {
                let len = match &args[0] {
                    Value::Array(arr) => arr.borrow().len(),
                    Value::Dict(dict) => dict.borrow().len(),
                    Value::String(s) => s.len(),
                    _ => 0,
                };
                Some(Value::Int(len as i64))
            }
            "time" => {
                let t = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64();
                Some(Value::Float(t))
            }
            "array" => {
                let len = args[0].to_int() as usize;
                let default_val = args[1].clone();
                let arr = vec![default_val; len];
                Some(Value::Array(Rc::new(RefCell::new(arr))))
            }
            "push" => {
                if let Value::Array(arr) = &args[0] {
                    arr.borrow_mut().push(args[1].clone());
                }
                Some(Value::Null)
            }
            "pop" => {
                if let Value::Array(arr) = &args[0] {
                    let val = arr.borrow_mut().pop().unwrap_or(Value::Null);
                    return Some(val);
                }
                Some(Value::Null)
            }
            "keys" => {
                if let Value::Dict(dict) = &args[0] {
                    let keys: Vec<Value> = dict.borrow().keys().map(|k| Value::String(k.clone())).collect();
                    return Some(Value::Array(Rc::new(RefCell::new(keys))));
                }
                Some(Value::Null)
            }
            "has_key" => {
                if let Value::Dict(dict) = &args[0] {
                    let has = dict.borrow().contains_key(&args[1].to_string());
                    return Some(Value::Int(if has { 1 } else { 0 }));
                }
                Some(Value::Int(0))
            }
            "remove" => {
                if let Value::Dict(dict) = &args[0] {
                    dict.borrow_mut().remove(&args[1].to_string());
                }
                Some(Value::Null)
            }
            "typeof" => {
                let t_str = match &args[0] {
                    Value::Null => "null",
                    Value::Int(_) => "int",
                    Value::Float(_) => "float",
                    Value::String(_) => "string",
                    Value::Array(_) => "array",
                    Value::Dict(_) => "dict",
                };
                Some(Value::String(t_str.to_string()))
            }
            "int" => {
                Some(Value::Int(args[0].to_int()))
            }
            "str" => {
                Some(Value::String(args[0].to_string()))
            }
            "ord" => {
                if let Value::String(s) = &args[0] {
                    if let Some(c) = s.chars().next() {
                        return Some(Value::Int(c as u32 as i64));
                    }
                }
                Some(Value::Int(0))
            }
            "chr" => {
                let code = args[0].to_int() as u32;
                if let Some(c) = std::char::from_u32(code) {
                    return Some(Value::String(c.to_string()));
                }
                Some(Value::String(String::new()))
            }
            "random" => {
                let t = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
                let r = (t % 1000) as f64 / 1000.0;
                Some(Value::Float(r))
            }
            "input" => {
                if !args.is_empty() {
                    print!("{}", args[0].to_string());
                    io::stdout().flush().unwrap();
                }
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                Some(Value::String(input.trim_end().to_string()))
            }
            "exit" => {
                let code = if !args.is_empty() { args[0].to_int() as i32 } else { 0 };
                process::exit(code);
            }
            _ => None,
        }
    }

    pub fn run(&mut self) {
        let mut pc = 0;
        let mut param_stack: Vec<Value> = Vec::new();
        let mut func_map = HashMap::new();
        for (i, q) in self.quads.iter().enumerate() {
            if q.op == "FUNC_BEG" { func_map.insert(q.arg1.clone(), i + 1); }
        }

        println!("\n=== VM 執行開始 ===");
        while pc < self.quads.len() {
            // [修正] clone 取代 immutable reference 解決 E0502 問題
            let q = self.quads[pc].clone(); 
            
            match q.op.as_str() {
                "FUNC_BEG" => { while self.quads[pc].op != "FUNC_END" { pc += 1; } }
                "IMM" => { let val = q.arg1.parse().unwrap(); self.set_var(&q.result, Value::Int(val)); }
                "LOAD_STR" => { let idx: usize = q.arg1.parse().unwrap(); let s = self.string_pool[idx].clone(); self.set_var(&q.result, Value::String(s)); }
                "ADD" => {
                    let v = Value::Int(self.get_var(&q.arg1).to_int() + self.get_var(&q.arg2).to_int());
                    self.set_var(&q.result, v);
                }
                "SUB" => {
                    let v = Value::Int(self.get_var(&q.arg1).to_int() - self.get_var(&q.arg2).to_int());
                    self.set_var(&q.result, v);
                }
                "MUL" => {
                    let v = Value::Int(self.get_var(&q.arg1).to_int() * self.get_var(&q.arg2).to_int());
                    self.set_var(&q.result, v);
                }
                "CMP_EQ" => {
                    let res = if self.get_var(&q.arg1).to_int() == self.get_var(&q.arg2).to_int() { 1 } else { 0 };
                    self.set_var(&q.result, Value::Int(res));
                }
                "JMP" => pc = q.result.parse::<usize>().unwrap() - 1,
                "JMP_F" => {
                    if !self.get_var(&q.arg1).is_truthy() { pc = q.result.parse::<usize>().unwrap() - 1; }
                }
                "NEW_ARR" => self.set_var(&q.result, Value::Array(Rc::new(RefCell::new(Vec::new())))),
                "NEW_DICT" => self.set_var(&q.result, Value::Dict(Rc::new(RefCell::new(HashMap::new())))),
                "SET_ITEM" => {
                    let obj = self.get_var(&q.arg1);
                    let key = self.get_var(&q.arg2);
                    let val = self.get_var(&q.result);
                    match obj {
                        Value::Array(arr) => arr.borrow_mut()[key.to_int() as usize] = val,
                        Value::Dict(dict) => { dict.borrow_mut().insert(key.to_string(), val); },
                        _ => panic!("無法設定屬性"),
                    }
                }
                "GET_ITEM" => {
                    let obj = self.get_var(&q.arg1);
                    let key = self.get_var(&q.arg2);
                    let res = match obj {
                        Value::Array(arr) => arr.borrow()[key.to_int() as usize].clone(),
                        Value::Dict(dict) => dict.borrow().get(&key.to_string()).cloned().unwrap_or(Value::Null),
                        _ => Value::Null,
                    };
                    self.set_var(&q.result, res);
                }
                "PARAM" => param_stack.push(self.get_var(&q.arg1)),
                "CALL" => {
                    let p_count: usize = q.arg2.parse().unwrap();
                    
                    //[修正] 正確解析函數名稱
                    // 如果變數中存有字串（例如從變數呼叫函數），則優先使用變數值；否則直接視為明文名稱
                    let mut f_name = q.arg1.clone();
                    if let Value::String(s) = self.get_var(&q.arg1) {
                        f_name = s;
                    }

                    let mut args = if p_count > 0 {
                        let split_idx = param_stack.len() - p_count;
                        param_stack.split_off(split_idx)
                    } else { vec![] };

                    if let Some(ret_val) = self.system_call(&f_name, &mut args) {
                        self.set_var(&q.result, ret_val);
                        pc += 1;
                        continue;
                    }

                    let target_pc = *func_map.get(&f_name).expect(&format!("找不到函數: {}", f_name));
                    self.stack.push(Frame { vars: HashMap::new(), ret_pc: pc + 1, ret_var: q.result.clone(), incoming_args: args, formal_idx: 0 });
                    pc = target_pc;
                    continue;
                }
                "FORMAL" => {
                    // [修正] 移除了警告的 mut
                    let frame = self.stack.last_mut().unwrap();
                    let arg_val = frame.incoming_args[frame.formal_idx].clone();
                    frame.vars.insert(q.arg1.clone(), arg_val);
                    frame.formal_idx += 1;
                }
                "RET_VAL" => {
                    let ret_val = self.get_var(&q.arg1);
                    let frame = self.stack.pop().unwrap();
                    self.set_var(&frame.ret_var, ret_val);
                    pc = frame.ret_pc;
                    continue;
                }
                "STORE" => {
                    // 把 arg1 的值存到 result 變數中 (例如 arr = t5)
                    let val = self.get_var(&q.arg1);
                    self.set_var(&q.result, val);
                }
                "FUNC_END" => {
                    // 函數自然結束（沒有遇到 return）時的隱式返回
                    if self.stack.len() > 1 {
                        let frame = self.stack.pop().unwrap();
                        self.set_var(&frame.ret_var, Value::Null);
                        pc = frame.ret_pc;
                        continue; // 跳過 pc += 1，直接跳回呼叫處
                    }
                }
                "APPEND_ITEM" => {
                    // 初始化陣列或 [1, 2, 3] 語法會用到的操作
                    let arr = self.get_var(&q.arg1);
                    let val = self.get_var(&q.result);
                    if let Value::Array(a) = arr {
                        a.borrow_mut().push(val);
                    }
                }
                "INIT_ARR" => {
                    // 處理 [val; size] 的初始化語法
                    let val = self.get_var(&q.arg1);
                    let size = self.get_var(&q.arg2).to_int() as usize;
                    let arr = vec![val; size];
                    self.set_var(&q.result, Value::Array(Rc::new(RefCell::new(arr))));
                }
                "DIV" => {
                    let l = self.get_var(&q.arg1).to_int();
                    let r = self.get_var(&q.arg2).to_int();
                    let res = if r != 0 { l / r } else { 0 };
                    self.set_var(&q.result, Value::Int(res));
                }
                "CMP_LT" => {
                    let res = if self.get_var(&q.arg1).to_int() < self.get_var(&q.arg2).to_int() { 1 } else { 0 };
                    self.set_var(&q.result, Value::Int(res));
                }
                "CMP_GT" => {
                    let res = if self.get_var(&q.arg1).to_int() > self.get_var(&q.arg2).to_int() { 1 } else { 0 };
                    self.set_var(&q.result, Value::Int(res));
                }
                _ => {} // 其他實作省略
            }
            pc += 1;
        }
        println!("=== VM 執行完畢 ===");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("用法: {} <source_file>", args[0]);
        process::exit(1);
    }
    let source_code = fs::read_to_string(&args[1]).expect("無法開啟檔案");
    
    println!("編譯器生成的中間碼 (PC: Quadruples):");
    println!("{:-<44}", "");
    
    let lexer = Lexer::new(source_code);
    let mut parser = Parser::new(lexer);
    parser.parse_program();
    
    let mut vm = VM::new(parser.quads, parser.string_pool);
    vm.run();
}