use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::process;

// =========================================================
// p0 語言 EBNF 語法 (EBNF Grammar for p0)
// =========================================================
// Program      = { Statement | FuncDecl } ;
// FuncDecl     = "func" Id "(" [ Id { "," Id } ] ")" "{" { Statement } "}" ;
// Statement    = IfStmt | WhileStmt | ForStmt | BreakStmt | ContinueStmt | ReturnStmt | ExprStmt ;
// IfStmt       = "if" "(" Expr ")" "{" { Statement } "}" [ "else" "{" { Statement } "}" ] ;
// WhileStmt    = "while" "(" Expr ")" "{" { Statement } "}" ;
// ForStmt      = "for" "(" [ ExprOrAssign ] ";" [ Expr ] ";" [ ExprOrAssign ] ")" "{" { Statement } "}" ;
// BreakStmt    = "break" ";" ;
// ContinueStmt = "continue" ";" ;
// ReturnStmt   = "return" Expr ";" ;
// ExprStmt     = ExprOrAssign ";" ;
//
// ExprOrAssign = Primary { "[" Expr "]" | "." Id | "(" [ Expr { "," Expr } ] ")" } [ "=" Expr ] ;
// Expr         = ArithExpr [ ( "==" | "<" | ">" ) ArithExpr ] ;
// ArithExpr    = Term { ( "+" | "-" ) Term } ;
// Term         = Factor { ( "*" | "/" ) Factor } ;
// Factor       = Primary { "[" Expr "]" | "." Id | "(" [ Expr { "," Expr } ] ")" } ;
// Primary      = Num | StringLit | Id | ArrayLit | DictLit | "(" Expr ")" ;
// ArrayLit     = "[" [ Expr ( ";" Expr | { "," Expr } ) ] "]" ;
// DictLit      = "{" [ ( Id | StringLit ) ":" Expr { "," ( Id | StringLit ) ":" Expr } ] "}" ;
// =========================================================

// =========================================================
// 1. 詞法分析 (Lexer)
// =========================================================

/// Token 類型列舉，定義了語言中所有的語彙單元 (Token)
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // 關鍵字 (保留字)
    Func, Return, If, Else, While, For, Break, Continue,
    // 識別碼、數字與字串字面量
    Id, Num, StringLit,
    // 括號與括弧
    LParen, RParen, LBrace, RBrace, LBracket, RBracket,
    // 符號與標點
    Dot, Colon, Comma, Semicolon,
    // 運算子
    Assign, Plus, Minus, Mul, Div, Eq, Lt, Gt,
    // 檔案結尾標記
    Eof,
}

/// 語彙單元 (Token) 結構，儲存解析出的 Token 類型、文字內容以及在原始碼中的位置
#[derive(Clone)]
pub struct Token {
    pub t_type: TokenType, // Token 的類型
    pub text: String,      // Token 的實際文字內容
    pub pos: usize,        // 該 Token 在原始字串中的起始位元組位置
}

/// 詞法分析器 (Lexer)，負責將原始碼字串轉換為一連串的 Token
pub struct Lexer {
    src: String,                  // 完整的原始碼字串
    pos: usize,                   // 目前讀取到的字元位置
    chars: Vec<char>,             // 將原始碼轉換為字元陣列，方便逐字元讀取
    pub cur_token: Option<Token>, // 目前剛解析出來的 Token
}

impl Lexer {
    /// 建立新的詞法分析器實例並載入第一個 Token
    pub fn new(src: String) -> Self {
        let chars = src.chars().collect();
        let mut lexer = Lexer { src, pos: 0, chars, cur_token: None };
        lexer.next_token(); // 初始化時就先讀入第一個 Token
        lexer
    }

    /// 報告語法錯誤，印出錯誤發生的行號、字元位置與錯誤訊息，並在引發錯誤的地方標示 `^`，最後終止程式
    fn report_error(&self, pos: usize, msg: &str) -> ! {
        let lines: Vec<&str> = self.src.split('\n').collect();
        let mut current_pos = 0;
        let mut line_idx = 0;
        
        // 尋找錯誤發生的行號與字元位置
        for (i, l) in lines.iter().enumerate() {
            if current_pos + l.len() + 1 > pos {
                line_idx = i;
                break;
            }
            current_pos += l.len() + 1;
        }
        
        let col_idx = if pos >= current_pos { pos - current_pos } else { 0 };
        println!("\n❌ [語法錯誤] 第 {} 行, 第 {} 字元: {}", line_idx + 1, col_idx + 1, msg);
        
        // 印出錯誤所在的原始碼，並在下方標記 ^ 符號指引
        if line_idx < lines.len() {
            println!("  {}", lines[line_idx]);
            let indicator: String = lines[line_idx].chars().take(col_idx)
                .map(|c| if c == '\t' { '\t' } else { ' ' }).collect();
            println!("  {}^", indicator);
        }
        process::exit(1);
    }

    /// 取出下一個語彙單元 (Token)，並將結果存入 `cur_token` 中
    pub fn next_token(&mut self) {
        // 略過所有的空白字元 (空格、換行、Tab 等)
        while self.pos < self.chars.len() && self.chars[self.pos].is_whitespace() {
            self.pos += 1;
        }

        // 檢查是否已達檔案結尾
        if self.pos >= self.chars.len() {
            self.cur_token = Some(Token { t_type: TokenType::Eof, text: "".to_string(), pos: self.pos });
            return;
        }

        // 處理註解
        if self.chars[self.pos] == '/' {
            if self.pos + 1 < self.chars.len() && self.chars[self.pos + 1] == '/' {
                // 單行註解 `//`，略過直到該行結束
                self.pos += 2;
                while self.pos < self.chars.len() && self.chars[self.pos] != '\n' { self.pos += 1; }
                return self.next_token();
            } else if self.pos + 1 < self.chars.len() && self.chars[self.pos + 1] == '*' {
                // 多行註解 `/* ... */`，略過直到遇到結尾的 `*/`
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

        // 處理字串字面量 (String Literal)
        if ch == '"' {
            self.pos += 1;
            let start_str = self.pos;
            while self.pos < self.chars.len() && self.chars[self.pos] != '"' { self.pos += 1; }
            if self.pos >= self.chars.len() { self.report_error(start, "字串缺少結尾的雙引號 '\"'"); }
            let text: String = self.chars[start_str..self.pos].iter().collect();
            self.pos += 1; // 略過結尾的雙引號
            self.cur_token = Some(Token { t_type: TokenType::StringLit, text, pos: start });
            return;
        }

        // 處理數字常數 (Number)
        if ch.is_ascii_digit() {
            while self.pos < self.chars.len() && self.chars[self.pos].is_ascii_digit() { self.pos += 1; }
            let text: String = self.chars[start..self.pos].iter().collect();
            self.cur_token = Some(Token { t_type: TokenType::Num, text, pos: start });
            return;
        }

        // 處理識別碼 (Identifier) 與關鍵字 (Keyword)
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
                _ => TokenType::Id, // 沒有配對到關鍵字，判定為一般識別碼
            };
            self.cur_token = Some(Token { t_type, text, pos: start });
            return;
        }

        // 處理單個或長度兩字元的符號及運算子
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
                    TokenType::Eq // ==
                } else {
                    TokenType::Assign // =
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

/// 四元式 (Quadruple) 結構，表示中間碼 (Intermediate Representation, IR)
/// 每道指令由運算元 (op)、兩個參數 (arg1, arg2) 及結果 (result) 組成
#[derive(Clone)]
pub struct Quad {
    pub op: String,     // 運算子名稱 (例如 ADD, SUB, JMP, LABEL 等)
    pub arg1: String,   // 第一個參數
    pub arg2: String,   // 第二個參數
    pub result: String, // 結果存放的地方 (可能是產生 t1, t2 等暫存變數)
}

/// 迴圈上下文結構，支援迴圈內部的 Break/Continue 跳躍到對應的標籤
struct LoopCtx {
    break_label: String,    // 迴圈結束標籤 (供 break 使用)
    continue_label: String, // 迴圈下一輪起始點標籤 (供 continue 使用)
}

/// 語法解析器 (Parser)，負責將 Token 序列解析並產生四元式中間碼 (IR)
pub struct Parser {
    lexer: Lexer,                 // 內建的詞法分析器
    pub quads: Vec<Quad>,         // 生成的所有四元式中間碼
    pub string_pool: Vec<String>, // 字串常數池
    loop_stack: Vec<LoopCtx>,     // 迴圈堆疊，追蹤當前所在迴圈的 break/continue 標籤
    t_idx: usize,                 // 用於產生唯一暫存變數 (t1, t2...)
    label_idx: usize,             // 新增：用於產生唯一標籤 (L1, L2...)
    debug: bool,                  // 是否開啟除錯模式 (列印生成的中間碼)
}

impl Parser {
    /// 建立新的解析器實例
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

    /// 取得目前指向的 Token
    fn cur(&self) -> &Token { self.lexer.cur_token.as_ref().unwrap() }
    
    /// 消耗當前 Token，並讓詞法分析器前進到下一個
    fn consume(&mut self) { self.lexer.next_token(); }
    
    /// 報告語法解析錯誤並終止編譯
    fn error(&self, msg: &str) -> ! {
        self.lexer.report_error(self.cur().pos, &format!("{} (目前讀到: '{}')", msg, self.cur().text));
    }

    /// 預期當前的 Token 是特定類型，如果是則消耗它；否則報錯
    fn expect(&mut self, expected: TokenType, msg: &str) {
        if self.cur().t_type == expected { self.consume(); } else { self.error(msg); }
    }

    /// 產生一個新的暫存變數名稱 (例如 t1, t2...)
    fn new_t(&mut self) -> String {
        self.t_idx += 1;
        format!("t{}", self.t_idx)
    }

    /// 產生一個新的控制流跳躍標籤名稱 (例如 L1, L2...)
    fn new_label(&mut self) -> String {
        self.label_idx += 1;
        format!("L{}", self.label_idx)
    }

    /// 產生並發布一道四元式 (Quad) 中間碼，加入指令列表中
    fn emit(&mut self, op: &str, a1: &str, a2: &str, res: &str) -> usize {
        let idx = self.quads.len();
        self.quads.push(Quad { op: op.to_string(), arg1: a1.to_string(), arg2: a2.to_string(), result: res.to_string() });
        // 若開啟 debug 模式，則即時印出四元式以便除錯
        if self.debug {
            println!("{:03}: {:<12} {:<10} {:<10} {:<10}", idx, op, a1, a2, res);
        }
        idx
    }

    /// 解析變數指派 (Assignment)、陣列/字典存取 (Array/Dict Access) 或函數呼叫 (Function Call)
    /// EBNF: ExprOrAssign = Primary { "[" Expr "]" | "." Id | "(" [ Expr { "," Expr } ] ")" } [ "=" Expr ] ;
    fn expr_or_assign(&mut self) {
        let obj = self.cur().text.clone();
        self.consume();
        let mut path = Vec::new(); // 儲存連續屬性存取的路徑
        let mut current_obj = obj.clone();

        // 不斷解析陣列存取 `[`、屬性存取 `.` 與函數呼叫 `(`
        while vec![TokenType::LBracket, TokenType::Dot, TokenType::LParen].contains(&self.cur().t_type) {
            match self.cur().t_type {
                TokenType::LBracket => { // 陣列索引
                    self.consume();
                    let idx = self.expression(); // 遞迴解析索引值的表達式
                    self.expect(TokenType::RBracket, "預期 ']'");
                    path.push(idx);
                }
                TokenType::Dot => { // 物件屬性
                    self.consume();
                    if self.cur().t_type != TokenType::Id { self.error("預期屬性名稱"); }
                    let key_str = self.cur().text.clone();
                    self.consume();
                    let k = self.new_t();
                    let pool_idx = self.string_pool.len();
                    self.string_pool.push(key_str); // 將字串屬性名稱加入常數池
                    self.emit("LOAD_STR", &pool_idx.to_string(), "-", &k);
                    path.push(k);
                }
                TokenType::LParen => { // 函數或方法呼叫
                    for p in &path { // 若有前綴屬性存取代碼，需先讀取屬性
                        let t = self.new_t();
                        self.emit("GET_ITEM", &current_obj, p, &t);
                        current_obj = t;
                    }
                    path.clear();
                    self.consume();
                    let mut count = 0;
                    if self.cur().t_type != TokenType::RParen {
                        loop { // 解析並傳入參數 (Arguments)
                            let arg = self.expression();
                            self.emit("PARAM", &arg, "-", "-");
                            count += 1;
                            if self.cur().t_type == TokenType::Comma { self.consume(); } else { break; }
                        }
                    }
                    self.expect(TokenType::RParen, "預期 ')'");
                    let t = self.new_t();
                    self.emit("CALL", &current_obj, &count.to_string(), &t); // 產生呼叫指令
                    current_obj = t;
                }
                _ => {}
            }
        }

        // 若最後帶有 `=` 則為賦值操作
        if self.cur().t_type == TokenType::Assign {
            self.consume();
            let val = self.expression();
            if path.is_empty() {
                // 單純變數賦值，例如 `a = 1`
                self.emit("STORE", &val, "-", &current_obj);
            } else {
                // 針對陣列或字典屬性賦值，例如 `a.b[c] = 1`
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

    /// 解析基本元素 (Primary)，包含數字、字串、變數、陣列字面量、字典字面量及括號號起來的子表達式
    /// EBNF: Primary = Num | StringLit | Id | ArrayLit | DictLit | "(" Expr ")" ;
    /// EBNF: ArrayLit = "[" [ Expr ( ";" Expr | { "," Expr } ) ] "]" ;
    /// EBNF: DictLit = "{" [ ( Id | StringLit ) ":" Expr { "," ( Id | StringLit ) ":" Expr } ] "}" ;
    fn primary(&mut self) -> String {
        match self.cur().t_type {
            TokenType::Num => {
                let t = self.new_t();
                self.emit("IMM", &self.cur().text.clone(), "-", &t); // 載入立即數 (Immediate)
                self.consume(); t
            }
            TokenType::StringLit => {
                let t = self.new_t();
                let pool_idx = self.string_pool.len();
                self.string_pool.push(self.cur().text.clone()); // 存入常數池
                self.emit("LOAD_STR", &pool_idx.to_string(), "-", &t);
                self.consume(); t
            }
            TokenType::Id => {
                let name = self.cur().text.clone(); // 單純的變數參考
                self.consume(); name
            }
            TokenType::LBracket => { // 陣列定義，例如 `[1, 2, 3]` 或 `[0; 5]`
                self.consume();
                let t = self.new_t();
                if self.cur().t_type == TokenType::RBracket {
                    self.emit("NEW_ARR", "-", "-", &t); // 空陣列
                } else {
                    let val = self.expression();
                    if self.cur().t_type == TokenType::Semicolon { // [INITIAL; SIZE] 語法
                        self.consume();
                        let size = self.expression();
                        self.emit("INIT_ARR", &val, &size, &t);
                    } else { // [ITEM1, ITEM2, ...] 語法
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
            TokenType::LBrace => { // 字典物件定義，例如 `{ "A": 1, B: 2 }`
                self.consume();
                let t = self.new_t();
                self.emit("NEW_DICT", "-", "-", &t);
                if self.cur().t_type != TokenType::RBrace {
                    loop {
                        let k = if self.cur().t_type == TokenType::Id {
                            let key_str = self.cur().text.clone(); // 標識符當成字串鍵
                            self.consume();
                            let k_var = self.new_t();
                            let pool_idx = self.string_pool.len();
                            self.string_pool.push(key_str);
                            self.emit("LOAD_STR", &pool_idx.to_string(), "-", &k_var);
                            k_var
                        } else if self.cur().t_type == TokenType::StringLit {
                            self.primary() // 字串字面量做為鍵
                        } else {
                            self.error("字典的鍵必須是字串或識別碼");
                        };
                        self.expect(TokenType::Colon, "預期 ':'");
                        let val = self.expression(); // 解析對應的值
                        self.emit("SET_ITEM", &t, &k, &val); // 設定字典項目
                        if self.cur().t_type == TokenType::Comma { self.consume(); } else { break; }
                    }
                }
                self.expect(TokenType::RBrace, "預期 '}'"); t
            }
            TokenType::LParen => { // 括號內的子表達式 `( expression )`
                self.consume();
                let res = self.expression();
                self.expect(TokenType::RParen, "預期 ')'"); res
            }
            _ => self.error("表達式預期外語法"),
        }
    }

    /// 解析因子 (Factor)，處理帶有綴飾的基本元素 (Primary)，例如連續的屬性存取 `.`、陣列索引 `[]`、函數呼叫 `()`
    /// EBNF: Factor = Primary { "[" Expr "]" | "." Id | "(" [ Expr { "," Expr } ] ")" } ;
    fn factor(&mut self) -> String {
        let mut res = self.primary(); // 先解析最基本的元素
        // 允許連續的綴飾，例如 a[0].b()
        while vec![TokenType::LBracket, TokenType::Dot, TokenType::LParen].contains(&self.cur().t_type) {
            match self.cur().t_type {
                TokenType::LBracket => { // 陣列索引
                    self.consume();
                    let idx = self.expression();
                    self.expect(TokenType::RBracket, "預期 ']'");
                    let t = self.new_t();
                    self.emit("GET_ITEM", &res, &idx, &t); // 產生取得物件項目指令
                    res = t;
                }
                TokenType::Dot => { // 屬性存取
                    self.consume();
                    let key_str = self.cur().text.clone();
                    self.consume();
                    let k = self.new_t();
                    let pool_idx = self.string_pool.len();
                    self.string_pool.push(key_str);
                    self.emit("LOAD_STR", &pool_idx.to_string(), "-", &k);
                    let t = self.new_t();
                    self.emit("GET_ITEM", &res, &k, &t); // 以屬性鍵取得項目
                    res = t;
                }
                TokenType::LParen => { // 函數呼叫
                    self.consume();
                    let mut count = 0;
                    if self.cur().t_type != TokenType::RParen {
                        loop { // 讀取並推入參數
                            let arg = self.expression();
                            self.emit("PARAM", &arg, "-", "-");
                            count += 1;
                            if self.cur().t_type == TokenType::Comma { self.consume(); } else { break; }
                        }
                    }
                    self.expect(TokenType::RParen, "預期 ')'");
                    let t = self.new_t();
                    self.emit("CALL", &res, &count.to_string(), &t); // 呼叫返回存入暫存變數
                    res = t;
                }
                _ => {}
            }
        }
        res
    }

    /// 解析項 (Term)，處理乘法 (`*`) 與除法 (`/`) 運算，優先級高於加減法
    /// EBNF: Term = Factor { ( "*" | "/" ) Factor } ;
    fn term(&mut self) -> String {
        let mut l = self.factor(); // 先解析最基本的因子 (Factor)
        while self.cur().t_type == TokenType::Mul || self.cur().t_type == TokenType::Div {
            let op = if self.cur().t_type == TokenType::Mul { "MUL" } else { "DIV" };
            self.consume();
            let r = self.factor();
            let t = self.new_t(); // 產生暫存變數存放結果
            self.emit(op, &l, &r, &t);
            l = t; // 更新左運算元為暫存變數
        }
        l
    }

    /// 解析算術表達式 (Arith Expr)，處理加法 (`+`) 與減法 (`-`) 運算
    /// EBNF: ArithExpr = Term { ( "+" | "-" ) Term } ;
    fn arith_expr(&mut self) -> String {
        let mut l = self.term(); // 先解析乘除法 (Term)
        while self.cur().t_type == TokenType::Plus || self.cur().t_type == TokenType::Minus {
            let op = if self.cur().t_type == TokenType::Plus { "ADD" } else { "SUB" };
            self.consume();
            let r = self.term();
            let t = self.new_t(); // 產生暫存變數存放結果
            self.emit(op, &l, &r, &t);
            l = t; // 更新左運算元為暫存變數
        }
        l
    }

    /// 解析一般表達式 (Expression)，處理關係比較運算子 (`==`, `<`, `>`)
    /// EBNF: Expr = ArithExpr [ ( "==" | "<" | ">" ) ArithExpr ] ;
    fn expression(&mut self) -> String {
        let l = self.arith_expr(); // 先解析算術運算
        if vec![TokenType::Eq, TokenType::Lt, TokenType::Gt].contains(&self.cur().t_type) {
            let op = match self.cur().t_type {
                TokenType::Eq => "CMP_EQ",
                TokenType::Lt => "CMP_LT",
                _ => "CMP_GT",
            };
            self.consume();
            let r = self.arith_expr();
            let t = self.new_t(); // 產生暫存變數存放比較結果 (布林值)
            self.emit(op, &l, &r, &t);
            return t;
        }
        l
    }

    /// 解析單一陳述句 (Statement)，包含控制流 (if/while/for) 與一般變數操作
    /// EBNF: Statement = IfStmt | WhileStmt | ForStmt | BreakStmt | ContinueStmt | ReturnStmt | ExprStmt ;
    /// EBNF: IfStmt = "if" "(" Expr ")" "{" { Statement } "}" [ "else" "{" { Statement } "}" ] ;
    /// EBNF: WhileStmt = "while" "(" Expr ")" "{" { Statement } "}" ;
    /// EBNF: ForStmt = "for" "(" [ ExprOrAssign ] ";" [ Expr ] ";" [ ExprOrAssign ] ")" "{" { Statement } "}" ;
    /// EBNF: BreakStmt = "break" ";" ;
    /// EBNF: ContinueStmt = "continue" ";" ;
    /// EBNF: ReturnStmt = "return" Expr ";" ;
    /// EBNF: ExprStmt = ExprOrAssign ";" ;
    fn statement(&mut self) {
        match self.cur().t_type {
            TokenType::If => {
                self.consume(); self.expect(TokenType::LParen, "預期 '('");
                let cond = self.expression(); // 解析 if 條件
                self.expect(TokenType::RParen, "預期 ')'"); self.expect(TokenType::LBrace, "預期 '{'");
                
                // 建立標籤，控制 if...else 的跳躍路徑
                let l_false = self.new_label();
                self.emit("JMP_F", &cond, "-", &l_false); // 若條件為假，跳至 l_false
                
                // 解析 if 區塊的內部陳述句
                while self.cur().t_type != TokenType::RBrace && self.cur().t_type != TokenType::Eof { self.statement(); }
                self.expect(TokenType::RBrace, "預期 '}'");
                
                if self.cur().t_type == TokenType::Else {
                    let l_end = self.new_label();
                    self.emit("JMP", "-", "-", &l_end);        // True 區塊執行完，跳到整個 if-else 結束點
                    self.emit("LABEL", &l_false, "-", "-");    // False 區塊起點標籤
                    
                    self.consume(); self.expect(TokenType::LBrace, "預期 '{'");
                    // 解析 else 區塊的內部陳述句
                    while self.cur().t_type != TokenType::RBrace && self.cur().t_type != TokenType::Eof { self.statement(); }
                    self.expect(TokenType::RBrace, "預期 '}'");
                    
                    self.emit("LABEL", &l_end, "-", "-");      // 整個 If-Else 結束標籤
                } else {
                    self.emit("LABEL", &l_false, "-", "-");    // 沒 Else 的話，False 直接跳到這裡
                }
            }
            TokenType::While => {
                self.consume(); self.expect(TokenType::LParen, "預期 '('");
                
                // 建立迴圈的起始標籤與結束標籤
                let l_start = self.new_label();
                let l_end = self.new_label();
                self.emit("LABEL", &l_start, "-", "-");        // 迴圈起點標籤
                
                let cond = self.expression(); // 解析 while 條件
                self.expect(TokenType::RParen, "預期 ')'"); self.expect(TokenType::LBrace, "預期 '{'");
                
                self.emit("JMP_F", &cond, "-", &l_end);        // 條件不成立跳離迴圈
                
                // 推入迴圈上下文，以供內部的 break / continue 使用
                self.loop_stack.push(LoopCtx { break_label: l_end.clone(), continue_label: l_start.clone() });
                
                // 解析 while 區塊的內部陳述句
                while self.cur().t_type != TokenType::RBrace && self.cur().t_type != TokenType::Eof { self.statement(); }
                
                self.emit("JMP", "-", "-", &l_start);          // 執行完一圈，跳回起點
                self.emit("LABEL", &l_end, "-", "-");          // 迴圈結束點標籤
                
                self.expect(TokenType::RBrace, "預期 '}'");
                self.loop_stack.pop(); // 離開迴圈，彈出上下文
            }
            TokenType::For => {
                self.consume();
                self.expect(TokenType::LParen, "預期 '('");

                // 1. for 初始化區塊 (可省略)
                if self.cur().t_type != TokenType::Semicolon {
                    self.expr_or_assign();
                }
                self.expect(TokenType::Semicolon, "預期 ';'");

                let l_cond = self.new_label();
                let l_body = self.new_label();
                let l_update = self.new_label();
                let l_end = self.new_label();

                // 2. 條件判斷區
                self.emit("LABEL", &l_cond, "-", "-");
                if self.cur().t_type != TokenType::Semicolon {
                    let cond = self.expression();
                    self.emit("JMP_F", &cond, "-", &l_end); // 條件為假則跳離
                }
                self.expect(TokenType::Semicolon, "預期 ';'");
                self.emit("JMP", "-", "-", &l_body); // 條件成立，跳至迴圈本體

                // 3. 更新區塊 (for 的 continue 會跳到這裡)
                self.emit("LABEL", &l_update, "-", "-");
                if self.cur().t_type != TokenType::RParen {
                    self.expr_or_assign();
                }
                self.expect(TokenType::RParen, "預期 ')'");
                self.emit("JMP", "-", "-", &l_cond); // 更新完畢，跳回條件判斷區

                // 4. 迴圈本體
                self.expect(TokenType::LBrace, "預期 '{'");
                self.emit("LABEL", &l_body, "-", "-");
                self.loop_stack.push(LoopCtx { break_label: l_end.clone(), continue_label: l_update.clone() }); // 推入上下文
                
                while self.cur().t_type != TokenType::RBrace && self.cur().t_type != TokenType::Eof {
                    self.statement();
                }
                self.emit("JMP", "-", "-", &l_update); // 執行完一圈，跳至更新區塊
                self.emit("LABEL", &l_end, "-", "-");
                self.expect(TokenType::RBrace, "預期 '}'");
                self.loop_stack.pop();
            }
            TokenType::Break => {
                self.consume(); self.expect(TokenType::Semicolon, "預期 ';'");
                // 取得目前所在迴圈的結束標籤
                if let Some(ctx) = self.loop_stack.last() {
                    let lbl = ctx.break_label.clone();
                    self.emit("JMP", "-", "-", &lbl);
                } else {
                    self.error("Break 必須在迴圈內使用");
                }
            }
            TokenType::Continue => {
                self.consume(); self.expect(TokenType::Semicolon, "預期 ';'");
                // 取得目前所在迴圈的下一輪起點標籤
                if let Some(ctx) = self.loop_stack.last() {
                    let lbl = ctx.continue_label.clone();
                    self.emit("JMP", "-", "-", &lbl);
                } else {
                    self.error("Continue 必須在迴圈內使用");
                }
            }
            TokenType::Return => {
                self.consume();
                let res = self.expression(); // 取得回傳值
                self.emit("RET_VAL", &res, "-", "-");
                self.expect(TokenType::Semicolon, "預期 ';'");
            }
            TokenType::Id => {
                // 如果是識別碼開頭，可能是一般賦值、函數呼叫或物件屬性操作
                self.expr_or_assign();
                self.expect(TokenType::Semicolon, "預期 ';'");
            }
            _ => self.error("無法辨識的陳述句"),
        }
    }

    /// 解析整個程式碼，程式庫的最上層是由函數宣告與全域陳述句所組成
    /// EBNF: Program = { FuncDecl | Statement } ;
    /// EBNF: FuncDecl = "func" Id "(" [ Id { "," Id } ] ")" "{" { Statement } "}" ;
    pub fn parse_program(&mut self) {
        while self.cur().t_type != TokenType::Eof {
            if self.cur().t_type == TokenType::Func {
                // 處理函數定義 (Func Declaration)
                self.consume();
                let f_name = self.cur().text.clone(); // 函數名稱
                self.consume();
                self.emit("FUNC_BEG", &f_name, "-", "-"); // 標記函數開始
                
                self.expect(TokenType::LParen, "預期 '('");
                // 解析函數形式參數
                if self.cur().t_type != TokenType::RParen {
                    loop {
                        self.emit("FORMAL", &self.cur().text.clone(), "-", "-"); // 紀錄外部傳出的參數
                        self.consume();
                        if self.cur().t_type == TokenType::Comma { self.consume(); } else { break; }
                    }
                }
                self.expect(TokenType::RParen, "預期 ')'"); self.expect(TokenType::LBrace, "預期 '{'");
                
                // 解析函數本體的陳述句
                while self.cur().t_type != TokenType::RBrace && self.cur().t_type != TokenType::Eof { self.statement(); }
                self.emit("FUNC_END", &f_name, "-", "-"); // 標記函數結束
                self.expect(TokenType::RBrace, "預期 '}'");
            } else {
                // 若非函數，則視為全域陳述句 (Global Statement)
                self.statement();
            }
        }
    }
}

// =========================================================
// 3. 程式進入點：編譯並輸出 IR 到檔案
// =========================================================

/// 程式進入點
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut debug = false;
    let mut positional: Vec<String> = Vec::new(); // 儲存位置參數 (檔案名稱)
    
    // 解析命令列參數
    for arg in args.iter().skip(1) {
        if arg == "-d" {
            debug = true; // 開啟除錯模式，印出中介碼
        } else {
            positional.push(arg.clone());
        }
    }

    // 參數不足時印出使用說明並終止
    if positional.is_empty() {
        println!("用法: {} [-d] <source_file.p0> [output_file.ir0]", args[0]);
        process::exit(1);
    }
    
    let source_file = &positional[0];
    
    // 決定輸出檔名：有提供就用提供的，沒提供就把來源檔名主檔名換副檔名為 .ir0
    let output_file = if positional.len() >= 2 {
        positional[1].clone()
    } else {
        std::path::Path::new(source_file)
            .with_extension("ir0")
            .to_string_lossy()
            .into_owned()
    };
    
    // 1. 讀取來源檔案
    let source_code = fs::read_to_string(source_file).expect("無法開啟來源檔案");
    
    // 2. 詞法分析與語法解析
    // println!("=== 開始編譯 ===");
    let lexer = Lexer::new(source_code); // 初始化詞法分析器
    let mut parser = Parser::new(lexer, debug); // 初始化語法解析器
    parser.parse_program(); // 啟動解析，生成中介碼
    
    // 3. 建立並寫入輸出檔案，使用剛剛決定好的 output_file
    let mut out_file = File::create(&output_file).expect("無法建立輸出檔案");
    
    // 4. 輸出字串池 (所有抽離出來的字串常數)
    writeln!(out_file, "===STRINGS===").unwrap();
    for s in &parser.string_pool {
        writeln!(out_file, "{:?}", s).unwrap();
    }
    
    // 5. 輸出四元組 (中間碼 IR 指令)
    writeln!(out_file, "===QUADS===").unwrap();
    for q in &parser.quads {
        writeln!(out_file, "{}\t{}\t{}\t{}", q.op, q.arg1, q.arg2, q.result).unwrap();
    }
    
    // println!("\n✅ 編譯成功！IR 已匯出至: {}", output_file);
}
