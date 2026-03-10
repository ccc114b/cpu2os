use std::collections::HashMap; // 用於建立「函數名稱」與「指令行號(PC)」的對應表
use std::env;                  // 用於讀取命令列參數 (例如執行 ./main test.p0)
use std::fs;                   // 用於讀取原始碼檔案

// =========================================================
// 1. 中間碼 (Quadruples) 資料結構
// 原理：四元組是編譯器常用的一種線性中間表示法 (IR)。
// 它將複雜的樹狀語法結構攤平，變成一連串最簡單的指令。
// 格式為：(操作碼 Op, 參數1 Arg1, 參數2 Arg2, 儲存結果 Result)
// =========================================================
#[derive(Debug, Clone)]
struct Quad {
    op: String,     // 操作碼，例如: ADD, MUL, CALL, JMP_F
    arg1: String,   // 第一個運算元（變數名稱或常數）
    arg2: String,   // 第二個運算元（變數名稱或常數）
    result: String, // 存放結果的目標變數（通常是 t1, t2 等暫存變數），或跳轉的行號
}

// =========================================================
// 2. 詞法分析器 (Lexer / Scanner)
// 原理：編譯器的第一關。負責將純文字字串（原始碼）一個個字元讀取進來，
// 剃除空白與註解，然後打包成有意義的「詞彙標記 (Token)」。
// =========================================================

// 定義所有可能的詞彙種類
#[derive(Debug, Clone, PartialEq)]
enum TokenType {
    Func, Return, If, Id, Num,                     // 關鍵字、識別碼(變數/函數名)、數字
    LParen, RParen, LBrace, RBrace, Comma, Semicolon, // 符號：() {} , ;
    Assign, Plus, Minus, Mul, Div,                 // 運算符：= + - * /
    Eq, Lt, Gt, Eof, // Dummy,                        // 邏輯符號：== < > 以及 檔案結束符號(EOF)
}

// Token 包含它的「種類」以及在原始碼中的「真實文字」
#[derive(Debug, Clone)]
struct Token {
    token_type: TokenType,
    text: String,
}

// 詞法分析器狀態機
struct Lexer<'a> {
    src: &'a [u8], // 將原始碼字串視為位元組陣列，這在 Rust 中處理英數字元效能極高
    pos: usize,    // 目前讀取到的字元位置 (游標)
}

impl<'a> Lexer<'a> {
    // 建立一個新的 Lexer
    fn new(src: &'a str) -> Self {
        Self {
            src: src.as_bytes(),
            pos: 0,
        }
    }

    // 讀取並回傳下一個 Token
    fn next_token(&mut self) -> Token {
        loop {
            // 1. 略過所有空白字元（空格、Tab、換行）
            while self.pos < self.src.len() && self.src[self.pos].is_ascii_whitespace() {
                self.pos += 1;
            }

            // 如果已經讀到檔案結尾，回傳 EOF Token
            if self.pos >= self.src.len() {
                return Token { token_type: TokenType::Eof, text: String::new() };
            }

            // 2. 處理註解 (單行 // 與多行 /* ... */)
            if self.src[self.pos] == b'/' {
                // 單行註解：//
                if self.pos + 1 < self.src.len() && self.src[self.pos + 1] == b'/' {
                    self.pos += 2; // 跳過 '//'
                    // 直到遇到換行符號才停止
                    while self.pos < self.src.len() && self.src[self.pos] != b'\n' {
                        self.pos += 1;
                    }
                    continue; // 註解略過不處理，重新開始找下一個 Token
                } 
                // 多行註解：/* ... */
                else if self.pos + 1 < self.src.len() && self.src[self.pos + 1] == b'*' {
                    self.pos += 2; // 跳過 '/*'
                    // 直到遇到 '*/' 才停止
                    while self.pos + 1 < self.src.len()
                        && !(self.src[self.pos] == b'*' && self.src[self.pos + 1] == b'/')
                    {
                        self.pos += 1;
                    }
                    if self.pos + 1 < self.src.len() {
                        self.pos += 2; // 跳過 '*/' 本身
                    }
                    continue; // 註解略過不處理，重新開始找下一個 Token
                }
            }
            break; // 如果不是空白也不是註解，跳出迴圈開始正式解析 Token
        }

        let start = self.pos; // 記錄這個 Token 的起點位置

        // 3. 辨識數字 (例如: 123, 45)
        if self.src[self.pos].is_ascii_digit() {
            while self.pos < self.src.len() && self.src[self.pos].is_ascii_digit() {
                self.pos += 1;
            }
            let text = String::from_utf8_lossy(&self.src[start..self.pos]).to_string();
            return Token { token_type: TokenType::Num, text };
        }
        // 4. 辨識英文單字 (包含變數名 ID 與 關鍵字 Keyword)
        else if self.src[self.pos].is_ascii_alphabetic() || self.src[self.pos] == b'_' {
            while self.pos < self.src.len()
                && (self.src[self.pos].is_ascii_alphanumeric() || self.src[self.pos] == b'_')
            {
                self.pos += 1;
            }
            let text = String::from_utf8_lossy(&self.src[start..self.pos]).to_string();
            // 判斷是否為保留字
            let token_type = match text.as_str() {
                "func" => TokenType::Func,
                "return" => TokenType::Return,
                "if" => TokenType::If,
                _ => TokenType::Id, // 都不是的話就是一般變數名或函數名
            };
            return Token { token_type, text };
        }
        // 5. 辨識符號與運算子
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
                    // 特別處理 '==' 與 '=' 的區別
                    if self.pos < self.src.len() && self.src[self.pos] == b'=' {
                        self.pos += 1;
                        text = "==".to_string();
                        TokenType::Eq
                    } else {
                        TokenType::Assign
                    }
                }
                _ => panic!("語法錯誤：未知的字元 '{}'", ch as char),
            };
            return Token { token_type, text };
        }
    }
}

// =========================================================
// 3. 語法解析器 (Parser) - 遞迴下降解析法 (Recursive Descent)
// 原理：根據 Token 序列，運用一組遞迴函數檢查程式文法。
// 並且在解析的過程中，同時生成「中間碼 (Quadruples)」。
// 這種作法被稱為「語法導向翻譯 (Syntax-Directed Translation)」。
// =========================================================
struct Parser<'a> {
    lexer: Lexer<'a>,   // 內含詞法分析器
    cur_token: Token,   // 當前正在檢查的 Token (Lookahead)
    quads: Vec<Quad>,   // 儲存編譯出來的所有中間碼
    t_idx: usize,       // 臨時變數 (t1, t2, t3...) 的流水號計數器
}

impl<'a> Parser<'a> {
    fn new(mut lexer: Lexer<'a>) -> Self {
        let cur_token = lexer.next_token(); // 預讀第一個 Token
        Self {
            lexer,
            cur_token,
            quads: Vec::new(),
            t_idx: 0,
        }
    }

    // 消耗當前 Token，並讀取下一個
    fn next_token(&mut self) {
        self.cur_token = self.lexer.next_token();
    }

    // 生成一個新的臨時變數名稱，例如 t1, t2... 用來儲存運算的中間結果
    fn new_t(&mut self) -> String {
        self.t_idx += 1;
        format!("t{}", self.t_idx)
    }

    // 生成一條中間碼並加入清單，同時印在畫面上方便除錯
    fn emit(&mut self, op: &str, a1: &str, a2: &str, res: &str) {
        let quad = Quad {
            op: op.to_string(),
            arg1: a1.to_string(),
            arg2: a2.to_string(),
            result: res.to_string(),
        };
        // 格式化輸出： 000: ADD        t1         t2         t3        
        println!("{:03}: {:<10} {:<10} {:<10} {:<10}", self.quads.len(), op, a1, a2, res);
        self.quads.push(quad);
    }

    // ---------------------------------------------------------
    // 以下為文法解析函數，利用函式的呼叫順序來決定運算的「優先級」
    // 優先級由高到低：factor (括號, 變數, 數字) -> term (乘除) -> arith_expr (加減) -> expression (關係比較)
    // ---------------------------------------------------------

    // 解析最小的運算單元：數字、變數、函數呼叫、括號
    fn factor(&mut self) -> String {
        let mut res = String::new();
        if self.cur_token.token_type == TokenType::Num {
            // 如果是數字，產生 IMM 指令 (立即數載入)
            res = self.new_t();
            self.emit("IMM", &self.cur_token.text.clone(), "-", &res);
            self.next_token();
        } else if self.cur_token.token_type == TokenType::Id {
            // 如果是識別碼，有可能是變數或函數呼叫
            let name = self.cur_token.text.clone();
            self.next_token();
            
            if self.cur_token.token_type == TokenType::LParen {
                // 遇到 '(' 代表是函數呼叫 e.g., add(1, 2)
                self.next_token(); // 消耗 '('
                let mut count = 0;
                // 解析傳遞進去的參數 (Arguments)
                while self.cur_token.token_type != TokenType::RParen {
                    let arg = self.expression(); // 計算參數的結果
                    self.emit("PARAM", &arg, "-", "-"); // 發送準備傳參的中間碼
                    count += 1;
                    if self.cur_token.token_type == TokenType::Comma {
                        self.next_token(); // 消耗 ','
                    }
                }
                self.next_token(); // 消耗 ')'
                res = self.new_t();
                // 產生 CALL 指令，記錄函數名稱、參數個數、以及回傳值要存到哪裡
                self.emit("CALL", &name, &count.to_string(), &res);
            } else {
                // 如果沒有 '('，那它就是單純的變數
                res = name;
            }
        } else if self.cur_token.token_type == TokenType::LParen {
            // 處理括號包起來的表達式 ( x + y )
            self.next_token(); // 消耗 '('
            res = self.expression();
            self.next_token(); // 消耗 ')'
        }
        res
    }

    // 解析乘除法 (優先級高於加減法)
    fn term(&mut self) -> String {
        let mut l = self.factor(); // 先解析左邊的單元
        // 如果遇到 * 或 /，持續解析右邊
        while self.cur_token.token_type == TokenType::Mul || self.cur_token.token_type == TokenType::Div {
            let op = if self.cur_token.token_type == TokenType::Mul { "MUL" } else { "DIV" };
            self.next_token();
            let r = self.factor(); // 解析右邊的單元
            let t = self.new_t();  // 產生新暫存變數存放結果
            self.emit(op, &l, &r, &t);
            l = t; // 將結果作為下一次運算的左運算元 (例如連乘 a * b * c)
        }
        l
    }

    // 解析加減法 (優先級低於乘除法)
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

    // 解析關係表達式 (==, <, >)
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
            t // 回傳比較結果的布林值 (存於暫存變數)
        } else {
            l // 沒有比較運算符，直接回傳算術結果
        }
    }

    // 解析陳述句 (If 判斷, 變數賦值, Return 回傳)
    fn statement(&mut self) {
        if self.cur_token.token_type == TokenType::If {
            // 解析：if ( expr ) { statements }
            self.next_token(); // 消耗 'if'
            self.next_token(); // 消耗 '('
            let cond = self.expression(); // 解析條件
            self.next_token(); // 消耗 ')'
            self.next_token(); // 消耗 '{'
            
            // 【回填技術 Backpatching】
            // 因為現在還不知道 if 區塊有多長，不知道條件不成立時要跳轉到哪一行。
            // 所以先記下 JMP_F 指令的位置，並先填入 "?"。
            let jmp_idx = self.quads.len();
            self.emit("JMP_F", &cond, "-", "?"); 
            
            // 解析 if 大括號內的陳述句
            while self.cur_token.token_type != TokenType::RBrace {
                self.statement();
            }
            self.next_token(); // 消耗 '}'
            
            // if 區塊結束了，現在知道當前行號了，把跳轉地址更新（回填）到剛剛的 JMP_F 指令中
            self.quads[jmp_idx].result = self.quads.len().to_string();
            
        } else if self.cur_token.token_type == TokenType::Id {
            // 解析：變數賦值 ( identifier = expression ; )
            let name = self.cur_token.text.clone();
            self.next_token(); // 消耗變數名
            if self.cur_token.token_type == TokenType::Assign {
                self.next_token(); // 消耗 '='
                let res = self.expression();
                self.emit("STORE", &res, "-", &name); // 將結果存入該變數
                if self.cur_token.token_type == TokenType::Semicolon {
                    self.next_token(); // 消耗 ';'
                }
            }
        } else if self.cur_token.token_type == TokenType::Return {
            // 解析：return 語句 ( return expression ; )
            self.next_token(); // 消耗 'return'
            let res = self.expression();
            self.emit("RET_VAL", &res, "-", "-"); // 將回傳值送出
            if self.cur_token.token_type == TokenType::Semicolon {
                self.next_token(); // 消耗 ';'
            }
        }
    }

    // 程式進入點：解析整個程式碼 (包含函數定義與全域語句)
    fn parse_program(&mut self) {
        while self.cur_token.token_type != TokenType::Eof {
            if self.cur_token.token_type == TokenType::Func {
                // 解析函數定義：func name ( arg1, arg2 ) { ... }
                self.next_token(); // 消耗 'func'
                let f_name = self.cur_token.text.clone();
                self.emit("FUNC_BEG", &f_name, "-", "-"); // 標記函數起點
                self.next_token(); // 消耗函數名
                self.next_token(); // 消耗 '('
                
                // 解析函數接受的參數列表 (Parameters)
                while self.cur_token.token_type == TokenType::Id {
                    self.emit("FORMAL", &self.cur_token.text.clone(), "-", "-"); // 產生接收參數的指令
                    self.next_token();
                    if self.cur_token.token_type == TokenType::Comma {
                        self.next_token(); // 消耗 ','
                    }
                }
                self.next_token(); // 消耗 ')'
                self.next_token(); // 消耗 '{'
                
                // 解析函數體
                while self.cur_token.token_type != TokenType::RBrace {
                    self.statement();
                }
                self.emit("FUNC_END", &f_name, "-", "-"); // 標記函數終點
                self.next_token(); // 消耗 '}'
            } else {
                // 如果不是函數定義，那就是直接執行的全域陳述句
                self.statement();
            }
        }
    }
}

// =========================================================
// 4. 虛擬機 (Virtual Machine)
// 原理：一個軟體模擬的 CPU。負責讀取剛才生成的 Quadruples 並真正執行運算。
// 為了支援函數呼叫與遞迴，這裡實作了「呼叫堆疊 (Call Stack)」。
// 每進入一個函數，就會產生一個「堆疊幀 (Frame)」來隔離區域變數。
// =========================================================

// 堆疊幀：儲存一個函數執行時所需的環境狀態
struct Frame {
    // 儲存區域變數 (使用 Vec 保留存入順序，方便最後按順序印出結果)
    vars: Vec<(String, i32)>, 
    ret_pc: usize,         // Return PC：函數執行完畢後，要回到母函數的哪一行指令
    ret_var: String,       // 函數回傳的結果，要塞入母函數的哪個暫存變數中
    incoming_args: Vec<i32>, // 從呼叫者 (Caller) 傳進來的參數值
    formal_idx: usize,     // 記錄目前已經把幾個傳入的參數分配給區域變數了
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

// 虛擬機主體
struct VM {
    quads: Vec<Quad>, // 虛擬機要執行的所有指令
    stack: Vec<Frame>,// 呼叫堆疊 (Call Stack)
    sp: usize,        // Stack Pointer：目前正在執行的堆疊幀索引
}

impl VM {
    fn new(quads: Vec<Quad>) -> Self {
        let mut stack = Vec::new();
        // 建立全域環境 (Global Frame, sp = 0)
        stack.push(Frame::new(0, String::new())); 
        Self {
            quads,
            stack,
            sp: 0,
        }
    }

    // 從當前堆疊幀(區域變數)中取得變數的值
    fn get_var(&self, name: &str) -> i32 {
        // 如果傳入的是純數字，直接轉型回傳
        if let Ok(val) = name.parse::<i32>() {
            return val;
        }
        // "-" 代表無此參數 (佔位符)
        if name == "-" {
            return 0;
        }
        // 尋找當前作用域的變數
        for (k, v) in &self.stack[self.sp].vars {
            if k == name {
                return *v;
            }
        }
        // 找不到預設為 0
        0
    }

    // 將值寫入當前堆疊幀(區域變數)中
    fn set_var(&mut self, name: &str, val: i32) {
        // 若變數已存在，更新其值
        for (k, v) in &mut self.stack[self.sp].vars {
            if k == name {
                *v = val;
                return;
            }
        }
        // 若變數不存在，新增至環境中
        self.stack[self.sp].vars.push((name.to_string(), val));
    }

    // 虛擬機執行主迴圈
    fn run(&mut self) {
        let mut pc = 0; // Program Counter：目前執行到的指令行號
        let mut param_stack = Vec::new(); // 準備傳遞給函數的參數暫存區
        let mut func_map = HashMap::new(); // 函數名稱與進入點行號的對照表

        // 預掃描階段 (Pre-scan)：掃描所有指令，記錄所有函數的起點位置
        for (i, q) in self.quads.iter().enumerate() {
            if q.op == "FUNC_BEG" {
                func_map.insert(q.arg1.clone(), i + 1); // 進入點為 FUNC_BEG 的下一行
            }
        }

        println!("\n=== VM 執行開始 ===");

        // 開始循序執行指令
        while pc < self.quads.len() {
            let q = self.quads[pc].clone();

            match q.op.as_str() {
                "FUNC_BEG" => {
                    // 函數宣告本身不該被「循序」執行，必須透過 CALL 指令跳轉過來。
                    // 所以如果執行流直接撞到函數宣告，代表它是宣告段落，直接跳到函數結尾。
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
                "STORE" => self.set_var(&q.result, self.get_var(&q.arg1)), // 變數賦值
                "JMP_F" => {
                    // Jump if False：如果條件判斷結果為 0 (False)，就跳轉到目標行號
                    if self.get_var(&q.arg1) == 0 {
                        // 注意：因為迴圈最後統一會有 pc += 1，所以跳轉目標要先減 1
                        pc = q.result.parse::<usize>().unwrap() - 1;
                    }
                }
                "PARAM" => {
                    // 將準備要傳給函數的參數值推入暫存棧
                    param_stack.push(self.get_var(&q.arg1));
                }
                "CALL" => {
                    // 呼叫函數，執行 Context Switch
                    let p_count: usize = q.arg2.parse().unwrap(); // 參數個數
                    let target_pc = *func_map.get(&q.arg1).expect("發生錯誤：找不到該函數");

                    // 建立新的堆疊幀，記錄返回行號與結果存放位置
                    let mut new_frame = Frame::new(pc + 1, q.result.clone());
                    
                    // 從參數暫存區把值取出來，放入新堆疊幀的傳入參數列表中
                    let start_idx = param_stack.len() - p_count;
                    for i in 0..p_count {
                        new_frame.incoming_args.push(param_stack[start_idx + i]);
                    }
                    param_stack.truncate(start_idx); // 清除暫存區已使用的參數
                    
                    self.stack.push(new_frame); // 推入 Call Stack
                    self.sp += 1;               // 切換到新的作用域
                    pc = target_pc;             // 跳轉 PC 到被呼叫的函數內部
                    continue; // 直接進入下一個指令 (避免執行底下的 pc += 1)
                }
                "FORMAL" => {
                    // 在函數內部，將傳進來的參數實際分配給指定的區域變數名稱
                    let val = self.stack[self.sp].incoming_args[self.stack[self.sp].formal_idx];
                    self.stack[self.sp].formal_idx += 1;
                    self.set_var(&q.arg1, val);
                }
                "RET_VAL" => {
                    // 處理 Return，將控制權交還給母函數 (Caller)
                    let ret_val = self.get_var(&q.arg1);          // 取得回傳結果
                    let ret_pc = self.stack[self.sp].ret_pc;      // 取得返回位址
                    let target_var = self.stack[self.sp].ret_var.clone(); // 取得應存放結果的變數名
                    
                    self.stack.pop(); // 銷毀當前函數的堆疊幀 (記憶體釋放/回收)
                    self.sp -= 1;     // 環境切換回母函數
                    
                    self.set_var(&target_var, ret_val); // 將回傳值寫入母函數的空間
                    pc = ret_pc;      // PC 恢復到母函數中呼叫點的下一行
                    continue; // 繼續執行母函數的指令
                }
                _ => {}
            }
            pc += 1; // 循序前進至下一行指令
        }

        // 程式執行結束，列印全域作用域 (sp=0) 內儲存的所有變數最終結果
        println!("=== VM 執行完畢 ===\n\n全域變數結果:");
        for (name, val) in &self.stack[0].vars {
            if !name.starts_with('t') { // 過濾掉編譯器自動產生的 t1, t2 暫存變數
                println!(">> {} = {}", name, val);
            }
        }
    }
}

// =========================================================
// 主程式 (Main Entry Point)
// 負責處理檔案讀取、管線 (Pipeline) 的串接。
// 流程：純文字 -> Lexer -> Parser (生成 IR) -> VM (執行 IR) -> 結果
// =========================================================
fn main() {
    // 讀取命令列參數
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("用法: {} <source_file>", args[0]);
        return;
    }

    // 讀取原始碼檔案內容
    let source_code = fs::read_to_string(&args[1]).unwrap_or_else(|_| {
        eprintln!("發生錯誤：無法開啟檔案 '{}'", args[1]);
        std::process::exit(1);
    });

    println!("編譯器生成的中間碼 (PC: Quadruples):");
    println!("--------------------------------------------");

    // 1. 初始化詞法分析器
    let lexer = Lexer::new(&source_code);
    
    // 2. 初始化語法分析器，並將 Lexer 傳入
    let mut parser = Parser::new(lexer);
    
    // 3. 啟動編譯過程，解析原始碼並生成中間碼 (儲存於 parser.quads)
    parser.parse_program();

    // 4. 將編譯好的中間碼傳遞給虛擬機
    let mut vm = VM::new(parser.quads);
    
    // 5. 虛擬機開始模擬執行
    vm.run();
}