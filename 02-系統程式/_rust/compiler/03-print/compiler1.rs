use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

// ==========================================================
// 1. 定義 Token 與 AST (保持不變)
// ==========================================================

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Fn, Let, If, Else, Return,
    Ident(String),
    Int(i32),
    Assign, Plus, Minus, Mul, Div, Eq,
    LParen, RParen, LBrace, RBrace, Semi, Comma,
    EOF,
}

#[derive(Debug, Clone)]
enum Expr {
    Number(i32),
    Variable(String),
    BinaryOp(Box<Expr>, Token, Box<Expr>),
    Call(String, Vec<Expr>),
}

#[derive(Debug, Clone)]
enum Stmt {
    VarDecl(String, Expr),
    If(Expr, Vec<Stmt>, Option<Vec<Stmt>>),
    Return(Expr),
    FuncDecl(String, Vec<String>, Vec<Stmt>),
}

// ==========================================================
// 2. Lexer (保持不變)
// ==========================================================

struct Lexer {
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    fn new(input: &str) -> Self {
        Self { input: input.chars().collect(), pos: 0 }
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        if self.pos >= self.input.len() { return Token::EOF; }

        let ch = self.input[self.pos];
        if ch.is_alphabetic() {
            let start = self.pos;
            while self.pos < self.input.len() && (self.input[self.pos].is_alphanumeric() || self.input[self.pos] == '_') {
                self.pos += 1;
            }
            let s: String = self.input[start..self.pos].iter().collect();
            return match s.as_str() {
                "fn" => Token::Fn,
                "let" => Token::Let,
                "if" => Token::If,
                "else" => Token::Else,
                "return" => Token::Return,
                _ => Token::Ident(s),
            };
        }

        if ch.is_digit(10) {
            let start = self.pos;
            while self.pos < self.input.len() && self.input[self.pos].is_digit(10) {
                self.pos += 1;
            }
            let s: String = self.input[start..self.pos].iter().collect();
            return Token::Int(s.parse().unwrap());
        }

        self.pos += 1;
        match ch {
            '=' => {
                if self.peek() == '=' { self.pos += 1; Token::Eq }
                else { Token::Assign }
            }
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Mul,
            '/' => Token::Div,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            ';' => Token::Semi,
            ',' => Token::Comma,
            _ => panic!("未知的字元: {}", ch),
        }
    }

    fn peek(&self) -> char {
        if self.pos < self.input.len() { self.input[self.pos] } else { '\0' }
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.input.len() && self.input[self.pos].is_whitespace() {
            self.pos += 1;
        }
    }
}

// ==========================================================
// 3. Parser (保持不變)
// ==========================================================

struct Parser {
    lexer: Lexer,
    cur_tok: Token,
}

impl Parser {
    fn new(mut lexer: Lexer) -> Self {
        let cur_tok = lexer.next_token();
        Self { lexer, cur_tok }
    }

    fn next(&mut self) { self.cur_tok = self.lexer.next_token(); }

    fn parse_program(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        while self.cur_tok != Token::EOF {
            if self.cur_tok == Token::Fn {
                stmts.push(self.parse_function());
            } else {
                self.next();
            }
        }
        stmts
    }

    fn parse_function(&mut self) -> Stmt {
        self.next(); // eat fn
        let name = if let Token::Ident(n) = &self.cur_tok { n.clone() } else { panic!("預期函數名稱") };
        self.next();
        self.next(); // eat (
        let mut params = Vec::new();
        while self.cur_tok != Token::RParen {
            if let Token::Ident(p) = &self.cur_tok { params.push(p.clone()); }
            self.next();
            if self.cur_tok == Token::Comma { self.next(); }
        }
        self.next(); // eat )
        let body = self.parse_block();
        Stmt::FuncDecl(name, params, body)
    }

    fn parse_block(&mut self) -> Vec<Stmt> {
        self.next(); // eat {
        let mut stmts = Vec::new();
        while self.cur_tok != Token::RBrace && self.cur_tok != Token::EOF {
            stmts.push(self.parse_stmt());
        }
        self.next(); // eat }
        stmts
    }

    fn parse_stmt(&mut self) -> Stmt {
        match &self.cur_tok {
            Token::Let => {
                self.next();
                let name = if let Token::Ident(n) = &self.cur_tok { n.clone() } else { panic!("預期變數名") };
                self.next(); self.next(); // eat =
                let expr = self.parse_expr(0);
                if self.cur_tok == Token::Semi { self.next(); }
                Stmt::VarDecl(name, expr)
            }
            Token::If => {
                self.next(); self.next(); // eat if (
                let cond = self.parse_expr(0);
                self.next(); // eat )
                let then_part = self.parse_block();
                let mut else_part = None;
                if self.cur_tok == Token::Else {
                    self.next();
                    else_part = Some(self.parse_block());
                }
                Stmt::If(cond, then_part, else_part)
            }
            Token::Return => {
                self.next();
                let expr = self.parse_expr(0);
                if self.cur_tok == Token::Semi { self.next(); }
                Stmt::Return(expr)
            }
            _ => panic!("未知的語句起始: {:?}", self.cur_tok),
        }
    }

    fn parse_expr(&mut self, prec: i32) -> Expr {
        let mut left = match &self.cur_tok {
            Token::Int(v) => { let e = Expr::Number(*v); self.next(); e }
            Token::Ident(name) => {
                let n = name.clone();
                self.next();
                if self.cur_tok == Token::LParen {
                    self.next();
                    let mut args = Vec::new();
                    while self.cur_tok != Token::RParen {
                        args.push(self.parse_expr(0));
                        if self.cur_tok == Token::Comma { self.next(); }
                    }
                    self.next();
                    Expr::Call(n, args)
                } else {
                    Expr::Variable(n)
                }
            }
            _ => panic!("無效的表達式: {:?}", self.cur_tok),
        };

        while let Some(p) = self.get_prec(&self.cur_tok) {
            if p < prec { break; }
            let op = self.cur_tok.clone();
            self.next();
            let right = self.parse_expr(p + 1);
            left = Expr::BinaryOp(Box::new(left), op, Box::new(right));
        }
        left
    }

    fn get_prec(&self, tok: &Token) -> Option<i32> {
        match tok {
            Token::Eq => Some(1),
            Token::Plus | Token::Minus => Some(2),
            Token::Mul | Token::Div => Some(3),
            _ => None,
        }
    }
}

// ==========================================================
// 4. IR Generator & VM (執行引擎與序列化)
// ==========================================================

#[derive(Debug, Clone)]
enum IR {
    LoadConst(String, i32),
    LoadVar(String, String),
    StoreVar(String, String),
    Add(String, String, String),
    Sub(String, String, String),
    Mul(String, String, String),
    Eq(String, String, String),
    Call(String, Vec<String>, String),
    Return(String),
    IfFalse(String, usize),
    Goto(usize),
    Label,
}

struct VM {
    // 儲存格式: { 函數名: (參數列表, 指令序列) }
    functions: HashMap<String, (Vec<String>, Vec<IR>)>,
}

impl VM {
    fn new() -> Self { Self { functions: HashMap::new() } }

    // --- 編譯 AST 到 IR ---
    fn compile(&mut self, stmts: Vec<Stmt>) {
        for stmt in stmts {
            if let Stmt::FuncDecl(name, params, body) = stmt {
                let mut irs = Vec::new();
                let mut temp_idx = 0;
                let mut label_idx = 0;
                for s in body {
                    self.gen_stmt(&s, &mut irs, &mut temp_idx, &mut label_idx);
                }
                self.functions.insert(name, (params, irs));
            }
        }
    }

    fn gen_expr(&self, expr: &Expr, irs: &mut Vec<IR>, t_idx: &mut i32, l_idx: &mut i32) -> String {
        match expr {
            Expr::Number(v) => {
                let t = format!("t{}", t_idx); *t_idx += 1;
                irs.push(IR::LoadConst(t.clone(), *v));
                t
            }
            Expr::Variable(n) => {
                let t = format!("t{}", t_idx); *t_idx += 1;
                irs.push(IR::LoadVar(t.clone(), n.clone()));
                t
            }
            Expr::BinaryOp(l, op, r) => {
                let lt = self.gen_expr(l, irs, t_idx, l_idx);
                let rt = self.gen_expr(r, irs, t_idx, l_idx);
                let t = format!("t{}", t_idx); *t_idx += 1;
                match op {
                    Token::Plus => irs.push(IR::Add(t.clone(), lt, rt)),
                    Token::Minus => irs.push(IR::Sub(t.clone(), lt, rt)),
                    Token::Mul => irs.push(IR::Mul(t.clone(), lt, rt)),
                    Token::Eq => irs.push(IR::Eq(t.clone(), lt, rt)),
                    _ => {}
                }
                t
            }
            Expr::Call(name, args) => {
                let mut arg_temps = Vec::new();
                for a in args { arg_temps.push(self.gen_expr(a, irs, t_idx, l_idx)); }
                let t = format!("t{}", t_idx); *t_idx += 1;
                irs.push(IR::Call(name.clone(), arg_temps, t.clone()));
                t
            }
        }
    }

    fn gen_stmt(&self, stmt: &Stmt, irs: &mut Vec<IR>, t_idx: &mut i32, l_idx: &mut i32) {
        match stmt {
            Stmt::VarDecl(name, expr) => {
                let t = self.gen_expr(expr, irs, t_idx, l_idx);
                irs.push(IR::StoreVar(name.clone(), t));
            }
            Stmt::Return(expr) => {
                let t = self.gen_expr(expr, irs, t_idx, l_idx);
                irs.push(IR::Return(t));
            }
            Stmt::If(cond, then_part, else_part) => {
                let ct = self.gen_expr(cond, irs, t_idx, l_idx);
                let if_false_pos = irs.len();
                irs.push(IR::IfFalse(ct.clone(), 0)); 
                for s in then_part { self.gen_stmt(s, irs, t_idx, l_idx); }
                if let Some(else_stmts) = else_part {
                    let goto_pos = irs.len();
                    irs.push(IR::Goto(0)); 
                    irs[if_false_pos] = IR::IfFalse(ct, irs.len());
                    for s in else_stmts { self.gen_stmt(s, irs, t_idx, l_idx); }
                    irs[goto_pos] = IR::Goto(irs.len());
                } else {
                    irs[if_false_pos] = IR::IfFalse(ct, irs.len());
                }
                irs.push(IR::Label);
            }
            _ => {}
        }
    }

    // --- 輸出中間碼到文字 ---
    fn dump_ir(&self) -> String {
        let mut output = String::new();
        for (name, (params, irs)) in &self.functions {
            output.push_str(&format!("FUNC {} {}\n", name, params.join(" ")));
            for ir in irs {
                let line = match ir {
                    IR::LoadConst(t, v) => format!("  LOAD_CONST {} {}", t, v),
                    IR::LoadVar(t, n) => format!("  LOAD_VAR {} {}", t, n),
                    IR::StoreVar(n, t) => format!("  STORE_VAR {} {}", n, t),
                    IR::Add(t, l, r) => format!("  ADD {} {} {}", t, l, r),
                    IR::Sub(t, l, r) => format!("  SUB {} {} {}", t, l, r),
                    IR::Mul(t, l, r) => format!("  MUL {} {} {}", t, l, r),
                    IR::Eq(t, l, r) => format!("  EQ {} {} {}", t, l, r),
                    IR::Call(f, args, t) => format!("  CALL {} {} {}", f, t, args.join(" ")),
                    IR::Return(t) => format!("  RETURN {}", t),
                    IR::IfFalse(t, target) => format!("  IFFALSE {} {}", t, target),
                    IR::Goto(target) => format!("  GOTO {}", target),
                    IR::Label => format!("  LABEL"),
                };
                output.push_str(&line);
                output.push('\n');
            }
            output.push_str("ENDFUNC\n\n");
        }
        output
    }

    // --- 從文字載入中間碼 ---
    fn load_ir(&mut self, content: &str) {
        let lines: Vec<&str> = content.lines().collect();
        let mut i = 0;
        while i < lines.len() {
            let line = lines[i].trim();
            if line.is_empty() { i += 1; continue; }

            if line.starts_with("FUNC") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                let func_name = parts[1].to_string();
                let params = parts[2..].iter().map(|s| s.to_string()).collect();
                let mut irs = Vec::new();
                i += 1;
                while i < lines.len() {
                    let ir_line = lines[i].trim();
                    if ir_line == "ENDFUNC" { break; }
                    let p: Vec<&str> = ir_line.split_whitespace().collect();
                    let ir = match p[0] {
                        "LOAD_CONST" => IR::LoadConst(p[1].into(), p[2].parse().unwrap()),
                        "LOAD_VAR"   => IR::LoadVar(p[1].into(), p[2].into()),
                        "STORE_VAR"  => IR::StoreVar(p[1].into(), p[2].into()),
                        "ADD"        => IR::Add(p[1].into(), p[2].into(), p[3].into()),
                        "SUB"        => IR::Sub(p[1].into(), p[2].into(), p[3].into()),
                        "MUL"        => IR::Mul(p[1].into(), p[2].into(), p[3].into()),
                        "EQ"         => IR::Eq(p[1].into(), p[2].into(), p[3].into()),
                        "RETURN"     => IR::Return(p[1].into()),
                        "IFFALSE"    => IR::IfFalse(p[1].into(), p[2].parse().unwrap()),
                        "GOTO"       => IR::Goto(p[1].parse().unwrap()),
                        "LABEL"      => IR::Label,
                        "CALL"       => {
                            let f_name = p[1].to_string();
                            let res_t = p[2].to_string();
                            let args = p[3..].iter().map(|s| s.to_string()).collect();
                            IR::Call(f_name, args, res_t)
                        }
                        _ => panic!("未知指令: {}", p[0]),
                    };
                    irs.push(ir);
                    i += 1;
                }
                self.functions.insert(func_name, (params, irs));
            }
            i += 1;
        }
    }

    // --- 執行 VM (遞迴執行) ---
    fn run(&self, func_name: &str, args: Vec<i32>) -> i32 {
        let (params, code) = self.functions.get(func_name).expect("找不到函數");
        let mut locals = HashMap::<String, i32>::new();
        for (i, p) in params.iter().enumerate() {
            locals.insert(p.clone(), args[i]);
        }

        let mut ip = 0;
        let mut temps = HashMap::<String, i32>::new();

        while ip < code.len() {
            match &code[ip] {
                IR::LoadConst(t, v) => { temps.insert(t.clone(), *v); }
                IR::LoadVar(t, n) => { temps.insert(t.clone(), *locals.get(n).unwrap_or(&0)); }
                IR::StoreVar(n, t) => { locals.insert(n.clone(), *temps.get(t).unwrap()); }
                IR::Add(t, l, r) => { temps.insert(t.clone(), temps[l] + temps[r]); }
                IR::Sub(t, l, r) => { temps.insert(t.clone(), temps[l] - temps[r]); }
                IR::Mul(t, l, r) => { temps.insert(t.clone(), temps[l] * temps[r]); }
                IR::Eq(t, l, r) => { temps.insert(t.clone(), if temps[l] == temps[r] { 1 } else { 0 }); }
                IR::IfFalse(t, target) => { if temps[t] == 0 { ip = *target; continue; } }
                IR::Goto(target) => { ip = *target; continue; }
                IR::Return(t) => return temps[t],
                IR::Call(name, arg_temps, result_t) => {
                    let call_args: Vec<i32> = arg_temps.iter().map(|at| temps[at]).collect();
                    let res = self.run(name, call_args);
                    temps.insert(result_t.clone(), res);
                }
                IR::Label => {}
            }
            ip += 1;
        }
        0
    }
}

// ==========================================================
// 5. 主程式
// ==========================================================

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("用法: {} <source_file.p | ir_file.ir>", args[0]);
        process::exit(1);
    }

    let file_path = &args[1];
    let extension = Path::new(file_path).extension().and_then(|s| s.to_str()).unwrap_or("");

    let mut vm = VM::new();

    if extension == "ir" {
        // --- 情境 A: 載入中間碼執行 ---
        println!("正在從載入中間碼: {}...", file_path);
        let ir_content = fs::read_to_string(file_path).expect("無法讀取 IR 檔案");
        vm.load_ir(&ir_content);
    } else {
        // --- 情境 B: 編譯原始碼 -> 儲存 IR -> 執行 ---
        println!("正在編譯原始碼: {}...", file_path);
        let source = fs::read_to_string(file_path).expect("無法讀取原始碼檔案");
        
        let lexer = Lexer::new(&source);
        let mut parser = Parser::new(lexer);
        let ast = parser.parse_program();

        vm.compile(ast);

        // 儲存 IR
        let ir_output = vm.dump_ir();
        let ir_filename = Path::new(file_path).with_extension("ir");
        fs::write(&ir_filename, ir_output).expect("無法寫入 IR 檔案");
        println!("中間碼已儲存至: {}", ir_filename.display());
    }

    println!("--- 虛擬機執行階段 ---");
    let result = vm.run("main", vec![]);
    println!("main 函數回傳值: {}", result);
}