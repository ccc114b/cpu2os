use std::collections::HashMap;
use std::env;
use std::fs;

// =========================================================
// 1. 中間碼 (Quadruples) 資料結構
// =========================================================
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
#[derive(Debug, Clone, PartialEq)]
enum TokenType {
    Func, Return, If, Id, Num,
    LParen, RParen, LBrace, RBrace, Comma, Semicolon,
    Assign, Plus, Minus, Mul, Div,
    Eq, Lt, Gt, Eof, // Dummy,
}

#[derive(Debug, Clone)]
struct Token {
    token_type: TokenType,
    text: String,
}

struct Lexer<'a> {
    src: &'a[u8],
    pos: usize,
}

impl<'a> Lexer<'a> {
    fn new(src: &'a str) -> Self {
        Self {
            src: src.as_bytes(),
            pos: 0,
        }
    }

    fn next_token(&mut self) -> Token {
        loop {
            // 忽略空格、換行
            while self.pos < self.src.len() && self.src[self.pos].is_ascii_whitespace() {
                self.pos += 1;
            }

            if self.pos >= self.src.len() {
                return Token { token_type: TokenType::Eof, text: String::new() };
            }

            // 處理註解
            if self.src[self.pos] == b'/' {
                if self.pos + 1 < self.src.len() && self.src[self.pos + 1] == b'/' {
                    self.pos += 2;
                    while self.pos < self.src.len() && self.src[self.pos] != b'\n' {
                        self.pos += 1;
                    }
                    continue;
                } else if self.pos + 1 < self.src.len() && self.src[self.pos + 1] == b'*' {
                    self.pos += 2;
                    while self.pos + 1 < self.src.len()
                        && !(self.src[self.pos] == b'*' && self.src[self.pos + 1] == b'/')
                    {
                        self.pos += 1;
                    }
                    if self.pos + 1 < self.src.len() {
                        self.pos += 2;
                    }
                    continue;
                }
            }
            break;
        }

        let start = self.pos;

        // 辨識數字 (NUM)
        if self.src[self.pos].is_ascii_digit() {
            while self.pos < self.src.len() && self.src[self.pos].is_ascii_digit() {
                self.pos += 1;
            }
            let text = String::from_utf8_lossy(&self.src[start..self.pos]).to_string();
            return Token { token_type: TokenType::Num, text };
        }
        // 辨識識別碼 (ID) 與 關鍵字 (Keyword)
        else if self.src[self.pos].is_ascii_alphabetic() || self.src[self.pos] == b'_' {
            while self.pos < self.src.len()
                && (self.src[self.pos].is_ascii_alphanumeric() || self.src[self.pos] == b'_')
            {
                self.pos += 1;
            }
            let text = String::from_utf8_lossy(&self.src[start..self.pos]).to_string();
            let token_type = match text.as_str() {
                "func" => TokenType::Func,
                "return" => TokenType::Return,
                "if" => TokenType::If,
                _ => TokenType::Id,
            };
            return Token { token_type, text };
        }
        // 辨識運算符與符號
        else {
            let ch = self.src[self.pos];
            self.pos += 1;
            let mut text = (ch as char).to_string();
            let token_type = match ch {
                b'(' => TokenType::LParen,
                b')' => TokenType::RParen,
                b'{' => TokenType::LBrace,
                b'}' => TokenType::RBrace,
                b'+' => TokenType::Plus,
                b'-' => TokenType::Minus,
                b'*' => TokenType::Mul,
                b'/' => TokenType::Div,
                b',' => TokenType::Comma,
                b';' => TokenType::Semicolon,
                b'<' => TokenType::Lt,
                b'>' => TokenType::Gt,
                b'=' => {
                    if self.pos < self.src.len() && self.src[self.pos] == b'=' {
                        self.pos += 1;
                        text = "==".to_string();
                        TokenType::Eq
                    } else {
                        TokenType::Assign
                    }
                }
                _ => panic!("未知的字元: {}", ch as char),
            };
            return Token { token_type, text };
        }
    }
}

// =========================================================
// 3. 語法解析 (Parser) - 遞迴下降法
// =========================================================
struct Parser<'a> {
    lexer: Lexer<'a>,
    cur_token: Token,
    quads: Vec<Quad>,
    t_idx: usize,
}

impl<'a> Parser<'a> {
    fn new(mut lexer: Lexer<'a>) -> Self {
        let cur_token = lexer.next_token();
        Self {
            lexer,
            cur_token,
            quads: Vec::new(),
            t_idx: 0,
        }
    }

    fn next_token(&mut self) {
        self.cur_token = self.lexer.next_token();
    }

    fn new_t(&mut self) -> String {
        self.t_idx += 1;
        format!("t{}", self.t_idx)
    }

    fn emit(&mut self, op: &str, a1: &str, a2: &str, res: &str) {
        let quad = Quad {
            op: op.to_string(),
            arg1: a1.to_string(),
            arg2: a2.to_string(),
            result: res.to_string(),
        };
        println!("{:03}: {:<10} {:<10} {:<10} {:<10}", self.quads.len(), op, a1, a2, res);
        self.quads.push(quad);
    }

    fn factor(&mut self) -> String {
        let mut res = String::new();
        if self.cur_token.token_type == TokenType::Num {
            res = self.new_t();
            self.emit("IMM", &self.cur_token.text.clone(), "-", &res);
            self.next_token();
        } else if self.cur_token.token_type == TokenType::Id {
            let name = self.cur_token.text.clone();
            self.next_token();
            if self.cur_token.token_type == TokenType::LParen {
                self.next_token();
                let mut count = 0;
                while self.cur_token.token_type != TokenType::RParen {
                    let arg = self.expression();
                    self.emit("PARAM", &arg, "-", "-");
                    count += 1;
                    if self.cur_token.token_type == TokenType::Comma {
                        self.next_token();
                    }
                }
                self.next_token();
                res = self.new_t();
                self.emit("CALL", &name, &count.to_string(), &res);
            } else {
                res = name;
            }
        } else if self.cur_token.token_type == TokenType::LParen {
            self.next_token();
            res = self.expression();
            self.next_token();
        }
        res
    }

    fn term(&mut self) -> String {
        let mut l = self.factor();
        while self.cur_token.token_type == TokenType::Mul || self.cur_token.token_type == TokenType::Div {
            let op = if self.cur_token.token_type == TokenType::Mul { "MUL" } else { "DIV" };
            self.next_token();
            let r = self.factor();
            let t = self.new_t();
            self.emit(op, &l, &r, &t);
            l = t;
        }
        l
    }

    fn arith_expr(&mut self) -> String {
        let mut l = self.term();
        while self.cur_token.token_type == TokenType::Plus || self.cur_token.token_type == TokenType::Minus {
            let op = if self.cur_token.token_type == TokenType::Plus { "ADD" } else { "SUB" };
            self.next_token();
            let r = self.term();
            let t = self.new_t();
            self.emit(op, &l, &r, &t);
            l = t;
        }
        l
    }

    fn expression(&mut self) -> String {
        let l = self.arith_expr();
        if matches!(
            self.cur_token.token_type,
            TokenType::Eq | TokenType::Lt | TokenType::Gt
        ) {
            let op = match self.cur_token.token_type {
                TokenType::Eq => "CMP_EQ",
                TokenType::Lt => "CMP_LT",
                _ => "CMP_GT",
            };
            self.next_token();
            let r = self.arith_expr();
            let t = self.new_t();
            self.emit(op, &l, &r, &t);
            t
        } else {
            l
        }
    }

    fn statement(&mut self) {
        if self.cur_token.token_type == TokenType::If {
            self.next_token();
            self.next_token(); // skip '('
            let cond = self.expression();
            self.next_token(); // skip ')'
            self.next_token(); // skip '{'
            
            let jmp_idx = self.quads.len();
            self.emit("JMP_F", &cond, "-", "?"); // Backpatching 預留位置
            
            while self.cur_token.token_type != TokenType::RBrace {
                self.statement();
            }
            self.next_token(); // skip '}'
            
            // 回填真實的跳轉地址
            self.quads[jmp_idx].result = self.quads.len().to_string();
            
        } else if self.cur_token.token_type == TokenType::Id {
            let name = self.cur_token.text.clone();
            self.next_token();
            if self.cur_token.token_type == TokenType::Assign {
                self.next_token();
                let res = self.expression();
                self.emit("STORE", &res, "-", &name);
                if self.cur_token.token_type == TokenType::Semicolon {
                    self.next_token();
                }
            }
        } else if self.cur_token.token_type == TokenType::Return {
            self.next_token();
            let res = self.expression();
            self.emit("RET_VAL", &res, "-", "-");
            if self.cur_token.token_type == TokenType::Semicolon {
                self.next_token();
            }
        }
    }

    fn parse_program(&mut self) {
        while self.cur_token.token_type != TokenType::Eof {
            if self.cur_token.token_type == TokenType::Func {
                self.next_token();
                let f_name = self.cur_token.text.clone();
                self.emit("FUNC_BEG", &f_name, "-", "-");
                self.next_token();
                self.next_token(); // skip '('
                
                while self.cur_token.token_type == TokenType::Id {
                    self.emit("FORMAL", &self.cur_token.text.clone(), "-", "-");
                    self.next_token();
                    if self.cur_token.token_type == TokenType::Comma {
                        self.next_token();
                    }
                }
                self.next_token(); // skip ')'
                self.next_token(); // skip '{'
                
                while self.cur_token.token_type != TokenType::RBrace {
                    self.statement();
                }
                self.emit("FUNC_END", &f_name, "-", "-");
                self.next_token(); // skip '}'
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
    // 為了模擬 C 語言按順序印出變數，我們使用 Vec 存放 Tuple，而不是 HashMap
    vars: Vec<(String, i32)>, 
    ret_pc: usize,
    ret_var: String,
    incoming_args: Vec<i32>,
    formal_idx: usize,
}

impl Frame {
    fn new(ret_pc: usize, ret_var: String) -> Self {
        Self {
            vars: Vec::new(),
            ret_pc,
            ret_var,
            incoming_args: Vec::new(),
            formal_idx: 0,
        }
    }
}

struct VM {
    quads: Vec<Quad>,
    stack: Vec<Frame>,
    sp: usize,
}

impl VM {
    fn new(quads: Vec<Quad>) -> Self {
        let mut stack = Vec::new();
        stack.push(Frame::new(0, String::new())); // 全域 Frame (sp = 0)
        Self {
            quads,
            stack,
            sp: 0,
        }
    }

    fn get_var(&self, name: &str) -> i32 {
        if let Ok(val) = name.parse::<i32>() {
            return val;
        }
        if name == "-" {
            return 0;
        }
        for (k, v) in &self.stack[self.sp].vars {
            if k == name {
                return *v;
            }
        }
        0
    }

    fn set_var(&mut self, name: &str, val: i32) {
        for (k, v) in &mut self.stack[self.sp].vars {
            if k == name {
                *v = val;
                return;
            }
        }
        self.stack[self.sp].vars.push((name.to_string(), val));
    }

    fn run(&mut self) {
        let mut pc = 0;
        let mut param_stack = Vec::new();
        let mut func_map = HashMap::new();

        // 預先掃描所有函數的進入點
        for (i, q) in self.quads.iter().enumerate() {
            if q.op == "FUNC_BEG" {
                func_map.insert(q.arg1.clone(), i + 1);
            }
        }

        println!("\n=== VM 執行開始 ===");

        while pc < self.quads.len() {
            let q = self.quads[pc].clone();

            match q.op.as_str() {
                "FUNC_BEG" => {
                    // 跳過整個函數定義
                    while self.quads[pc].op != "FUNC_END" {
                        pc += 1;
                    }
                }
                "IMM" => self.set_var(&q.result, q.arg1.parse().unwrap_or(0)),
                "ADD" => self.set_var(&q.result, self.get_var(&q.arg1) + self.get_var(&q.arg2)),
                "SUB" => self.set_var(&q.result, self.get_var(&q.arg1) - self.get_var(&q.arg2)),
                "MUL" => self.set_var(&q.result, self.get_var(&q.arg1) * self.get_var(&q.arg2)),
                "DIV" => self.set_var(&q.result, self.get_var(&q.arg1) / self.get_var(&q.arg2)),
                "CMP_EQ" => self.set_var(&q.result, if self.get_var(&q.arg1) == self.get_var(&q.arg2) { 1 } else { 0 }),
                "CMP_LT" => self.set_var(&q.result, if self.get_var(&q.arg1) < self.get_var(&q.arg2) { 1 } else { 0 }),
                "CMP_GT" => self.set_var(&q.result, if self.get_var(&q.arg1) > self.get_var(&q.arg2) { 1 } else { 0 }),
                "STORE" => self.set_var(&q.result, self.get_var(&q.arg1)),
                "JMP_F" => {
                    if self.get_var(&q.arg1) == 0 {
                        pc = q.result.parse::<usize>().unwrap() - 1;
                    }
                }
                "PARAM" => {
                    param_stack.push(self.get_var(&q.arg1));
                }
                "CALL" => {
                    let p_count: usize = q.arg2.parse().unwrap();
                    let target_pc = *func_map.get(&q.arg1).expect("找不到函數");

                    let mut new_frame = Frame::new(pc + 1, q.result.clone());
                    
                    // 從參數暫存區把值拿過來
                    let start_idx = param_stack.len() - p_count;
                    for i in 0..p_count {
                        new_frame.incoming_args.push(param_stack[start_idx + i]);
                    }
                    param_stack.truncate(start_idx); // 清除已使用的參數
                    
                    self.stack.push(new_frame);
                    self.sp += 1;
                    pc = target_pc;
                    continue;
                }
                "FORMAL" => {
                    let val = self.stack[self.sp].incoming_args[self.stack[self.sp].formal_idx];
                    self.stack[self.sp].formal_idx += 1;
                    self.set_var(&q.arg1, val);
                }
                "RET_VAL" => {
                    let ret_val = self.get_var(&q.arg1);
                    let ret_pc = self.stack[self.sp].ret_pc;
                    let target_var = self.stack[self.sp].ret_var.clone();
                    
                    self.stack.pop(); // 銷毀當前堆疊幀
                    self.sp -= 1;     // 回到 Caller
                    
                    self.set_var(&target_var, ret_val); // 回填結果
                    pc = ret_pc;
                    continue;
                }
                _ => {}
            }
            pc += 1;
        }

        println!("=== VM 執行完畢 ===\n\n全域變數結果:");
        for (name, val) in &self.stack[0].vars {
            if !name.starts_with('t') { // 過濾掉臨時變數 t1, t2...
                println!(">> {} = {}", name, val);
            }
        }
    }
}

// =========================================================
// 主程式
// =========================================================
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("用法: {} <source_file>", args[0]);
        return;
    }

    let source_code = fs::read_to_string(&args[1]).unwrap_or_else(|_| {
        eprintln!("無法開啟檔案: {}", args[1]);
        std::process::exit(1);
    });

    println!("編譯器生成的中間碼 (PC: Quadruples):");
    println!("--------------------------------------------");

    let lexer = Lexer::new(&source_code);
    let mut parser = Parser::new(lexer);
    
    // 解析語法並生成 Quadruples
    parser.parse_program();

    // 將產生的 Quadruples 交給 VM 執行
    let mut vm = VM::new(parser.quads);
    vm.run();
}