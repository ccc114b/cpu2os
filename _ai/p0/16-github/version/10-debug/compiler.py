import sys
from enum import Enum, auto

# =========================================================
# 錯誤回報工具 (Error Reporter)
# 原理：將 1D 的字元索引 (pos) 轉換為 2D 的行號與欄位，
# 並提取出錯誤發生的那一整行，畫出箭頭 (^) 指示位置。
# =========================================================
def report_error(src: str, pos: int, msg: str):
    lines = src.split('\n')
    current_pos = 0
    line_idx = 0
    
    # 計算該 pos 屬於第幾行
    for i, l in enumerate(lines):
        # +1 是為了把被 split 吃掉的 '\n' 補回來計算
        if current_pos + len(l) + 1 > pos:
            line_idx = i
            break
        current_pos += len(l) + 1
    
    col_idx = pos - current_pos # 該行的第幾個字元
    
    # 避免 index 超出範圍 (例如剛好在檔案結尾)
    if line_idx >= len(lines):
        line_idx = len(lines) - 1
        col_idx = len(lines[line_idx])

    print(f"\n❌ [語法錯誤] 第 {line_idx + 1} 行, 第 {col_idx + 1} 字元: {msg}")
    
    # 印出錯誤的那一行
    line_str = lines[line_idx]
    print(f"  {line_str}")
    
    # 畫出箭頭 (考慮到 \t tab 字元造成的對齊偏移，用相同的空白格式填充)
    indicator = ""
    for i in range(col_idx):
        if i < len(line_str) and line_str[i] == '\t':
            indicator += '\t'
        else:
            indicator += ' '
    indicator += "^"
    
    print(f"  {indicator}")
    sys.exit(1) # 發生語法錯誤，立即中止編譯

# =========================================================
# 1. 中間碼與 Tokens 結構
# =========================================================
class TokenType(Enum):
    TK_FUNC = auto(); TK_RETURN = auto(); TK_IF = auto(); TK_PRINT = auto(); 
    TK_ID = auto(); TK_NUM = auto(); TK_STRING = auto()
    TK_LPAREN = auto(); TK_RPAREN = auto(); TK_LBRACE = auto(); TK_RBRACE = auto()
    TK_COMMA = auto(); TK_SEMICOLON = auto()
    TK_ASSIGN = auto(); TK_PLUS = auto(); TK_MINUS = auto(); TK_MUL = auto(); TK_DIV = auto()
    TK_EQ = auto(); TK_LT = auto(); TK_GT = auto()
    TK_EOF = auto()

class Token:
    # 【修改】新增 pos 參數，紀錄自己在原始碼的位置
    def __init__(self, t_type: TokenType, text: str, pos: int):
        self.type = t_type
        self.text = text
        self.pos = pos

class Quad:
    def __init__(self, op: str, arg1: str, arg2: str, result: str):
        self.op = op; self.arg1 = arg1; self.arg2 = arg2; self.result = result

# =========================================================
# 2. 詞法分析 (Lexer)
# =========================================================
class Lexer:
    def __init__(self, src: str):
        self.src = src
        self.pos = 0
        self.cur_token = None
        self.next_token()

    def next_token(self):
        while True:
            while self.pos < len(self.src) and self.src[self.pos].isspace():
                self.pos += 1
            
            if self.pos >= len(self.src):
                self.cur_token = Token(TokenType.TK_EOF, "", self.pos)
                return

            if self.src[self.pos] == '/':
                if self.pos + 1 < len(self.src) and self.src[self.pos + 1] == '/':
                    self.pos += 2
                    while self.pos < len(self.src) and self.src[self.pos] != '\n':
                        self.pos += 1
                    continue
                elif self.pos + 1 < len(self.src) and self.src[self.pos + 1] == '*':
                    self.pos += 2
                    while self.pos + 1 < len(self.src) and not (self.src[self.pos] == '*' and self.src[self.pos + 1] == '/'):
                        self.pos += 1
                    if self.pos + 1 < len(self.src):
                        self.pos += 2 
                    continue
            break

        start = self.pos # 記下這個 Token 在原始碼中的起點

        if self.src[self.pos] == '"':
            self.pos += 1 
            start_str = self.pos
            while self.pos < len(self.src) and self.src[self.pos] != '"':
                self.pos += 1
                
            if self.pos >= len(self.src): # 【語法錯誤防護】缺少雙引號
                report_error(self.src, start, "字串缺少結尾的雙引號 '\"'")
                
            text = self.src[start_str:self.pos]
            self.pos += 1 
            self.cur_token = Token(TokenType.TK_STRING, text, start)
            return

        if self.src[self.pos].isdigit():
            while self.pos < len(self.src) and self.src[self.pos].isdigit():
                self.pos += 1
            self.cur_token = Token(TokenType.TK_NUM, self.src[start:self.pos], start)
            return

        if self.src[self.pos].isalpha() or self.src[self.pos] == '_':
            while self.pos < len(self.src) and (self.src[self.pos].isalnum() or self.src[self.pos] == '_'):
                self.pos += 1
            text = self.src[start:self.pos]
            keywords = {
                "func": TokenType.TK_FUNC, "return": TokenType.TK_RETURN,
                "if": TokenType.TK_IF, "print": TokenType.TK_PRINT
            }
            self.cur_token = Token(keywords.get(text, TokenType.TK_ID), text, start)
            return

        ch = self.src[self.pos]
        self.pos += 1
        symbols = {
            '(': TokenType.TK_LPAREN, ')': TokenType.TK_RPAREN, '{': TokenType.TK_LBRACE, '}': TokenType.TK_RBRACE,
            '+': TokenType.TK_PLUS, '-': TokenType.TK_MINUS, '*': TokenType.TK_MUL, '/': TokenType.TK_DIV,
            ',': TokenType.TK_COMMA, ';': TokenType.TK_SEMICOLON, '<': TokenType.TK_LT, '>': TokenType.TK_GT
        }
        
        if ch in symbols:
            self.cur_token = Token(symbols[ch], ch, start)
        elif ch == '=':
            if self.pos < len(self.src) and self.src[self.pos] == '=':
                self.pos += 1
                self.cur_token = Token(TokenType.TK_EQ, "==", start)
            else:
                self.cur_token = Token(TokenType.TK_ASSIGN, "=", start)
        else:
            # 【語法錯誤防護】不認識的特殊符號
            report_error(self.src, start, f"無法辨識的字元: '{ch}'")

# =========================================================
# 3. 語法解析 (Parser) + 嚴謹的錯誤捕捉
# =========================================================
class Parser:
    def __init__(self, lexer: Lexer):
        self.lexer = lexer
        self.quads =[]
        self.string_pool =[]
        self.t_idx = 0

    @property
    def cur(self):
        return self.lexer.cur_token

    def consume(self):
        self.lexer.next_token()
        
    # 【新增】主動觸發語法錯誤
    def error(self, msg: str):
        token_text = self.cur.text if self.cur.type != TokenType.TK_EOF else "檔案結尾 (EOF)"
        report_error(self.lexer.src, self.cur.pos, f"{msg} (目前讀到: '{token_text}')")

    # 【新增】嚴格預期下一個 Token 應該是什麼，否則噴錯
    def expect(self, expected_type: TokenType, err_msg: str):
        if self.cur.type == expected_type:
            self.consume()
        else:
            self.error(err_msg)

    def new_t(self) -> str:
        self.t_idx += 1
        return f"t{self.t_idx}"

    def emit(self, op: str, a1: str, a2: str, res: str):
        self.quads.append(Quad(op, a1, a2, res))
        print(f"{len(self.quads)-1:03d}: {op:<10} {a1:<10} {a2:<10} {res:<10}")

    def factor(self) -> str:
        res = ""
        if self.cur.type == TokenType.TK_NUM:
            res = self.new_t()
            self.emit("IMM", self.cur.text, "-", res)
            self.consume()
        elif self.cur.type == TokenType.TK_ID:
            name = self.cur.text
            self.consume()
            if self.cur.type == TokenType.TK_LPAREN: # 函數呼叫
                self.consume()
                count = 0
                if self.cur.type != TokenType.TK_RPAREN:
                    while True:
                        arg = self.expression()
                        self.emit("PARAM", arg, "-", "-")
                        count += 1
                        if self.cur.type == TokenType.TK_COMMA:
                            self.consume()
                        else:
                            break
                self.expect(TokenType.TK_RPAREN, "函數呼叫結尾預期要有 ')'")
                res = self.new_t()
                self.emit("CALL", name, str(count), res)
            else:
                res = name
        elif self.cur.type == TokenType.TK_LPAREN:
            self.consume()
            res = self.expression()
            self.expect(TokenType.TK_RPAREN, "括號表達式結尾預期要有 ')'")
        else:
            # 【錯誤防護】如果這不是合法的表達式開頭
            self.error("表達式中出現預期外的語法結構")
        return res

    def term(self) -> str:
        l = self.factor()
        while self.cur.type in (TokenType.TK_MUL, TokenType.TK_DIV):
            op = "MUL" if self.cur.type == TokenType.TK_MUL else "DIV"
            self.consume()
            r = self.factor()
            t = self.new_t()
            self.emit(op, l, r, t)
            l = t
        return l

    def arith_expr(self) -> str:
        l = self.term()
        while self.cur.type in (TokenType.TK_PLUS, TokenType.TK_MINUS):
            op = "ADD" if self.cur.type == TokenType.TK_PLUS else "SUB"
            self.consume()
            r = self.term()
            t = self.new_t()
            self.emit(op, l, r, t)
            l = t
        return l

    def expression(self) -> str:
        l = self.arith_expr()
        if self.cur.type in (TokenType.TK_EQ, TokenType.TK_LT, TokenType.TK_GT):
            if self.cur.type == TokenType.TK_EQ: op = "CMP_EQ"
            elif self.cur.type == TokenType.TK_LT: op = "CMP_LT"
            else: op = "CMP_GT"
            self.consume()
            r = self.arith_expr()
            t = self.new_t()
            self.emit(op, l, r, t)
            return t
        return l

    def statement(self):
        if self.cur.type == TokenType.TK_IF:
            self.consume()
            self.expect(TokenType.TK_LPAREN, "if 判斷式後面預期要有 '('")
            cond = self.expression()
            self.expect(TokenType.TK_RPAREN, "條件判斷式後面預期要有 ')'")
            self.expect(TokenType.TK_LBRACE, "預期要有 '{' 來開啟 if 區塊")
            
            jmp_idx = len(self.quads)
            self.emit("JMP_F", cond, "-", "?")
            
            while self.cur.type != TokenType.TK_RBRACE and self.cur.type != TokenType.TK_EOF:
                self.statement()
                
            self.expect(TokenType.TK_RBRACE, "預期要有 '}' 來結束 if 區塊")
            self.quads[jmp_idx].result = str(len(self.quads))
            
        elif self.cur.type == TokenType.TK_ID:
            name = self.cur.text
            self.consume()
            if self.cur.type == TokenType.TK_ASSIGN:
                self.consume()
                res = self.expression()
                self.emit("STORE", res, "-", name)
                self.expect(TokenType.TK_SEMICOLON, "變數賦值結尾預期要有 ';'")
            else:
                self.error("變數名稱後預期要有 '=' (不支援的變數操作)")
                
        elif self.cur.type == TokenType.TK_RETURN:
            self.consume()
            res = self.expression()
            self.emit("RET_VAL", res, "-", "-")
            self.expect(TokenType.TK_SEMICOLON, "return 語句結尾預期要有 ';'")
            
        elif self.cur.type == TokenType.TK_PRINT:
            self.consume() 
            self.expect(TokenType.TK_LPAREN, "print 後面預期要有 '('")
            
            if self.cur.type != TokenType.TK_RPAREN:
                while True:
                    if self.cur.type == TokenType.TK_STRING:
                        str_idx = len(self.string_pool)
                        self.string_pool.append(self.cur.text)
                        self.emit("PRINT_STR", str(str_idx), "-", "-")
                        self.consume()
                    else:
                        res = self.expression()
                        self.emit("PRINT_VAL", res, "-", "-")
                    
                    if self.cur.type == TokenType.TK_COMMA:
                        self.consume()
                    else:
                        break
                        
            self.emit("PRINT_NL", "-", "-", "-") 
            self.expect(TokenType.TK_RPAREN, "print 參數結尾預期要有 ')'")
            self.expect(TokenType.TK_SEMICOLON, "print 語句結尾預期要有 ';'")
        else:
            self.error("無法辨識的陳述句或語法結構")

    def parse_program(self):
        while self.cur.type != TokenType.TK_EOF:
            if self.cur.type == TokenType.TK_FUNC:
                self.consume()
                
                if self.cur.type != TokenType.TK_ID:
                    self.error("func 關鍵字後預期要有函數名稱")
                f_name = self.cur.text
                self.consume()
                
                self.expect(TokenType.TK_LPAREN, "函數名稱後預期要有 '('")
                
                if self.cur.type != TokenType.TK_RPAREN:
                    while True:
                        if self.cur.type != TokenType.TK_ID:
                            self.error("參數列表內預期要有變數名稱")
                        self.emit("FORMAL", self.cur.text, "-", "-")
                        self.consume()
                        if self.cur.type == TokenType.TK_COMMA:
                            self.consume()
                        else:
                            break
                            
                self.expect(TokenType.TK_RPAREN, "參數列表結尾預期要有 ')'")
                self.expect(TokenType.TK_LBRACE, "預期要有 '{' 來開啟函數區塊")
                
                while self.cur.type != TokenType.TK_RBRACE and self.cur.type != TokenType.TK_EOF:
                    self.statement()
                    
                self.emit("FUNC_END", f_name, "-", "-")
                self.expect(TokenType.TK_RBRACE, "預期要有 '}' 來結束函數區塊")
            else:
                self.statement()

# =========================================================
# 4. 虛擬機 (Virtual Machine) -> (保持與上一版相同)
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
        self.quads = quads
        self.string_pool = string_pool
        self.stack = [Frame()]
        self.sp = 0
        self.print_buf =[]

    def get_var(self, name: str) -> int:
        if name.isdigit() or (name.startswith('-') and name[1:].isdigit()): return int(name)
        if name == "-": return 0
        return self.stack[self.sp].vars.get(name, 0)

    def set_var(self, name: str, val: int):
        self.stack[self.sp].vars[name] = val

    def run(self):
        pc = 0; param_stack =[]
        func_map = {q.arg1: i + 1 for i, q in enumerate(self.quads) if q.op == "FUNC_BEG"}

        print("\n=== VM 執行開始 ===")
        while pc < len(self.quads):
            q = self.quads[pc]
            if q.op == "FUNC_BEG":
                while self.quads[pc].op != "FUNC_END": pc += 1
            elif q.op == "IMM": self.set_var(q.result, int(q.arg1))
            elif q.op == "ADD": self.set_var(q.result, self.get_var(q.arg1) + self.get_var(q.arg2))
            elif q.op == "SUB": self.set_var(q.result, self.get_var(q.arg1) - self.get_var(q.arg2))
            elif q.op == "MUL": self.set_var(q.result, self.get_var(q.arg1) * self.get_var(q.arg2))
            elif q.op == "DIV": self.set_var(q.result, self.get_var(q.arg1) // max(self.get_var(q.arg2), 1))
            elif q.op == "CMP_EQ": self.set_var(q.result, 1 if self.get_var(q.arg1) == self.get_var(q.arg2) else 0)
            elif q.op == "CMP_LT": self.set_var(q.result, 1 if self.get_var(q.arg1) < self.get_var(q.arg2) else 0)
            elif q.op == "CMP_GT": self.set_var(q.result, 1 if self.get_var(q.arg1) > self.get_var(q.arg2) else 0)
            elif q.op == "STORE": self.set_var(q.result, self.get_var(q.arg1))
            elif q.op == "JMP_F":
                if self.get_var(q.arg1) == 0: pc = int(q.result) - 1
            elif q.op == "PRINT_STR": self.print_buf.append(self.string_pool[int(q.arg1)])
            elif q.op == "PRINT_VAL": self.print_buf.append(str(self.get_var(q.arg1)))
            elif q.op == "PRINT_NL":
                print("[程式輸出] >> " + " ".join(self.print_buf))
                self.print_buf =[]
            elif q.op == "PARAM": param_stack.append(self.get_var(q.arg1))
            elif q.op == "CALL":
                p_count = int(q.arg2); target_pc = func_map[q.arg1]
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
            pc += 1

        print("=== VM 執行完畢 ===\n\n記憶體狀態 (全域變數):")
        for name, val in self.stack[0].vars.items():
            if not name.startswith('t'): print(f"[{name}] = {val}")

# =========================================================
# 讀取檔案與主程式
# =========================================================
def main():
    if len(sys.argv) < 2:
        print(f"用法: python {sys.argv[0]} <source_file>")
        sys.exit(1)

    try:
        with open(sys.argv[1], 'r', encoding='utf-8') as f:
            source_code = f.read()
    except Exception as e:
        print(f"無法開啟檔案: {e}")
        sys.exit(1)

    print("編譯器生成的中間碼 (PC: Quadruples):")
    print("-" * 44)
    
    lexer = Lexer(source_code)
    parser = Parser(lexer)
    parser.parse_program()
    
    vm = VM(parser.quads, parser.string_pool)
    vm.run()

if __name__ == "__main__":
    main()