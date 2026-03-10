use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::process;

// =========================================================
// 1. 詞法分析 (Lexer)
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
// 2. 語法解析 (Parser) & 中間碼 (Quad)
// =========================================================

#[derive(Clone)]
pub struct Quad {
    pub op: String,
    pub arg1: String,
    pub arg2: String,
    pub result: String,
}

// 支援 Break/Continue 跳躍到標籤
struct LoopCtx {
    break_label: String,
    continue_label: String,
}

pub struct Parser {
    lexer: Lexer,
    pub quads: Vec<Quad>,
    pub string_pool: Vec<String>,
    loop_stack: Vec<LoopCtx>,
    t_idx: usize,
    label_idx: usize, // 新增：用於產生唯一標籤 (L1, L2...)
    debug: bool,
}

impl Parser {
    pub fn new(lexer: Lexer, debug: bool) -> Self {
        Parser { 
            lexer, 
            quads: Vec::new(), 
            string_pool: Vec::new(), 
            loop_stack: Vec::new(), 
            t_idx: 0,
            label_idx: 0,
            debug,
        }
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

    // 新增：產生新的標籤名稱
    fn new_label(&mut self) -> String {
        self.label_idx += 1;
        format!("L{}", self.label_idx)
    }

    fn emit(&mut self, op: &str, a1: &str, a2: &str, res: &str) -> usize {
        let idx = self.quads.len();
        self.quads.push(Quad { op: op.to_string(), arg1: a1.to_string(), arg2: a2.to_string(), result: res.to_string() });
        if self.debug {
            println!("{:03}: {:<12} {:<10} {:<10} {:<10}", idx, op, a1, a2, res);
        }
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
                
                // [改動] 建立標籤，不再依賴絕對行號
                let l_false = self.new_label();
                self.emit("JMP_F", &cond, "-", &l_false);
                
                while self.cur().t_type != TokenType::RBrace && self.cur().t_type != TokenType::Eof { self.statement(); }
                self.expect(TokenType::RBrace, "預期 '}'");
                
                if self.cur().t_type == TokenType::Else {
                    let l_end = self.new_label();
                    self.emit("JMP", "-", "-", &l_end);        // True 區塊執行完跳轉到最後
                    self.emit("LABEL", &l_false, "-", "-");    // False 區塊起點
                    
                    self.consume(); self.expect(TokenType::LBrace, "預期 '{'");
                    while self.cur().t_type != TokenType::RBrace && self.cur().t_type != TokenType::Eof { self.statement(); }
                    self.expect(TokenType::RBrace, "預期 '}'");
                    
                    self.emit("LABEL", &l_end, "-", "-");      // 整個 If 結束標籤
                } else {
                    self.emit("LABEL", &l_false, "-", "-");    // 沒 Else 的話，False 直接跳到這裡
                }
            }
            TokenType::While => {
                self.consume(); self.expect(TokenType::LParen, "預期 '('");
                
                // [改動] 建立迴圈的起始標籤與結束標籤
                let l_start = self.new_label();
                let l_end = self.new_label();
                self.emit("LABEL", &l_start, "-", "-");        // 迴圈起點
                
                let cond = self.expression();
                self.expect(TokenType::RParen, "預期 ')'"); self.expect(TokenType::LBrace, "預期 '{'");
                
                self.emit("JMP_F", &cond, "-", &l_end);        // 條件不成立跳離迴圈
                
                self.loop_stack.push(LoopCtx { break_label: l_end.clone(), continue_label: l_start.clone() });
                
                while self.cur().t_type != TokenType::RBrace && self.cur().t_type != TokenType::Eof { self.statement(); }
                
                self.emit("JMP", "-", "-", &l_start);          // 執行完一圈，跳回起點
                self.emit("LABEL", &l_end, "-", "-");          // 迴圈結束點
                
                self.expect(TokenType::RBrace, "預期 '}'");
                self.loop_stack.pop();
            }
            TokenType::For => {
                self.consume();
                self.expect(TokenType::LParen, "預期 '('");

                // for 初始化區塊 (可省略)
                if self.cur().t_type != TokenType::Semicolon {
                    self.expr_or_assign();
                }
                self.expect(TokenType::Semicolon, "預期 ';'");

                let l_cond = self.new_label();
                let l_body = self.new_label();
                let l_update = self.new_label();
                let l_end = self.new_label();

                // 條件判斷區
                self.emit("LABEL", &l_cond, "-", "-");
                if self.cur().t_type != TokenType::Semicolon {
                    let cond = self.expression();
                    self.emit("JMP_F", &cond, "-", &l_end);
                }
                self.expect(TokenType::Semicolon, "預期 ';'");
                self.emit("JMP", "-", "-", &l_body);

                // 更新區塊 (for 的 continue 會跳到這裡)
                self.emit("LABEL", &l_update, "-", "-");
                if self.cur().t_type != TokenType::RParen {
                    self.expr_or_assign();
                }
                self.expect(TokenType::RParen, "預期 ')'");
                self.emit("JMP", "-", "-", &l_cond);

                self.expect(TokenType::LBrace, "預期 '{'");
                self.emit("LABEL", &l_body, "-", "-");
                self.loop_stack.push(LoopCtx { break_label: l_end.clone(), continue_label: l_update.clone() });
                while self.cur().t_type != TokenType::RBrace && self.cur().t_type != TokenType::Eof {
                    self.statement();
                }
                self.emit("JMP", "-", "-", &l_update);
                self.emit("LABEL", &l_end, "-", "-");
                self.expect(TokenType::RBrace, "預期 '}'");
                self.loop_stack.pop();
            }
            TokenType::Break => {
                self.consume(); self.expect(TokenType::Semicolon, "預期 ';'");
                if let Some(ctx) = self.loop_stack.last() {
                    let lbl = ctx.break_label.clone();
                    self.emit("JMP", "-", "-", &lbl);
                } else {
                    self.error("Break 必須在迴圈內使用");
                }
            }
            TokenType::Continue => {
                self.consume(); self.expect(TokenType::Semicolon, "預期 ';'");
                if let Some(ctx) = self.loop_stack.last() {
                    let lbl = ctx.continue_label.clone();
                    self.emit("JMP", "-", "-", &lbl);
                } else {
                    self.error("Continue 必須在迴圈內使用");
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
// 3. 程式進入點：編譯並輸出 IR 到檔案
// =========================================================

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut debug = false;
    let mut positional: Vec<String> = Vec::new();
    for arg in args.iter().skip(1) {
        if arg == "-d" {
            debug = true;
        } else {
            positional.push(arg.clone());
        }
    }

    if positional.is_empty() {
        println!("用法: {} [-d] <source_file.p0> [output_file.ir0]", args[0]);
        process::exit(1);
    }
    
    let source_file = &positional[0];
    
    // 決定輸出檔名：有提供就用提供的，沒提供就把來源檔名換成 .ir0
    let output_file = if positional.len() >= 2 {
        positional[1].clone()
    } else {
        std::path::Path::new(source_file)
            .with_extension("ir0")
            .to_string_lossy()
            .into_owned()
    };
    
    let source_code = fs::read_to_string(source_file).expect("無法開啟來源檔案");
    
    println!("=== 開始編譯 ===");
    let lexer = Lexer::new(source_code);
    let mut parser = Parser::new(lexer, debug);
    parser.parse_program();
    
    // 建立輸出檔案，使用我們剛剛決定好的 output_file
    let mut out_file = File::create(&output_file).expect("無法建立輸出檔案");
    
    // 輸出字串池
    writeln!(out_file, "===STRINGS===").unwrap();
    for s in &parser.string_pool {
        writeln!(out_file, "{:?}", s).unwrap();
    }
    
    // 輸出四元組
    writeln!(out_file, "===QUADS===").unwrap();
    for q in &parser.quads {
        writeln!(out_file, "{}\t{}\t{}\t{}", q.op, q.arg1, q.arg2, q.result).unwrap();
    }
    
    println!("\n✅ 編譯成功！IR 已匯出至: {}", output_file);
}
