import sys
from enum import Enum, auto

# =========================================================
# 1. 中間碼 (Quadruples) 與 詞彙標記 (Tokens) 定義
# =========================================================

class TokenType(Enum):
    TK_FUNC = auto(); TK_RETURN = auto(); TK_IF = auto(); TK_ID = auto(); TK_NUM = auto()
    TK_LPAREN = auto(); TK_RPAREN = auto(); TK_LBRACE = auto(); TK_RBRACE = auto()
    TK_COMMA = auto(); TK_SEMICOLON = auto()
    TK_ASSIGN = auto(); TK_PLUS = auto(); TK_MINUS = auto(); TK_MUL = auto(); TK_DIV = auto()
    TK_EQ = auto(); TK_LT = auto(); TK_GT = auto()
    TK_EOF = auto()

class Token:
    def __init__(self, t_type: TokenType, text: str):
        self.type = t_type
        self.text = text

class Quad:
    def __init__(self, op: str, arg1: str, arg2: str, result: str):
        self.op = op
        self.arg1 = arg1
        self.arg2 = arg2
        self.result = result

# =========================================================
# 2. 詞法分析 (Lexer)
# 原理：將字串切分成標記，取代 C 語言的指標位移(*src++)，改用 pos 索引。
# =========================================================
class Lexer:
    def __init__(self, src: str):
        self.src = src
        self.pos = 0
        self.cur_token = None
        self.next_token() # 預讀第一個 Token

    def next_token(self):
        while True:
            # 忽略空格、換行
            while self.pos < len(self.src) and self.src[self.pos].isspace():
                self.pos += 1
            
            if self.pos >= len(self.src):
                self.cur_token = Token(TokenType.TK_EOF, "")
                return

            # 處理註解
            if self.src[self.pos] == '/':
                # 單行註解 //
                if self.pos + 1 < len(self.src) and self.src[self.pos + 1] == '/':
                    self.pos += 2
                    while self.pos < len(self.src) and self.src[self.pos] != '\n':
                        self.pos += 1
                    continue
                # 多行註解 /* ... */
                elif self.pos + 1 < len(self.src) and self.src[self.pos + 1] == '*':
                    self.pos += 2
                    while self.pos + 1 < len(self.src) and not (self.src[self.pos] == '*' and self.src[self.pos + 1] == '/'):
                        self.pos += 1
                    if self.pos + 1 < len(self.src):
                        self.pos += 2 # 跳過 */
                    continue
            break

        start = self.pos

        # 辨識數字 (NUM)
        if self.src[self.pos].isdigit():
            while self.pos < len(self.src) and self.src[self.pos].isdigit():
                self.pos += 1
            self.cur_token = Token(TokenType.TK_NUM, self.src[start:self.pos])
            return

        # 辨識識別碼 (ID) 與 關鍵字 (Keyword)
        if self.src[self.pos].isalpha() or self.src[self.pos] == '_':
            while self.pos < len(self.src) and (self.src[self.pos].isalnum() or self.src[self.pos] == '_'):
                self.pos += 1
            text = self.src[start:self.pos]
            
            keywords = {
                "func": TokenType.TK_FUNC,
                "return": TokenType.TK_RETURN,
                "if": TokenType.TK_IF
            }
            self.cur_token = Token(keywords.get(text, TokenType.TK_ID), text)
            return

        # 辨識運算符與符號
        ch = self.src[self.pos]
        self.pos += 1
        
        symbols = {
            '(': TokenType.TK_LPAREN, ')': TokenType.TK_RPAREN,
            '{': TokenType.TK_LBRACE, '}': TokenType.TK_RBRACE,
            '+': TokenType.TK_PLUS,   '-': TokenType.TK_MINUS,
            '*': TokenType.TK_MUL,    '/': TokenType.TK_DIV,
            ',': TokenType.TK_COMMA,  ';': TokenType.TK_SEMICOLON,
            '<': TokenType.TK_LT,     '>': TokenType.TK_GT
        }
        
        if ch in symbols:
            self.cur_token = Token(symbols[ch], ch)
        elif ch == '=':
            if self.pos < len(self.src) and self.src[self.pos] == '=':
                self.pos += 1
                self.cur_token = Token(TokenType.TK_EQ, "==")
            else:
                self.cur_token = Token(TokenType.TK_ASSIGN, "=")
        else:
            raise SyntaxError(f"未知的字元: {ch}")

# =========================================================
# 3. 語法解析 (Parser) - 遞迴下降法
# =========================================================
class Parser:
    def __init__(self, lexer: Lexer):
        self.lexer = lexer
        self.quads =[]
        self.t_idx = 0

    def new_t(self) -> str:
        self.t_idx += 1
        return f"t{self.t_idx}"

    def emit(self, op: str, a1: str, a2: str, res: str):
        self.quads.append(Quad(op, a1, a2, res))
        print(f"{len(self.quads)-1:03d}: {op:<10} {a1:<10} {a2:<10} {res:<10}")

    def consume(self):
        self.lexer.next_token()

    @property
    def cur(self):
        return self.lexer.cur_token

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
                while self.cur.type != TokenType.TK_RPAREN:
                    arg = self.expression()
                    self.emit("PARAM", arg, "-", "-")
                    count += 1
                    if self.cur.type == TokenType.TK_COMMA:
                        self.consume()
                self.consume()
                res = self.new_t()
                self.emit("CALL", name, str(count), res)
            else:
                res = name
        elif self.cur.type == TokenType.TK_LPAREN:
            self.consume()
            res = self.expression()
            self.consume()
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
            self.consume(); self.consume() # if, (
            cond = self.expression()
            self.consume(); self.consume() # ), {
            
            jmp_idx = len(self.quads)
            self.emit("JMP_F", cond, "-", "?") # Backpatching
            
            while self.cur.type != TokenType.TK_RBRACE:
                self.statement()
            self.consume() # }
            
            self.quads[jmp_idx].result = str(len(self.quads)) # 回填跳轉地址
            
        elif self.cur.type == TokenType.TK_ID:
            name = self.cur.text
            self.consume()
            if self.cur.type == TokenType.TK_ASSIGN:
                self.consume()
                res = self.expression()
                self.emit("STORE", res, "-", name)
                if self.cur.type == TokenType.TK_SEMICOLON:
                    self.consume()
                    
        elif self.cur.type == TokenType.TK_RETURN:
            self.consume()
            res = self.expression()
            self.emit("RET_VAL", res, "-", "-")
            if self.cur.type == TokenType.TK_SEMICOLON:
                self.consume()

    def parse_program(self):
        while self.cur.type != TokenType.TK_EOF:
            if self.cur.type == TokenType.TK_FUNC:
                self.consume()
                f_name = self.cur.text
                self.emit("FUNC_BEG", f_name, "-", "-")
                self.consume(); self.consume() # name, (
                
                while self.cur.type == TokenType.TK_ID:
                    self.emit("FORMAL", self.cur.text, "-", "-")
                    self.consume()
                    if self.cur.type == TokenType.TK_COMMA:
                        self.consume()
                        
                self.consume(); self.consume() # ), {
                while self.cur.type != TokenType.TK_RBRACE:
                    self.statement()
                self.emit("FUNC_END", f_name, "-", "-")
                self.consume() # }
            else:
                self.statement()

# =========================================================
# 4. 虛擬機 (Virtual Machine)
# =========================================================
class Frame:
    def __init__(self, ret_pc: int = 0, ret_var: str = ""):
        self.vars = {} # 用字典取代 C 語言的 names/values 陣列
        self.ret_pc = ret_pc
        self.ret_var = ret_var
        self.incoming_args =[]
        self.formal_idx = 0

class VM:
    def __init__(self, quads: list):
        self.quads = quads
        self.stack = [Frame()] # 初始化全域環境
        self.sp = 0

    def get_var(self, name: str) -> int:
        if name.isdigit() or (name.startswith('-') and name[1:].isdigit()):
            return int(name)
        if name == "-": return 0
        return self.stack[self.sp].vars.get(name, 0)

    def set_var(self, name: str, val: int):
        self.stack[self.sp].vars[name] = val

    def run(self):
        pc = 0
        param_stack =[]
        
        # 預掃描：記錄函數進入點
        func_map = {}
        for i, q in enumerate(self.quads):
            if q.op == "FUNC_BEG":
                func_map[q.arg1] = i + 1

        print("\n=== VM 執行開始 ===")

        while pc < len(self.quads):
            q = self.quads[pc]

            if q.op == "FUNC_BEG":
                while self.quads[pc].op != "FUNC_END":
                    pc += 1
            elif q.op == "IMM": self.set_var(q.result, int(q.arg1))
            elif q.op == "ADD": self.set_var(q.result, self.get_var(q.arg1) + self.get_var(q.arg2))
            elif q.op == "SUB": self.set_var(q.result, self.get_var(q.arg1) - self.get_var(q.arg2))
            elif q.op == "MUL": self.set_var(q.result, self.get_var(q.arg1) * self.get_var(q.arg2))
            elif q.op == "DIV": self.set_var(q.result, self.get_var(q.arg1) // max(self.get_var(q.arg2), 1)) # 避免除以0
            elif q.op == "CMP_EQ": self.set_var(q.result, 1 if self.get_var(q.arg1) == self.get_var(q.arg2) else 0)
            elif q.op == "CMP_LT": self.set_var(q.result, 1 if self.get_var(q.arg1) < self.get_var(q.arg2) else 0)
            elif q.op == "CMP_GT": self.set_var(q.result, 1 if self.get_var(q.arg1) > self.get_var(q.arg2) else 0)
            elif q.op == "STORE": self.set_var(q.result, self.get_var(q.arg1))
            elif q.op == "JMP_F":
                if self.get_var(q.arg1) == 0:
                    pc = int(q.result) - 1
            elif q.op == "PARAM":
                param_stack.append(self.get_var(q.arg1))
            elif q.op == "CALL":
                p_count = int(q.arg2)
                target_pc = func_map[q.arg1]
                
                new_frame = Frame(ret_pc=pc + 1, ret_var=q.result)
                
                # 從參數暫存區取得參數
                if p_count > 0:
                    new_frame.incoming_args = param_stack[-p_count:]
                    del param_stack[-p_count:]
                
                self.stack.append(new_frame)
                self.sp += 1
                pc = target_pc
                continue
            elif q.op == "FORMAL":
                frame = self.stack[self.sp]
                self.set_var(q.arg1, frame.incoming_args[frame.formal_idx])
                frame.formal_idx += 1
            elif q.op == "RET_VAL":
                ret_val = self.get_var(q.arg1)
                ret_address = self.stack[self.sp].ret_pc
                target_var = self.stack[self.sp].ret_var
                
                self.stack.pop() # 銷毀當前堆疊幀
                self.sp -= 1     # 回到 Caller
                
                self.set_var(target_var, ret_val)
                pc = ret_address
                continue
                
            pc += 1

        print("=== VM 執行完畢 ===\n\n全域變數結果:")
        for name, val in self.stack[0].vars.items():
            if not name.startswith('t'): # 忽略暫存變數
                print(f">> {name} = {val}")

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
    
    vm = VM(parser.quads)
    vm.run()

if __name__ == "__main__":
    main()