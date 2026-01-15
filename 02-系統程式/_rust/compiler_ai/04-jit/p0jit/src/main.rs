use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{Linkage, Module};
use std::collections::HashMap;
use std::env;
use std::fs;

// ==========================================================
// 1. 定義 Token 與 AST
// ==========================================================

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Fn, Let, If, Else, Return,
    Ident(String),
    Int(i32),
    Str(String),
    Assign, Plus, Minus, Mul, Div, Eq,
    LParen, RParen, LBrace, RBrace, Semi, Comma,
    EOF,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Number(i32),
    Variable(String),
    #[allow(dead_code)]
    Str(String),
    BinaryOp(Box<Expr>, Token, Box<Expr>),
    Call(String, Vec<Expr>),
}

#[derive(Debug, Clone)]
pub enum Stmt {
    VarDecl(String, Expr),
    If(Expr, Vec<Stmt>, Option<Vec<Stmt>>),
    Return(Expr),
    FuncDecl(String, Vec<String>, Vec<Stmt>),
    ExprStmt(Expr),
}

// ==========================================================
// 2. Lexer
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

        if ch == '\'' {
            self.pos += 1;
            let start = self.pos;
            while self.pos < self.input.len() && self.input[self.pos] != '\'' { self.pos += 1; }
            let s: String = self.input[start..self.pos].iter().collect();
            self.pos += 1;
            return Token::Str(s);
        }

        if ch.is_alphabetic() {
            let start = self.pos;
            while self.pos < self.input.len() && (self.input[self.pos].is_alphanumeric() || self.input[self.pos] == '_') {
                self.pos += 1;
            }
            let s: String = self.input[start..self.pos].iter().collect();
            return match s.as_str() {
                "fn" => Token::Fn, "let" => Token::Let, "if" => Token::If,
                "else" => Token::Else, "return" => Token::Return,
                _ => Token::Ident(s),
            };
        }

        if ch.is_digit(10) {
            let start = self.pos;
            while self.pos < self.input.len() && self.input[self.pos].is_digit(10) { self.pos += 1; }
            let s: String = self.input[start..self.pos].iter().collect();
            return Token::Int(s.parse().unwrap());
        }

        self.pos += 1;
        match ch {
            '=' => if self.peek() == '=' { self.pos += 1; Token::Eq } else { Token::Assign },
            '+' => Token::Plus, '-' => Token::Minus, '*' => Token::Mul, '/' => Token::Div,
            '(' => Token::LParen, ')' => Token::RParen, '{' => Token::LBrace, '}' => Token::RBrace,
            ';' => Token::Semi, ',' => Token::Comma,
            _ => panic!("未知字元: {}", ch),
        }
    }
    fn peek(&self) -> char { if self.pos < self.input.len() { self.input[self.pos] } else { '\0' } }
    fn skip_whitespace(&mut self) { while self.pos < self.input.len() && self.input[self.pos].is_whitespace() { self.pos += 1; } }
}

// ==========================================================
// 3. Parser
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
            if self.cur_tok == Token::Fn { stmts.push(self.parse_function()); }
            else { self.next(); }
        }
        stmts
    }
    fn parse_function(&mut self) -> Stmt {
        self.next(); // fn
        let name = if let Token::Ident(n) = &self.cur_tok { n.clone() } else { panic!("預期函數名稱") };
        self.next(); self.next(); // (
        let mut params = Vec::new();
        while self.cur_tok != Token::RParen {
            if let Token::Ident(p) = &self.cur_tok { params.push(p.clone()); }
            self.next();
            if self.cur_tok == Token::Comma { self.next(); }
        }
        self.next(); // )
        Stmt::FuncDecl(name, params, self.parse_block())
    }
    fn parse_block(&mut self) -> Vec<Stmt> {
        self.next(); // {
        let mut stmts = Vec::new();
        while self.cur_tok != Token::RBrace && self.cur_tok != Token::EOF { stmts.push(self.parse_stmt()); }
        self.next(); // }
        stmts
    }
    fn parse_stmt(&mut self) -> Stmt {
        match &self.cur_tok {
            Token::Let => {
                self.next();
                let name = if let Token::Ident(n) = &self.cur_tok { n.clone() } else { panic!("預期變數名") };
                self.next(); self.next(); // =
                let expr = self.parse_expr(0);
                if self.cur_tok == Token::Semi { self.next(); }
                Stmt::VarDecl(name, expr)
            }
            Token::If => {
                self.next(); self.next(); // if (
                let cond = self.parse_expr(0);
                self.next(); // )
                let then_part = self.parse_block();
                let mut else_part = None;
                if self.cur_tok == Token::Else { self.next(); else_part = Some(self.parse_block()); }
                Stmt::If(cond, then_part, else_part)
            }
            Token::Return => {
                self.next();
                let expr = self.parse_expr(0);
                if self.cur_tok == Token::Semi { self.next(); }
                Stmt::Return(expr)
            }
            _ => {
                let expr = self.parse_expr(0);
                if self.cur_tok == Token::Semi { self.next(); }
                Stmt::ExprStmt(expr)
            }
        }
    }
    fn parse_expr(&mut self, prec: i32) -> Expr {
        let mut left = match &self.cur_tok {
            Token::Int(v) => { let e = Expr::Number(*v); self.next(); e }
            Token::Str(s) => { let e = Expr::Str(s.clone()); self.next(); e }
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
                } else { Expr::Variable(n) }
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
// 4. JIT 編譯器核心
// ==========================================================

extern "C" fn p0_print_i32(val: i32) { println!("{}", val); }

pub struct JIT {
    builder_context: FunctionBuilderContext,
    ctx: codegen::Context,
    module: JITModule,
}

impl JIT {
    pub fn new() -> Self {
        let mut flag_builder = settings::builder();
        flag_builder.set("use_colocated_libcalls", "false").unwrap();
        flag_builder.set("is_pic", "false").unwrap();
        let isa_builder = cranelift_native::builder().unwrap_or_else(|msg| panic!("ISA 錯誤: {}", msg));
        let isa = isa_builder.finish(settings::Flags::new(flag_builder)).unwrap();
        let mut builder = JITBuilder::with_isa(isa, cranelift_module::default_libcall_names());
        builder.symbol("print_i32", p0_print_i32 as *const u8);
        let module = JITModule::new(builder);
        Self {
            builder_context: FunctionBuilderContext::new(),
            ctx: codegen::Context::new(),
            module,
        }
    }

pub fn compile(&mut self, program: Vec<Stmt>) -> HashMap<String, *const u8> {
        let mut function_pointers = HashMap::new();
        for stmt in program {
            if let Stmt::FuncDecl(name, params, body) = stmt {
                // --- 修正處：先建立正確的簽名再宣告 ---
                let mut sig = self.module.make_signature();
                // 根據參數數量加入 i32 型別
                for _ in &params {
                    sig.params.push(AbiParam::new(types::I32));
                }
                // 加入回傳值 i32 型別
                sig.returns.push(AbiParam::new(types::I32));

                // 使用正確的 sig 進行宣告
                let id = self.module.declare_function(&name, Linkage::Export, &sig).unwrap();
                
                let ptr = self.compile_fn(id, params, body);
                function_pointers.insert(name, ptr);
            }
        }
        function_pointers
    }
    
fn compile_fn(&mut self, id: cranelift_module::FuncId, params: Vec<String>, body: Vec<Stmt>) -> *const u8 {
        // 確保 ctx 內的簽名與剛剛宣告的一致
        self.ctx.func.signature.params.clear();
        self.ctx.func.signature.returns.clear();
        for _ in &params {
            self.ctx.func.signature.params.push(AbiParam::new(types::I32));
        }
        self.ctx.func.signature.returns.push(AbiParam::new(types::I32));

        let mut builder = FunctionBuilder::new(&mut self.ctx.func, &mut self.builder_context);
        let entry_block = builder.create_block();
        builder.append_block_params_for_function_params(entry_block);
        builder.switch_to_block(entry_block);
        builder.seal_block(entry_block);

        let mut variables = HashMap::new();
        for (i, name) in params.iter().enumerate() {
            let val = builder.block_params(entry_block)[i];
            let var = Variable::new(i);
            builder.declare_var(var, types::I32);
            builder.def_var(var, val);
            variables.insert(name.clone(), var);
        }

        let mut translator = FunctionTranslator { 
            builder, 
            variables, 
            module: &mut self.module, 
            next_var: params.len(),
            terminated: false 
        };
        
        for stmt in body { translator.translate_stmt(stmt); }
        
        if !translator.terminated {
            let zero = translator.builder.ins().iconst(types::I32, 0);
            translator.builder.ins().return_(&[zero]);
        }
        
        translator.builder.finalize();
        self.module.define_function(id, &mut self.ctx).unwrap();
        self.module.clear_context(&mut self.ctx);
        self.module.finalize_definitions().unwrap();
        self.module.get_finalized_function(id)
    }
}

struct FunctionTranslator<'a> {
    builder: FunctionBuilder<'a>,
    variables: HashMap<String, Variable>,
    module: &'a mut JITModule,
    next_var: usize,
    terminated: bool, // 手動追蹤當前 Block 是否已結束
}

impl<'a> FunctionTranslator<'a> {
    fn translate_stmt(&mut self, stmt: Stmt) {
        if self.terminated { return; } // 如果已經 Return，跳過後續指令
        match stmt {
            Stmt::VarDecl(name, expr) => {
                let val = self.translate_expr(expr);
                let var = *self.variables.entry(name).or_insert_with(|| {
                    let v = Variable::new(self.next_var);
                    self.next_var += 1;
                    self.builder.declare_var(v, types::I32);
                    v
                });
                self.builder.def_var(var, val);
            }
            Stmt::Return(expr) => {
                let val = self.translate_expr(expr);
                self.builder.ins().return_(&[val]);
                self.terminated = true;
            }
            Stmt::ExprStmt(expr) => { self.translate_expr(expr); }
            Stmt::If(cond, then_body, else_body) => {
                let cond_val = self.translate_expr(cond);
                let then_block = self.builder.create_block();
                let else_block = self.builder.create_block();
                let merge_block = self.builder.create_block();

                self.builder.ins().brif(cond_val, then_block, &[], else_block, &[]);

                // Then
                self.builder.switch_to_block(then_block);
                self.builder.seal_block(then_block);
                self.terminated = false;
                for s in then_body { self.translate_stmt(s); }
                if !self.terminated { self.builder.ins().jump(merge_block, &[]); }

                // Else
                self.builder.switch_to_block(else_block);
                self.builder.seal_block(else_block);
                self.terminated = false;
                if let Some(eb) = else_body { for s in eb { self.translate_stmt(s); } }
                if !self.terminated { self.builder.ins().jump(merge_block, &[]); }

                // Merge
                self.builder.switch_to_block(merge_block);
                self.builder.seal_block(merge_block);
                self.terminated = false;
            }
            _ => {}
        }
    }

    fn translate_expr(&mut self, expr: Expr) -> cranelift::prelude::Value {
        match expr {
            Expr::Number(n) => self.builder.ins().iconst(types::I32, n as i64),
            Expr::Variable(name) => {
                let var = self.variables.get(&name).expect("undefined variable");
                self.builder.use_var(*var)
            }
            Expr::BinaryOp(left, op, right) => {
                let lhs = self.translate_expr(*left);
                let rhs = self.translate_expr(*right);
                match op {
                    Token::Plus => self.builder.ins().iadd(lhs, rhs),
                    Token::Minus => self.builder.ins().isub(lhs, rhs),
                    Token::Mul => self.builder.ins().imul(lhs, rhs),
                    Token::Eq => {
                        let res = self.builder.ins().icmp(IntCC::Equal, lhs, rhs);
                        self.builder.ins().uextend(types::I32, res)
                    }
                    _ => self.builder.ins().iconst(types::I32, 0),
                }
            }
            Expr::Call(name, args) => {
                let mut sig = self.module.make_signature();
                if name == "print" {
                    sig.params.push(AbiParam::new(types::I32));
                    let callee = self.module.declare_function("print_i32", Linkage::Import, &sig).unwrap();
                    let local_callee = self.module.declare_func_in_func(callee, &mut self.builder.func);
                    let arg_vals: Vec<cranelift::prelude::Value> = args.into_iter().map(|a| self.translate_expr(a)).collect();
                    self.builder.ins().call(local_callee, &[arg_vals[0]]);
                    self.builder.ins().iconst(types::I32, 0)
                } else {
                    for _ in &args { sig.params.push(AbiParam::new(types::I32)); }
                    sig.returns.push(AbiParam::new(types::I32));
                    let callee = self.module.declare_function(&name, Linkage::Import, &sig).unwrap();
                    let local_callee = self.module.declare_func_in_func(callee, &mut self.builder.func);
                    let arg_vals: Vec<cranelift::prelude::Value> = args.into_iter().map(|a| self.translate_expr(a)).collect();
                    let call = self.builder.ins().call(local_callee, &arg_vals);
                    self.builder.inst_results(call)[0]
                }
            }
            _ => self.builder.ins().iconst(types::I32, 0),
        }
    }
}

// ==========================================================
// 5. 主程式
// ==========================================================

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { println!("用法: cargo run -- <source.p>"); return; }
    let source = fs::read_to_string(&args[1]).expect("讀取檔案失敗");
    let lexer = Lexer::new(&source);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    let mut jit = JIT::new();
    let symbols = jit.compile(program);
    let main_ptr = symbols.get("main").expect("找不到 main 函數");
    let main_fn: extern "C" fn() -> i32 = unsafe { std::mem::transmute(*main_ptr) };

    println!("--- JIT 執行中 ---");
    let result = main_fn();
    println!("回傳值: {}", result);
}