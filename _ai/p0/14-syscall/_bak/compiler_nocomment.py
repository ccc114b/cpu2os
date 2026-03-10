import sys
from enum import Enum, auto

# =========================================================
# 錯誤回報工具
# =========================================================
def report_error(src: str, pos: int, msg: str):
    lines = src.split('\n')
    current_pos = 0
    line_idx = 0
    for i, l in enumerate(lines):
        if current_pos + len(l) + 1 > pos:
            line_idx = i; break
        current_pos += len(l) + 1
    col_idx = pos - current_pos
    if line_idx >= len(lines):
        line_idx = len(lines) - 1; col_idx = len(lines[line_idx])

    print(f"\n❌ [語法錯誤] 第 {line_idx + 1} 行, 第 {col_idx + 1} 字元: {msg}")
    line_str = lines[line_idx]
    print(f"  {line_str}")
    indicator = "".join(['\t' if i < len(line_str) and line_str[i] == '\t' else ' ' for i in range(col_idx)]) + "^"
    print(f"  {indicator}")
    sys.exit(1)

# =========================================================
# 1. 詞彙標記與中間碼
# =========================================================
class TokenType(Enum):
    TK_FUNC = auto(); TK_RETURN = auto(); TK_IF = auto(); TK_PRINT = auto()
    TK_WHILE = auto(); TK_FOR = auto(); TK_BREAK = auto(); TK_CONTINUE = auto() # 【新增】迴圈關鍵字
    TK_ID = auto(); TK_NUM = auto(); TK_STRING = auto()
    TK_LPAREN = auto(); TK_RPAREN = auto()
    TK_LBRACE = auto(); TK_RBRACE = auto()     
    TK_LBRACKET = auto(); TK_RBRACKET = auto() 
    TK_DOT = auto(); TK_COLON = auto()         
    TK_COMMA = auto(); TK_SEMICOLON = auto()   
    TK_ASSIGN = auto(); TK_PLUS = auto(); TK_MINUS = auto(); TK_MUL = auto(); TK_DIV = auto()
    TK_EQ = auto(); TK_LT = auto(); TK_GT = auto()
    TK_EOF = auto()

class Token:
    def __init__(self, t_type: TokenType, text: str, pos: int):
        self.type = t_type; self.text = text; self.pos = pos

class Quad:
    def __init__(self, op: str, arg1: str, arg2: str, result: str):
        self.op = op; self.arg1 = arg1; self.arg2 = arg2; self.result = result

# =========================================================
# 2. 詞法分析 (Lexer)
# =========================================================
class Lexer:
    def __init__(self, src: str):
        self.src = src; self.pos = 0; self.cur_token = None
        self.next_token()

    def next_token(self):
        while True:
            while self.pos < len(self.src) and self.src[self.pos].isspace(): self.pos += 1
            if self.pos >= len(self.src):
                self.cur_token = Token(TokenType.TK_EOF, "", self.pos); return

            if self.src[self.pos] == '/':
                if self.pos + 1 < len(self.src) and self.src[self.pos + 1] == '/':
                    self.pos += 2
                    while self.pos < len(self.src) and self.src[self.pos] != '\n': self.pos += 1
                    continue
                elif self.pos + 1 < len(self.src) and self.src[self.pos + 1] == '*':
                    self.pos += 2
                    while self.pos + 1 < len(self.src) and not (self.src[self.pos] == '*' and self.src[self.pos + 1] == '/'): self.pos += 1
                    if self.pos + 1 < len(self.src): self.pos += 2 
                    continue
            break

        start = self.pos

        if self.src[self.pos] == '"':
            self.pos += 1 
            start_str = self.pos
            while self.pos < len(self.src) and self.src[self.pos] != '"': self.pos += 1
            if self.pos >= len(self.src): report_error(self.src, start, "字串缺少結尾的雙引號 '\"'")
            text = self.src[start_str:self.pos]
            self.pos += 1 
            self.cur_token = Token(TokenType.TK_STRING, text, start)
            return

        if self.src[self.pos].isdigit():
            while self.pos < len(self.src) and self.src[self.pos].isdigit(): self.pos += 1
            self.cur_token = Token(TokenType.TK_NUM, self.src[start:self.pos], start)
            return

        if self.src[self.pos].isalpha() or self.src[self.pos] == '_':
            while self.pos < len(self.src) and (self.src[self.pos].isalnum() or self.src[self.pos] == '_'): self.pos += 1
            text = self.src[start:self.pos]
            keywords = { 
                "func": TokenType.TK_FUNC, "return": TokenType.TK_RETURN, 
                "if": TokenType.TK_IF, "print": TokenType.TK_PRINT,
                "while": TokenType.TK_WHILE, "for": TokenType.TK_FOR,
                "break": TokenType.TK_BREAK, "continue": TokenType.TK_CONTINUE # 【新增】關鍵字註冊
            }
            self.cur_token = Token(keywords.get(text, TokenType.TK_ID), text, start)
            return

        ch = self.src[self.pos]; self.pos += 1
        symbols = {
            '(': TokenType.TK_LPAREN, ')': TokenType.TK_RPAREN, '{': TokenType.TK_LBRACE, '}': TokenType.TK_RBRACE,
            '[': TokenType.TK_LBRACKET, ']': TokenType.TK_RBRACKET, '.': TokenType.TK_DOT, ':': TokenType.TK_COLON,
            '+': TokenType.TK_PLUS, '-': TokenType.TK_MINUS, '*': TokenType.TK_MUL, '/': TokenType.TK_DIV,
            ',': TokenType.TK_COMMA, ';': TokenType.TK_SEMICOLON, '<': TokenType.TK_LT, '>': TokenType.TK_GT
        }
        
        if ch in symbols: self.cur_token = Token(symbols[ch], ch, start)
        elif ch == '=':
            if self.pos < len(self.src) and self.src[self.pos] == '=':
                self.pos += 1; self.cur_token = Token(TokenType.TK_EQ, "==", start)
            else: self.cur_token = Token(TokenType.TK_ASSIGN, "=", start)
        else: report_error(self.src, start, f"無法辨識的字元: '{ch}'")

# =========================================================
# 3. 語法解析 (Parser)
# =========================================================
class Parser:
    def __init__(self, lexer: Lexer):
        self.lexer = lexer
        self.quads = []
        self.string_pool = []
        self.loop_stack =[] # 【新增】記錄當前所在的迴圈狀態 (break/continue)
        self.t_idx = 0

    @property
    def cur(self): return self.lexer.cur_token

    def consume(self): self.lexer.next_token()
        
    def error(self, msg: str):
        report_error(self.lexer.src, self.cur.pos, f"{msg} (目前讀到: '{self.cur.text}')")

    def expect(self, expected_type: TokenType, err_msg: str):
        if self.cur.type == expected_type: self.consume()
        else: self.error(err_msg)

    def new_t(self) -> str:
        self.t_idx += 1
        return f"t{self.t_idx}"

    def emit(self, op: str, a1: str, a2: str, res: str) -> int:
        idx = len(self.quads)
        self.quads.append(Quad(op, a1, a2, res))
        print(f"{idx:03d}: {op:<12} {a1:<10} {a2:<10} {res:<10}")
        return idx # 回傳該指令的行號，方便做跳轉修改

    # ================= 提取出的賦值表達式 =================
    # 【新增】抽離出來的賦值與呼叫邏輯，給 for 迴圈共用
    def expr_or_assign(self):
        name = self.cur.text; self.consume()
        obj = name; path =[]
        
        while self.cur.type in (TokenType.TK_LBRACKET, TokenType.TK_DOT, TokenType.TK_LPAREN):
            if self.cur.type == TokenType.TK_LBRACKET:
                self.consume(); idx = self.expression()
                self.expect(TokenType.TK_RBRACKET, "預期 ']'"); path.append(idx)
            elif self.cur.type == TokenType.TK_DOT:
                self.consume()
                if self.cur.type != TokenType.TK_ID: self.error("預期屬性名稱")
                key_str = self.cur.text; self.consume()
                k = self.new_t()
                pool_idx = len(self.string_pool); self.string_pool.append(key_str)
                self.emit("LOAD_STR", str(pool_idx), "-", k); path.append(k)
            elif self.cur.type == TokenType.TK_LPAREN:
                for p in path:
                    t = self.new_t(); self.emit("GET_ITEM", obj, p, t); obj = t
                path =[]; self.consume(); count = 0
                if self.cur.type != TokenType.TK_RPAREN:
                    while True:
                        arg = self.expression(); self.emit("PARAM", arg, "-", "-"); count += 1
                        if self.cur.type == TokenType.TK_COMMA: self.consume()
                        else: break
                self.expect(TokenType.TK_RPAREN, "預期 ')'")
                t = self.new_t(); self.emit("CALL", obj, str(count), t); obj = t
        
        if self.cur.type == TokenType.TK_ASSIGN:
            self.consume(); val = self.expression()
            if not path:
                self.emit("STORE", val, "-", obj)
            else:
                for p in path[:-1]:
                    t = self.new_t(); self.emit("GET_ITEM", obj, p, t); obj = t
                self.emit("SET_ITEM", obj, path[-1], val)

    # ================= 基本單元 (Primary) =================
    def primary(self) -> str:
        if self.cur.type == TokenType.TK_NUM:
            t = self.new_t(); self.emit("IMM", self.cur.text, "-", t); self.consume(); return t
        elif self.cur.type == TokenType.TK_STRING:
            t = self.new_t(); pool_idx = len(self.string_pool); self.string_pool.append(self.cur.text)
            self.emit("LOAD_STR", str(pool_idx), "-", t); self.consume(); return t
        elif self.cur.type == TokenType.TK_ID:
            name = self.cur.text; self.consume(); return name
            
        elif self.cur.type == TokenType.TK_LBRACKET:
            self.consume(); t = self.new_t(); self.emit("NEW_ARR", "-", "-", t)
            if self.cur.type != TokenType.TK_RBRACKET:
                while True:
                    val = self.expression()
                    self.emit("APPEND_ITEM", t, "-", val)
                    if self.cur.type == TokenType.TK_COMMA: self.consume()
                    else: break
            self.expect(TokenType.TK_RBRACKET, "陣列預期要有 ']' 結尾"); return t
            
        elif self.cur.type == TokenType.TK_LBRACE:
            self.consume(); t = self.new_t(); self.emit("NEW_DICT", "-", "-", t)
            if self.cur.type != TokenType.TK_RBRACE:
                while True:
                    if self.cur.type == TokenType.TK_ID:
                        key_str = self.cur.text; self.consume(); k = self.new_t()
                        pool_idx = len(self.string_pool); self.string_pool.append(key_str)
                        self.emit("LOAD_STR", str(pool_idx), "-", k)
                    elif self.cur.type == TokenType.TK_STRING: k = self.primary()
                    else: self.error("字典的鍵(Key)必須是字串或識別碼")
                    
                    self.expect(TokenType.TK_COLON, "字典預期要有 ':' 分隔鍵值")
                    val = self.expression()
                    self.emit("SET_ITEM", t, k, val)
                    if self.cur.type == TokenType.TK_COMMA: self.consume()
                    else: break
            self.expect(TokenType.TK_RBRACE, "字典預期要有 '}' 結尾"); return t
            
        elif self.cur.type == TokenType.TK_LPAREN:
            self.consume(); res = self.expression()
            self.expect(TokenType.TK_RPAREN, "括號表達式結尾預期要有 ')'"); return res
        else: self.error("表達式中出現預期外的語法結構")

    # ================= 後綴操作 (Factor) =================
    def factor(self) -> str:
        res = self.primary()
        while self.cur.type in (TokenType.TK_LBRACKET, TokenType.TK_DOT, TokenType.TK_LPAREN):
            if self.cur.type == TokenType.TK_LBRACKET:
                self.consume(); idx = self.expression(); self.expect(TokenType.TK_RBRACKET, "預期 ']'")
                t = self.new_t(); self.emit("GET_ITEM", res, idx, t); res = t
            elif self.cur.type == TokenType.TK_DOT:
                self.consume();
                if self.cur.type != TokenType.TK_ID: self.error("預期屬性名稱")
                key_str = self.cur.text; self.consume(); k = self.new_t()
                pool_idx = len(self.string_pool); self.string_pool.append(key_str)
                self.emit("LOAD_STR", str(pool_idx), "-", k)
                t = self.new_t(); self.emit("GET_ITEM", res, k, t); res = t
            elif self.cur.type == TokenType.TK_LPAREN:
                self.consume(); count = 0
                if self.cur.type != TokenType.TK_RPAREN:
                    while True:
                        arg = self.expression(); self.emit("PARAM", arg, "-", "-"); count += 1
                        if self.cur.type == TokenType.TK_COMMA: self.consume()
                        else: break
                self.expect(TokenType.TK_RPAREN, "預期 ')'")
                t = self.new_t(); self.emit("CALL", res, str(count), t); res = t
        return res

    def term(self) -> str:
        l = self.factor()
        while self.cur.type in (TokenType.TK_MUL, TokenType.TK_DIV):
            op = "MUL" if self.cur.type == TokenType.TK_MUL else "DIV"
            self.consume(); r = self.factor(); t = self.new_t()
            self.emit(op, l, r, t); l = t
        return l

    def arith_expr(self) -> str:
        l = self.term()
        while self.cur.type in (TokenType.TK_PLUS, TokenType.TK_MINUS):
            op = "ADD" if self.cur.type == TokenType.TK_PLUS else "SUB"
            self.consume(); r = self.term(); t = self.new_t()
            self.emit(op, l, r, t); l = t
        return l

    def expression(self) -> str:
        l = self.arith_expr()
        if self.cur.type in (TokenType.TK_EQ, TokenType.TK_LT, TokenType.TK_GT):
            op = "CMP_EQ" if self.cur.type == TokenType.TK_EQ else "CMP_LT" if self.cur.type == TokenType.TK_LT else "CMP_GT"
            self.consume(); r = self.arith_expr(); t = self.new_t()
            self.emit(op, l, r, t); return t
        return l

    # ================= 陳述句 (Statement) =================
    def statement(self):
        if self.cur.type == TokenType.TK_IF:
            self.consume(); self.expect(TokenType.TK_LPAREN, "預期 '('")
            cond = self.expression()
            self.expect(TokenType.TK_RPAREN, "預期 ')'"); self.expect(TokenType.TK_LBRACE, "預期 '{'")
            
            jmp_f_idx = self.emit("JMP_F", cond, "-", "?")
            while self.cur.type not in (TokenType.TK_RBRACE, TokenType.TK_EOF): self.statement()
            self.expect(TokenType.TK_RBRACE, "預期 '}'")
            
            self.quads[jmp_f_idx].result = str(len(self.quads))
            
        # 【新增】WHILE 迴圈處理
        elif self.cur.type == TokenType.TK_WHILE:
            self.consume(); self.expect(TokenType.TK_LPAREN, "預期 '('")
            
            cond_idx = len(self.quads) # 紀錄每次開始檢查條件的位置
            cond = self.expression()
            
            self.expect(TokenType.TK_RPAREN, "預期 ')'"); self.expect(TokenType.TK_LBRACE, "預期 '{'")
            
            jmp_f_idx = self.emit("JMP_F", cond, "-", "?")
            
            # 建立迴圈 Context
            self.loop_stack.append({'break':[], 'continue': cond_idx})
            
            while self.cur.type not in (TokenType.TK_RBRACE, TokenType.TK_EOF): self.statement()
            
            self.emit("JMP", "-", "-", str(cond_idx)) # 執行完一圈跳回條件
            self.expect(TokenType.TK_RBRACE, "預期 '}'")
            
            # 回填離開迴圈的位址
            end_idx = len(self.quads)
            self.quads[jmp_f_idx].result = str(end_idx)
            
            # 處理這個迴圈內的所有 break
            loop_ctx = self.loop_stack.pop()
            for b_idx in loop_ctx['break']:
                self.quads[b_idx].result = str(end_idx)
                
        # 【新增】FOR 迴圈處理與指令編織 (Instruction Weaving)
        elif self.cur.type == TokenType.TK_FOR:
            self.consume(); self.expect(TokenType.TK_LPAREN, "預期 '('")
            
            # 1. 執行 Init (初始化)
            if self.cur.type != TokenType.TK_SEMICOLON: self.expr_or_assign()
            self.expect(TokenType.TK_SEMICOLON, "預期 ';'")
            
            # 2. 條件檢查點 (Condition)
            cond_idx = len(self.quads)
            if self.cur.type != TokenType.TK_SEMICOLON:
                cond = self.expression()
            else:
                cond = self.new_t(); self.emit("IMM", "1", "-", cond) # 如果沒有條件，預設為 True
            
            jmp_f_idx = self.emit("JMP_F", cond, "-", "?")
            jmp_body_idx = self.emit("JMP", "-", "-", "?") # 因為下面要放 step，所以要跨過去
            
            self.expect(TokenType.TK_SEMICOLON, "預期 ';'")
            
            # 3. 步進點 (Step)
            step_idx = len(self.quads)
            if self.cur.type != TokenType.TK_RPAREN: self.expr_or_assign()
            self.emit("JMP", "-", "-", str(cond_idx)) # 步進做完要回去檢查條件
            
            self.expect(TokenType.TK_RPAREN, "預期 ')'"); self.expect(TokenType.TK_LBRACE, "預期 '{'")
            
            # 4. 迴圈實體區塊 (Body)
            self.quads[jmp_body_idx].result = str(len(self.quads)) # 讓 JMP_BODY 跳到這裡
            self.loop_stack.append({'break':[], 'continue': step_idx})
            
            while self.cur.type not in (TokenType.TK_RBRACE, TokenType.TK_EOF): self.statement()
                
            self.emit("JMP", "-", "-", str(step_idx)) # 區塊做完去執行步進
            self.expect(TokenType.TK_RBRACE, "預期 '}'")
            
            # 5. 回填離開迴圈位址與 Break
            end_idx = len(self.quads)
            self.quads[jmp_f_idx].result = str(end_idx)
            
            loop_ctx = self.loop_stack.pop()
            for b_idx in loop_ctx['break']:
                self.quads[b_idx].result = str(end_idx)

        # 【新增】BREAK 與 CONTINUE 處理
        elif self.cur.type == TokenType.TK_BREAK:
            self.consume()
            if not self.loop_stack: self.error("break 必須在迴圈內部使用")
            b_idx = self.emit("JMP", "-", "-", "?")
            self.loop_stack[-1]['break'].append(b_idx) # 記錄下來等迴圈結束回填
            self.expect(TokenType.TK_SEMICOLON, "預期 ';'")
            
        elif self.cur.type == TokenType.TK_CONTINUE:
            self.consume()
            if not self.loop_stack: self.error("continue 必須在迴圈內部使用")
            c_target = self.loop_stack[-1]['continue']
            self.emit("JMP", "-", "-", str(c_target)) # 往上跳到迴圈目標
            self.expect(TokenType.TK_SEMICOLON, "預期 ';'")

        # 一般表達式陳述
        elif self.cur.type == TokenType.TK_ID:
            self.expr_or_assign()
            self.expect(TokenType.TK_SEMICOLON, "預期 ';'")
                
        elif self.cur.type == TokenType.TK_RETURN:
            self.consume(); res = self.expression(); self.emit("RET_VAL", res, "-", "-")
            self.expect(TokenType.TK_SEMICOLON, "預期 ';'")
            
        elif self.cur.type == TokenType.TK_PRINT:
            self.consume(); self.expect(TokenType.TK_LPAREN, "預期 '('")
            if self.cur.type != TokenType.TK_RPAREN:
                while True:
                    val = self.expression(); self.emit("PRINT_VAL", val, "-", "-")
                    if self.cur.type == TokenType.TK_COMMA: self.consume()
                    else: break
            self.emit("PRINT_NL", "-", "-", "-") 
            self.expect(TokenType.TK_RPAREN, "預期 ')'")
            self.expect(TokenType.TK_SEMICOLON, "預期 ';'")
        else:
            self.error("無法辨識的陳述句或語法結構")

    def parse_program(self):
        while self.cur.type != TokenType.TK_EOF:
            if self.cur.type == TokenType.TK_FUNC:
                self.consume()
                if self.cur.type != TokenType.TK_ID: self.error("預期函數名稱")
                f_name = self.cur.text; self.consume()
                self.emit("FUNC_BEG", f_name, "-", "-")
                self.expect(TokenType.TK_LPAREN, "預期 '('")
                
                if self.cur.type != TokenType.TK_RPAREN:
                    while True:
                        if self.cur.type != TokenType.TK_ID: self.error("預期變數名稱")
                        self.emit("FORMAL", self.cur.text, "-", "-"); self.consume()
                        if self.cur.type == TokenType.TK_COMMA: self.consume()
                        else: break
                            
                self.expect(TokenType.TK_RPAREN, "預期 ')'"); self.expect(TokenType.TK_LBRACE, "預期 '{'")
                while self.cur.type not in (TokenType.TK_RBRACE, TokenType.TK_EOF): self.statement()
                self.emit("FUNC_END", f_name, "-", "-")
                self.expect(TokenType.TK_RBRACE, "預期 '}'")
            else:
                self.statement()

# =========================================================
# 4. 虛擬機 (Virtual Machine)
# =========================================================
class Frame:
    def __init__(self, ret_pc: int = 0, ret_var: str = ""):
        self.vars = {}
        self.ret_pc = ret_pc
        self.ret_var = ret_var
        self.incoming_args =[]
        self.formal_idx = 0

class VM:
    def __init__(self, quads: list, string_pool: list):
        self.quads = quads; self.string_pool = string_pool
        self.stack = [Frame()]; self.sp = 0; self.print_buf =[]

    def get_var(self, name: str):
        if name.isdigit() or (name.startswith('-') and name[1:].isdigit()): return int(name)
        if name == "-": return 0
        return self.stack[self.sp].vars.get(name, 0)

    def set_var(self, name: str, val):
        self.stack[self.sp].vars[name] = val

    def run(self):
        pc = 0; param_stack =[]
        func_map = {q.arg1: i + 1 for i, q in enumerate(self.quads) if q.op == "FUNC_BEG"}

        print("\n=== VM 執行開始 ===")
        while pc < len(self.quads):
            q = self.quads[pc]
            try:
                if q.op == "FUNC_BEG":
                    while self.quads[pc].op != "FUNC_END": pc += 1
                
                elif q.op == "IMM": self.set_var(q.result, int(q.arg1))
                elif q.op == "LOAD_STR": self.set_var(q.result, self.string_pool[int(q.arg1)])
                elif q.op == "ADD": self.set_var(q.result, self.get_var(q.arg1) + self.get_var(q.arg2))
                elif q.op == "SUB": self.set_var(q.result, self.get_var(q.arg1) - self.get_var(q.arg2))
                elif q.op == "MUL": self.set_var(q.result, self.get_var(q.arg1) * self.get_var(q.arg2))
                elif q.op == "DIV": self.set_var(q.result, self.get_var(q.arg1) // max(self.get_var(q.arg2), 1))
                elif q.op == "CMP_EQ": self.set_var(q.result, 1 if self.get_var(q.arg1) == self.get_var(q.arg2) else 0)
                elif q.op == "CMP_LT": self.set_var(q.result, 1 if self.get_var(q.arg1) < self.get_var(q.arg2) else 0)
                elif q.op == "CMP_GT": self.set_var(q.result, 1 if self.get_var(q.arg1) > self.get_var(q.arg2) else 0)
                elif q.op == "STORE": self.set_var(q.result, self.get_var(q.arg1))
                
                elif q.op == "NEW_ARR": self.set_var(q.result,[])
                elif q.op == "NEW_DICT": self.set_var(q.result, {})
                
                # ✅ 這裡已經修復了原本 q.arg2 抓錯的 Bug！
                elif q.op == "APPEND_ITEM": self.get_var(q.arg1).append(self.get_var(q.result))
                
                elif q.op == "SET_ITEM": self.get_var(q.arg1)[self.get_var(q.arg2)] = self.get_var(q.result)
                elif q.op == "GET_ITEM": self.set_var(q.result, self.get_var(q.arg1)[self.get_var(q.arg2)])
                
                # 【新增】無條件跳轉 (用於 for 迴圈編織、break 與 continue)
                elif q.op == "JMP":
                    pc = int(q.result) - 1
                elif q.op == "JMP_F":
                    if self.get_var(q.arg1) == 0: pc = int(q.result) - 1
                    
                elif q.op == "PRINT_VAL": self.print_buf.append(str(self.get_var(q.arg1)))
                elif q.op == "PRINT_NL":
                    print("[程式輸出] >> " + " ".join(self.print_buf))
                    self.print_buf =[]
                
                elif q.op == "PARAM": param_stack.append(self.get_var(q.arg1))
                elif q.op == "CALL":
                    p_count = int(q.arg2)
                    f_name = self.get_var(q.arg1) if isinstance(self.get_var(q.arg1), str) else q.arg1
                    target_pc = func_map.get(f_name)
                    if target_pc is None: raise Exception(f"找不到函數 '{f_name}'")
                    
                    new_frame = Frame(ret_pc=pc + 1, ret_var=q.result)
                    if p_count > 0:
                        new_frame.incoming_args = param_stack[-p_count:]
                        del param_stack[-p_count:]
                    self.stack.append(new_frame); self.sp += 1; pc = target_pc; continue
                elif q.op == "FORMAL":
                    frame = self.stack[self.sp]
                    self.set_var(q.arg1, frame.incoming_args[frame.formal_idx]); frame.formal_idx += 1
                elif q.op == "RET_VAL":
                    ret_val = self.get_var(q.arg1); ret_address = self.stack[self.sp].ret_pc; target_var = self.stack[self.sp].ret_var
                    self.stack.pop(); self.sp -= 1
                    self.set_var(target_var, ret_val); pc = ret_address; continue
                elif q.op == "FUNC_END":
                    if self.sp > 0:
                        ret_address = self.stack[self.sp].ret_pc; target_var = self.stack[self.sp].ret_var
                        self.stack.pop(); self.sp -= 1
                        self.set_var(target_var, 0); pc = ret_address; continue
                        
            except Exception as e:
                print(f"\n[VM 執行時期錯誤] 發生在指令列 {pc:03d} ({q.op}): {e}")
                sys.exit(1)
                
            pc += 1

        print("=== VM 執行完畢 ===\n\n記憶體狀態 (全域變數):")
        for name, val in self.stack[0].vars.items():
            if not name.startswith('t'): print(f"[{name}] = {val}")

# =========================================================
# 讀取檔案與主程式
# =========================================================
def main():
    if len(sys.argv) < 2: print(f"用法: python {sys.argv[0]} <source_file>"); sys.exit(1)
    try:
        with open(sys.argv[1], 'r', encoding='utf-8') as f: source_code = f.read()
    except Exception as e: print(f"無法開啟檔案: {e}"); sys.exit(1)

    print("編譯器生成的中間碼 (PC: Quadruples):")
    print("-" * 44)
    lexer = Lexer(source_code); parser = Parser(lexer)
    parser.parse_program()
    vm = VM(parser.quads, parser.string_pool)
    vm.run()

if __name__ == "__main__":
    main()