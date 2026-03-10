use std::collections::HashMap;
use std::env;
use std::fs;
use std::process;

// ==========================================
// 1. Lexer (詞法分析器)
// ==========================================
#[derive(Debug, Clone, PartialEq)]
enum Token {
    Fn, Let, If, Else, Return, IntType, StrType,
    Ident(String), Number(i64), StringLit(String),
    Assign, Plus, Minus, Star, Slash, Arrow,
    LParen, RParen, LBrace, RBrace, Colon, Semi, Comma,
    EOF,
}

struct Lexer {
    chars: Vec<char>,
    pos: usize,
}

impl Lexer {
    fn new(input: &str) -> Self {
        Lexer { chars: input.chars().collect(), pos: 0 }
    }

fn next_token(&mut self) -> Token {
        // 使用 loop 來處理「跳過空白」與「跳過註解」的循環
        loop {
            // 1. 跳過空白字元
            while self.pos < self.chars.len() && self.chars[self.pos].is_whitespace() {
                self.pos += 1;
            }
            if self.pos >= self.chars.len() { return Token::EOF; }

            // 2. 處理單行註解 `//`
            if self.pos + 1 < self.chars.len() && self.chars[self.pos] == '/' && self.chars[self.pos + 1] == '/' {
                // 一直往後找，直到遇到換行符號 (\n)
                while self.pos < self.chars.len() && self.chars[self.pos] != '\n' {
                    self.pos += 1;
                }
                continue; // 註解結束後，重新回到 loop 開頭跳過換行與下一行的空白
            }
            
            break; // 如果既不是空白也不是註解，就跳出迴圈開始解析 Token
        }

        let ch = self.chars[self.pos];
        
        // 解析 Identifier 變數名稱或關鍵字
        if ch.is_ascii_alphabetic() || ch == '_' {
            let mut id = String::new();
            while self.pos < self.chars.len() && (self.chars[self.pos].is_ascii_alphanumeric() || self.chars[self.pos] == '_') {
                id.push(self.chars[self.pos]);
                self.pos += 1;
            }
            return match id.as_str() {
                "fn" => Token::Fn, "let" => Token::Let, "if" => Token::If, "else" => Token::Else,
                "return" => Token::Return, "int" => Token::IntType, "str" => Token::StrType,
                _ => Token::Ident(id),
            };
        }
        
        // 解析數字
        if ch.is_ascii_digit() {
            let mut num = 0;
            while self.pos < self.chars.len() && self.chars[self.pos].is_ascii_digit() {
                num = num * 10 + (self.chars[self.pos] as i64 - '0' as i64);
                self.pos += 1;
            }
            return Token::Number(num);
        }
        
        // 解析字串
        if ch == '"' {
            self.pos += 1;
            let mut s = String::new();
            while self.pos < self.chars.len() && self.chars[self.pos] != '"' {
                s.push(self.chars[self.pos]);
                self.pos += 1;
            }
            self.pos += 1; // 跳過結尾的雙引號
            return Token::StringLit(s);
        }

        // 解析單一符號與運算子
        self.pos += 1;
        match ch {
            '+' => Token::Plus, '-' => {
                if self.pos < self.chars.len() && self.chars[self.pos] == '>' {
                    self.pos += 1; Token::Arrow
                } else { Token::Minus }
            },
            '*' => Token::Star, '/' => Token::Slash, '=' => Token::Assign,
            '(' => Token::LParen, ')' => Token::RParen, '{' => Token::LBrace, '}' => Token::RBrace,
            ':' => Token::Colon, ';' => Token::Semi, ',' => Token::Comma,
            _ => panic!("未知字元: {}", ch),
        }
    }
}

// ==========================================
// 2. AST (抽象語法樹)
// ==========================================
#[derive(Debug, Clone, PartialEq)]
enum Type { Int, Str }

#[derive(Debug, Clone)]
enum Expr {
    Number(i64), StringLit(String), Ident(String),
    Binary(Box<Expr>, Token, Box<Expr>),
    Call(String, Vec<Expr>),
}

#[derive(Debug, Clone)]
enum Stmt {
    Let(String, Type, Expr),
    Assign(String, Expr),
    If(Expr, Vec<Stmt>, Option<Vec<Stmt>>),
    Return(Expr),
    Expr(Expr),
}

#[derive(Debug, Clone)]
struct Function { name: String, args: Vec<(String, Type)>, ret_type: Type, body: Vec<Stmt> }

// ==========================================
// 3. Parser (語法分析器)
// ==========================================
struct Parser { lexer: Lexer, current: Token }

impl Parser {
    fn new(mut lexer: Lexer) -> Self {
        let current = lexer.next_token();
        Parser { lexer, current }
    }
    fn eat(&mut self, expected: Token) {
        if self.current == expected { self.current = self.lexer.next_token(); } 
        else { panic!("語法錯誤: 預期 {:?} 但獲得 {:?}", expected, self.current); }
    }
    fn parse_program(&mut self) -> Vec<Function> {
        let mut funcs = Vec::new();
        while self.current != Token::EOF { funcs.push(self.parse_function()); }
        funcs
    }
    fn parse_function(&mut self) -> Function {
        self.eat(Token::Fn);
        let name = if let Token::Ident(n) = self.current.clone() { self.eat(Token::Ident(n.clone())); n } else { panic!("預期函式名稱") };
        self.eat(Token::LParen);
        let mut args = Vec::new();
        if self.current != Token::RParen {
            loop {
                let arg_name = if let Token::Ident(n) = self.current.clone() { self.eat(Token::Ident(n.clone())); n } else { panic!("預期參數名稱") };
                self.eat(Token::Colon);
                let arg_ty = self.parse_type();
                args.push((arg_name, arg_ty));
                if self.current == Token::Comma { self.eat(Token::Comma); } else { break; }
            }
        }
        self.eat(Token::RParen);
        let mut ret_type = Type::Int; // 預設為 Int
        if self.current == Token::Arrow { self.eat(Token::Arrow); ret_type = self.parse_type(); }
        let body = self.parse_block();
        Function { name, args, ret_type, body }
    }
    fn parse_type(&mut self) -> Type {
        match self.current {
            Token::IntType => { self.eat(Token::IntType); Type::Int },
            Token::StrType => { self.eat(Token::StrType); Type::Str },
            _ => panic!("預期型別標註 (int 或 str)"),
        }
    }
    fn parse_block(&mut self) -> Vec<Stmt> {
        self.eat(Token::LBrace);
        let mut stmts = Vec::new();
        while self.current != Token::RBrace { stmts.push(self.parse_statement()); }
        self.eat(Token::RBrace);
        stmts
    }
    fn parse_statement(&mut self) -> Stmt {
        match self.current {
            Token::Let => {
                self.eat(Token::Let);
                let name = if let Token::Ident(n) = self.current.clone() { self.eat(Token::Ident(n.clone())); n } else { panic!("預期變數名稱") };
                self.eat(Token::Colon);
                let ty = self.parse_type();
                self.eat(Token::Assign);
                let expr = self.parse_expr();
                self.eat(Token::Semi);
                Stmt::Let(name, ty, expr)
            },
            Token::If => {
                self.eat(Token::If);
                let cond = self.parse_expr();
                let then_block = self.parse_block();
                let mut else_block = None;
                if self.current == Token::Else {
                    self.eat(Token::Else);
                    if self.current == Token::If { else_block = Some(vec![self.parse_statement()]); } 
                    else { else_block = Some(self.parse_block()); }
                }
                Stmt::If(cond, then_block, else_block)
            },
            Token::Return => {
                self.eat(Token::Return);
                let expr = self.parse_expr();
                self.eat(Token::Semi);
                Stmt::Return(expr)
            },
            Token::Ident(_) => {
                let expr = self.parse_expr();
                if self.current == Token::Assign {
                    if let Expr::Ident(name) = expr {
                        self.eat(Token::Assign);
                        let val = self.parse_expr();
                        self.eat(Token::Semi);
                        return Stmt::Assign(name, val);
                    } else { panic!("無效的賦值"); }
                }
                self.eat(Token::Semi);
                Stmt::Expr(expr)
            },
            _ => panic!("未知的語句起始: {:?}", self.current),
        }
    }
    fn parse_expr(&mut self) -> Expr { self.parse_term() }
    fn parse_term(&mut self) -> Expr {
        let mut node = self.parse_factor();
        while self.current == Token::Plus || self.current == Token::Minus {
            let op = self.current.clone();
            self.eat(op.clone());
            node = Expr::Binary(Box::new(node), op, Box::new(self.parse_factor()));
        }
        node
    }
    fn parse_factor(&mut self) -> Expr {
        let mut node = self.parse_primary();
        while self.current == Token::Star || self.current == Token::Slash {
            let op = self.current.clone();
            self.eat(op.clone());
            node = Expr::Binary(Box::new(node), op, Box::new(self.parse_primary()));
        }
        node
    }
    fn parse_primary(&mut self) -> Expr {
        match self.current.clone() {
            Token::Number(n) => { self.eat(Token::Number(n)); Expr::Number(n) },
            Token::StringLit(s) => { self.eat(Token::StringLit(s.clone())); Expr::StringLit(s) },
            Token::Ident(name) => {
                self.eat(Token::Ident(name.clone()));
                if self.current == Token::LParen {
                    self.eat(Token::LParen);
                    let mut args = Vec::new();
                    if self.current != Token::RParen {
                        loop {
                            args.push(self.parse_expr());
                            if self.current == Token::Comma { self.eat(Token::Comma); } else { break; }
                        }
                    }
                    self.eat(Token::RParen);
                    Expr::Call(name, args)
                } else { Expr::Ident(name) }
            },
            Token::LParen => {
                self.eat(Token::LParen);
                let expr = self.parse_expr();
                self.eat(Token::RParen);
                expr
            },
            _ => panic!("預期表達式，獲得 {:?}", self.current),
        }
    }
}

// ==========================================
// 4. CodeGen (LLVM IR 生成與強型態檢查)
// ==========================================
struct CodeGen {
    ir: String,
    reg: usize,
    label: usize,
    env: Vec<HashMap<String, (String, Type)>>,
    strings: Vec<(String, usize)>, // 儲存全域字串常數
    funcs: HashMap<String, Type>,  // 函式回傳型別
    terminated: bool,
}

impl CodeGen {
    fn new() -> Self {
        let mut funcs = HashMap::new();
        funcs.insert("print".to_string(), Type::Int); // 內建 print
        funcs.insert("to_str".to_string(), Type::Str); // to_str 會回傳 Str
        CodeGen { ir: String::new(), reg: 1, label: 1, env: vec![HashMap::new()], strings: Vec::new(), funcs, terminated: false }
    }
    fn next_reg(&mut self) -> String { let r = format!("%t{}", self.reg); self.reg += 1; r }
    fn next_label(&mut self) -> String { let l = format!("L{}", self.label); self.label += 1; l }
    fn llvm_type(ty: &Type) -> &'static str { match ty { Type::Int => "i64", Type::Str => "ptr" } }

    fn gen_expr(&mut self, expr: &Expr) -> (String, Type) {
        match expr {
            Expr::Number(n) => (n.to_string(), Type::Int),
            Expr::StringLit(s) => {
                let str_id = self.strings.len() + 1;
                self.strings.push((s.clone(), str_id));
                (format!("@.str.{}", str_id), Type::Str)
            },
            Expr::Ident(name) => {
                let mut found = None;
                // 1. 先從環境中找尋變數，找到後把指標和型別 clone 出來
                for scope in self.env.iter().rev() {
                    if let Some((ptr, ty)) = scope.get(name) {
                        found = Some((ptr.clone(), ty.clone()));
                        break;
                    }
                }
                
                // 2. 結束不可變借用後，再來做修改 self 的操作
                if let Some((ptr, ty)) = found {
                    let res = self.next_reg();
                    let lty = Self::llvm_type(&ty);
                    self.ir.push_str(&format!("  {} = load {}, ptr {}\n", res, lty, ptr));
                    (res, ty)
                } else {
                    panic!("未定義的變數: {}", name);
                }
            },
            Expr::Binary(left, op, right) => {
                let (l_val, l_ty) = self.gen_expr(left);
                let (r_val, r_ty) = self.gen_expr(right);
                
                if l_ty != r_ty { panic!("型別錯誤：不允許混合型別運算"); }
                
                if l_ty == Type::Str {
                    if *op == Token::Plus {
                        let res = self.next_reg();
                        self.ir.push_str(&format!("  {} = call ptr @concat(ptr {}, ptr {})\n", res, l_val, r_val));
                        return (res, Type::Str);
                    } else { panic!("字串只支援 + 運算"); }
                }

                let res = self.next_reg();
                let instr = match op {
                    Token::Plus => "add", Token::Minus => "sub",
                    Token::Star => "mul", Token::Slash => "sdiv",
                    _ => unreachable!()
                };
                self.ir.push_str(&format!("  {} = {} i64 {}, {}\n", res, instr, l_val, r_val));
                (res, Type::Int)
            },
            Expr::Call(name, args) => {
                let mut arg_vals = Vec::new();
                for a in args { arg_vals.push(self.gen_expr(a)); }
                let ret_ty = self.funcs.get(name).unwrap_or_else(|| panic!("未知的函式: {}", name)).clone();
                let res = self.next_reg();
                let mut arg_str = String::new();
                for (i, (val, ty)) in arg_vals.iter().enumerate() {
                    if i > 0 { arg_str.push_str(", "); }
                    arg_str.push_str(&format!("{} {}", Self::llvm_type(ty), val));
                }
                self.ir.push_str(&format!("  {} = call {} @{}({})\n", res, Self::llvm_type(&ret_ty), name, arg_str));
                (res, ret_ty)
            }
        }
    }
    fn gen_stmt(&mut self, stmt: &Stmt) {
        if self.terminated { return; }
        match stmt {
            Stmt::Let(name, ty, expr) => {
                let (val, e_ty) = self.gen_expr(expr);
                if *ty != e_ty { panic!("強型態錯誤：變數 {} 宣告為 {:?} 但賦予了 {:?}", name, ty, e_ty); }
                let ptr = self.next_reg();
                let lty = Self::llvm_type(ty);
                self.ir.push_str(&format!("  {} = alloca {}\n", ptr, lty));
                self.ir.push_str(&format!("  store {} {}, ptr {}\n", lty, val, ptr));
                self.env.last_mut().unwrap().insert(name.clone(), (ptr, ty.clone()));
            },
            Stmt::Assign(name, expr) => {
                let (val, e_ty) = self.gen_expr(expr);
                let mut target_ptr = None;
                for scope in self.env.iter().rev() {
                    if let Some((ptr, ty)) = scope.get(name) {
                        if *ty != e_ty { panic!("強型態錯誤：無法賦值 {:?} 給變數 {}", e_ty, name); }
                        target_ptr = Some((ptr.clone(), ty.clone())); break;
                    }
                }
                if let Some((ptr, ty)) = target_ptr {
                    self.ir.push_str(&format!("  store {} {}, ptr {}\n", Self::llvm_type(&ty), val, ptr));
                } else { panic!("賦值給未定義的變數: {}", name); }
            },
            Stmt::If(cond, then_b, else_b) => {
                let (c_val, c_ty) = self.gen_expr(cond);
                if c_ty != Type::Int { panic!("強型態錯誤：If 條件必須為 int (0 或 非0整數)"); }
                
                let cond_bool = self.next_reg();
                self.ir.push_str(&format!("  {} = icmp ne i64 {}, 0\n", cond_bool, c_val));
                
                let label_then = self.next_label();
                let label_else = self.next_label();
                let label_merge = self.next_label();
                
                self.ir.push_str(&format!("  br i1 {}, label %{}, label %{}\n", cond_bool, label_then, label_else));
                
                // Then Block
                self.ir.push_str(&format!("{}:\n", label_then));
                self.env.push(HashMap::new());
                self.terminated = false;
                for s in then_b { self.gen_stmt(s); }
                self.env.pop();
                let then_term = self.terminated;
                if !then_term { self.ir.push_str(&format!("  br label %{}\n", label_merge)); }
                
                // Else Block
                self.terminated = false;
                self.ir.push_str(&format!("{}:\n", label_else));
                if let Some(eb) = else_b {
                    self.env.push(HashMap::new());
                    for s in eb { self.gen_stmt(s); }
                    self.env.pop();
                }
                let else_term = self.terminated;
                if !else_term { self.ir.push_str(&format!("  br label %{}\n", label_merge)); }
                
                // 只有當 then 或 else 其中一個沒有 return 時，才需要 merge 區塊
                if !then_term || !else_term {
                    self.ir.push_str(&format!("{}:\n", label_merge));
                }
                
                self.terminated = then_term && else_term;
            },
            Stmt::Return(expr) => {
                let (val, ty) = self.gen_expr(expr);
                self.ir.push_str(&format!("  ret {} {}\n", Self::llvm_type(&ty), val));
                self.terminated = true;
            },
            Stmt::Expr(expr) => { self.gen_expr(expr); },
        }
    }

    fn compile(&mut self, prog: Vec<Function>) -> String {
        for f in &prog { self.funcs.insert(f.name.clone(), f.ret_type.clone()); }
        
        for f in prog {
            self.reg = 1; self.label = 1; self.terminated = false;
            self.env = vec![HashMap::new()];
            
            let mut args_str = String::new();
            for (i, (name, ty)) in f.args.iter().enumerate() {
                if i > 0 { args_str.push_str(", "); }
                args_str.push_str(&format!("{} %arg_{}", Self::llvm_type(ty), name));
            }
            self.ir.push_str(&format!("define {} @{}({}) {{\nentry:\n", Self::llvm_type(&f.ret_type), f.name, args_str));
            
            // Allocate args to local stack variables for mutability
            for (name, ty) in &f.args {
                let ptr = self.next_reg();
                let lty = Self::llvm_type(ty);
                self.ir.push_str(&format!("  {} = alloca {}\n", ptr, lty));
                self.ir.push_str(&format!("  store {} %arg_{}, ptr {}\n", lty, name, ptr));
                self.env[0].insert(name.clone(), (ptr, ty.clone()));
            }

            for s in &f.body { self.gen_stmt(s); }
            
            if !self.terminated {
                match f.ret_type { // 保障基本的 ret 避免 LLVM 報錯
                    Type::Int => self.ir.push_str("  ret i64 0\n"),
                    Type::Str => self.ir.push_str("  ret ptr null\n"),
                }
            }
            self.ir.push_str("}\n\n");
        }

        // 宣告全域字串
        let mut final_ir = String::new();
        for (s, id) in &self.strings {
            let mut chars = s.as_bytes().to_vec();
            chars.push(0); // Null terminator
            let len = chars.len();
            let mut hex_str = String::new();
            for c in chars { hex_str.push_str(&format!("\\{:02X}", c)); }
            final_ir.push_str(&format!("@.str.{} = private unnamed_addr constant [{} x i8] c\"{}\"\n", id, len, hex_str));
        }
        
        // 外部 C 函數依賴
        final_ir.push_str("\ndeclare ptr @concat(ptr, ptr)\n");
        final_ir.push_str("declare i64 @print(ptr)\n\n");
        final_ir.push_str("declare ptr @to_str(i64)\n\n");
        final_ir.push_str(&self.ir);
        
        final_ir
    }
}

// ==========================================
// 5. 主程式 Entry (CLI)
// ==========================================
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("使用方法: tp0c <file.tp0>");
        process::exit(1);
    }

    let filepath = &args[1];
    let source = fs::read_to_string(filepath).expect("無法讀取檔案");

    // 1. 詞法分析與語法分析
    let lexer = Lexer::new(&source);
    let mut parser = Parser::new(lexer);
    let ast = parser.parse_program();

    // 2. 生成 LLVM IR (含型別檢查)
    let mut codegen = CodeGen::new();
    let llvm_ir = codegen.compile(ast);

    // 3. 輸出到 .ll
    let out_path = filepath.replace(".tp0", ".ll");
    fs::write(&out_path, llvm_ir).expect("寫入 LLVM IR 失敗");
    println!("編譯成功！已產出: {}", out_path);
}