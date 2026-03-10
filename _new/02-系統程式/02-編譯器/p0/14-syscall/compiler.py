import sys
import time
import random
from enum import Enum, auto

# =========================================================
# 程式語言 EBNF (Extended Backus-Naur Form) 語法定義
# =========================================================
# program ::= ( function_def | statement )*
#
# function_def ::= "func" id "(" [ id ("," id)* ] ")" "{" statement* "}"
#
# statement ::= "if" "(" expression ")" "{" statement* "}"[ "else" "{" statement* "}" ]
#             | "while" "(" expression ")" "{" statement* "}"
#             | "for" "(" [expr_or_assign] ";" [expression] ";"[expr_or_assign] ")" "{" statement* "}"
#             | "break" ";"
#             | "continue" ";"
#             | "return" expression ";"
#             | expr_or_assign ";"
#
# expr_or_assign ::= id { "[" expression "]" | "." id | "(" [ expression ("," expression)* ] ")" }[ "=" expression ]
#
# expression ::= arith_expr[ ("==" | "<" | ">") arith_expr ]
#
# arith_expr ::= term ( ("+" | "-") term )*
#
# term ::= factor ( ("*" | "/") factor )*
#
# factor ::= primary { "[" expression "]" | "." id | "(" [ expression ("," expression)* ] ")" }
#
# primary ::= num 
#           | string 
#           | id 
#           | "[" [ expression (";" expression | ("," expression)* ) ] "]" 
#           | "{"[ (id|string) ":" expression ("," (id|string) ":" expression)* ] "}" 
#           | "(" expression ")"
# =========================================================


# =========================================================
# 錯誤回報工具：定位原始碼位置並輸出指標 ^ 符號
# =========================================================
def report_error(src: str, pos: int, msg: str):
    lines = src.split('\n')
    current_pos = 0
    line_idx = 0
    for i, l in enumerate(lines):
        if current_pos + len(l) + 1 > pos: # 判斷錯誤在哪一行
            line_idx = i; break
        current_pos += len(l) + 1
    col_idx = pos - current_pos
    if line_idx >= len(lines):
        line_idx = len(lines) - 1; col_idx = len(lines[line_idx])

    print(f"\n❌ [語法錯誤] 第 {line_idx + 1} 行, 第 {col_idx + 1} 字元: {msg}")
    line_str = lines[line_idx]
    print(f"  {line_str}")
    # 根據 Tab 或空格對齊指標
    indicator = "".join(['\t' if i < len(line_str) and line_str[i] == '\t' else ' ' for i in range(col_idx)]) + "^"
    print(f"  {indicator}")
    sys.exit(1)


# =========================================================
# 1. 詞彙標記與中間碼 (Intermediate Representation)
# =========================================================
class TokenType(Enum):
    # 關鍵字 (已移除 print，改為系統呼叫)
    TK_FUNC = auto(); TK_RETURN = auto(); TK_IF = auto(); TK_ELSE = auto() 
    TK_WHILE = auto(); TK_FOR = auto()
    TK_BREAK = auto(); TK_CONTINUE = auto()
    # 識別碼、常數
    TK_ID = auto(); TK_NUM = auto(); TK_STRING = auto()
    # 符號
    TK_LPAREN = auto(); TK_RPAREN = auto()     # ()
    TK_LBRACE = auto(); TK_RBRACE = auto()     # {}
    TK_LBRACKET = auto(); TK_RBRACKET = auto() #[]
    TK_DOT = auto(); TK_COLON = auto()         # . :
    TK_COMMA = auto(); TK_SEMICOLON = auto()   # , ;
    # 運算子
    TK_ASSIGN = auto(); TK_PLUS = auto(); TK_MINUS = auto(); TK_MUL = auto(); TK_DIV = auto()
    TK_EQ = auto(); TK_LT = auto(); TK_GT = auto()
    TK_EOF = auto()

class Token:
    """儲存單一 Token 詞彙資訊"""
    def __init__(self, t_type: TokenType, text: str, pos: int):
        self.type = t_type; self.text = text; self.pos = pos

class Quad:
    """四位組 (Quadruple) 中間碼：op arg1 arg2 result
    VM 藉由逐行執行這種抽象組合指令來運作"""
    def __init__(self, op: str, arg1: str, arg2: str, result: str):
        self.op = op; self.arg1 = arg1; self.arg2 = arg2; self.result = result


# =========================================================
# 2. 詞法分析 (Lexer)：將原始字串轉為 Token 流
# =========================================================
class Lexer:
    def __init__(self, src: str):
        self.src = src; self.pos = 0; self.cur_token = None
        self.next_token()

    def next_token(self):
        while True:
            # 跳過空格
            while self.pos < len(self.src) and self.src[self.pos].isspace(): self.pos += 1
            if self.pos >= len(self.src):
                self.cur_token = Token(TokenType.TK_EOF, "", self.pos); return

            # 處理單行註解 (//) 或多行註解 (/* */)
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

        # 處理字串常數 "..."
        if self.src[self.pos] == '"':
            self.pos += 1 
            start_str = self.pos
            while self.pos < len(self.src) and self.src[self.pos] != '"': self.pos += 1
            if self.pos >= len(self.src): report_error(self.src, start, "字串缺少結尾的雙引號 '\"'")
            text = self.src[start_str:self.pos]
            self.pos += 1 
            self.cur_token = Token(TokenType.TK_STRING, text, start); return

        # 處理數字
        if self.src[self.pos].isdigit():
            while self.pos < len(self.src) and self.src[self.pos].isdigit(): self.pos += 1
            self.cur_token = Token(TokenType.TK_NUM, self.src[start:self.pos], start); return

        # 處理關鍵字與變數名稱 (Identifier)
        if self.src[self.pos].isalpha() or self.src[self.pos] == '_':
            while self.pos < len(self.src) and (self.src[self.pos].isalnum() or self.src[self.pos] == '_'): self.pos += 1
            text = self.src[start:self.pos]
            keywords = { 
                "func": TokenType.TK_FUNC, "return": TokenType.TK_RETURN, 
                "if": TokenType.TK_IF, "else": TokenType.TK_ELSE,
                "while": TokenType.TK_WHILE, "for": TokenType.TK_FOR,
                "break": TokenType.TK_BREAK, "continue": TokenType.TK_CONTINUE
            }
            # 如果不是關鍵字，統一視為識別碼 (包含 print)
            self.cur_token = Token(keywords.get(text, TokenType.TK_ID), text, start); return

        # 處理單/雙字元符號與運算子
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
# 3. 語法解析 (Parser)：將 Token 流轉為四位組 (Quads)
# =========================================================
class Parser:
    def __init__(self, lexer: Lexer):
        self.lexer = lexer
        self.quads =[]       # 儲存產生的中間碼串列
        self.string_pool =[] # 儲存字串常數（減少重複記憶體並方便 VM 透過 index 取用）
        self.loop_stack =[]  # 用於處理迴圈嵌套與 break/continue 的跳轉回填 (Backpatching)
        self.t_idx = 0        # 暫存變數 (t1, t2...) 計數器

    @property
    def cur(self): return self.lexer.cur_token

    def consume(self): self.lexer.next_token()
        
    def error(self, msg: str):
        report_error(self.lexer.src, self.cur.pos, f"{msg} (目前讀到: '{self.cur.text}')")

    def expect(self, expected_type: TokenType, err_msg: str):
        """期望目前 Token 為某種類型，否則報錯"""
        if self.cur.type == expected_type: self.consume()
        else: self.error(err_msg)

    def new_t(self) -> str:
        """生成唯一的暫存變數名稱 (例如: t1, t2)"""
        self.t_idx += 1
        return f"t{self.t_idx}"

    def emit(self, op: str, a1: str, a2: str, res: str) -> int:
        """發送一條四位組指令，並返回該指令在 quads 中的索引（行號），用於後續跳轉位置回填"""
        idx = len(self.quads)
        self.quads.append(Quad(op, a1, a2, res))
        print(f"{idx:03d}: {op:<12} {a1:<10} {a2:<10} {res:<10}")
        return idx

    # ================= 處理賦值與鏈式呼叫 (L-Value / R-Value) =================
    # EBNF: expr_or_assign ::= id { "[" expression "]" | "." id | "(" [ expression ("," expression)* ] ")" }[ "=" expression ]
    def expr_or_assign(self):
        """處理如 a = 1, a[0] = 1, obj.prop = 1 或單純的函式呼叫 a()"""
        name = self.cur.text; self.consume()
        obj = name; path =[] # path 用於紀錄連續存取的路徑（陣列索引或屬性字串）
        
        # 處理點運算、中括號、圓括號 (處理巢狀或連續呼叫: obj.foo().bar[0] 或 print())
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
                # 若是函式呼叫，前方的路徑（陣列索引或字典鍵值）必須先解析取出真正的函式對象
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
        
        # 賦值判斷
        if self.cur.type == TokenType.TK_ASSIGN:
            self.consume(); val = self.expression()
            if not path: # 一般變數賦值: a = 1
                self.emit("STORE", val, "-", obj)
            else: # 物件/陣列屬性賦值: obj[k] = val
                for p in path[:-1]: # 先取到目標的最底層父結構
                    t = self.new_t(); self.emit("GET_ITEM", obj, p, t); obj = t
                self.emit("SET_ITEM", obj, path[-1], val) # 設置最後一個鍵值

    # ================= 基本表達式元 (Primary) =================
    # EBNF: primary ::= num | string | id | "["[ expression (";" expression | ("," expression)* ) ] "]" 
    #                 | "{"[ (id|string) ":" expression ("," (id|string) ":" expression)* ] "}" 
    #                 | "(" expression ")"
    def primary(self) -> str:
        """處理最小單位的表達式（常數、變數、字面量宣告、括號）"""
        if self.cur.type == TokenType.TK_NUM:
            t = self.new_t(); self.emit("IMM", self.cur.text, "-", t); self.consume(); return t
        elif self.cur.type == TokenType.TK_STRING:
            t = self.new_t(); pool_idx = len(self.string_pool); self.string_pool.append(self.cur.text)
            self.emit("LOAD_STR", str(pool_idx), "-", t); self.consume(); return t
        elif self.cur.type == TokenType.TK_ID:
            name = self.cur.text; self.consume(); return name
            
        elif self.cur.type == TokenType.TK_LBRACKET: 
            # 處理陣列宣告 [1, 2, 3] 或 [初始值; 長度]
            self.consume()
            t = self.new_t()
            
            # 如果裡面什麼都沒有[]
            if self.cur.type == TokenType.TK_RBRACKET:
                self.emit("NEW_ARR", "-", "-", t)
            else:
                # 讀取第一個表達式 (可能是元素 1，也可能是初始值)
                val = self.expression()
                
                # 情況 1：遇到分號 -> 代表這是 [初始值; 長度] 語法
                if self.cur.type == TokenType.TK_SEMICOLON:
                    self.consume()           # 吃掉 ';'
                    size = self.expression() # 讀取陣列長度
                    self.emit("INIT_ARR", val, size, t) # 發送初始化固定長度陣列的中間碼
                    
                # 情況 2：遇到逗號或直接結束 -> 代表這是傳統的 [元素1, 元素2] 語法
                else:
                    self.emit("NEW_ARR", "-", "-", t)
                    self.emit("APPEND_ITEM", t, "-", val)
                    while self.cur.type == TokenType.TK_COMMA:
                        self.consume()
                        val = self.expression()
                        self.emit("APPEND_ITEM", t, "-", val)
                        
            self.expect(TokenType.TK_RBRACKET, "陣列預期要有 ']' 結尾"); return t
            
        elif self.cur.type == TokenType.TK_LBRACE: # 字典字面量 {key: val}
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

    # ================= 運算優先級解析 (factor -> term -> arith -> expression) =================
    # EBNF: factor ::= primary { "[" expression "]" | "." id | "("[ expression ("," expression)* ] ")" }
    def factor(self) -> str:
        """處理屬性存取與函式呼叫的 R-Value (作為右值被取用)"""
        res = self.primary()
        while self.cur.type in (TokenType.TK_LBRACKET, TokenType.TK_DOT, TokenType.TK_LPAREN):
            if self.cur.type == TokenType.TK_LBRACKET:
                self.consume(); idx = self.expression(); self.expect(TokenType.TK_RBRACKET, "預期 ']'")
                t = self.new_t(); self.emit("GET_ITEM", res, idx, t); res = t
            elif self.cur.type == TokenType.TK_DOT:
                self.consume()
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

    # EBNF: term ::= factor ( ("*" | "/") factor )*
    def term(self) -> str:
        """處理乘除運算 * /"""
        l = self.factor()
        while self.cur.type in (TokenType.TK_MUL, TokenType.TK_DIV):
            op = "MUL" if self.cur.type == TokenType.TK_MUL else "DIV"
            self.consume(); r = self.factor(); t = self.new_t()
            self.emit(op, l, r, t); l = t
        return l

    # EBNF: arith_expr ::= term ( ("+" | "-") term )*
    def arith_expr(self) -> str:
        """處理加減運算 + -"""
        l = self.term()
        while self.cur.type in (TokenType.TK_PLUS, TokenType.TK_MINUS):
            op = "ADD" if self.cur.type == TokenType.TK_PLUS else "SUB"
            self.consume(); r = self.term(); t = self.new_t()
            self.emit(op, l, r, t); l = t
        return l

    # EBNF: expression ::= arith_expr[ ("==" | "<" | ">") arith_expr ]
    def expression(self) -> str:
        """處理比較運算 == < > (最低運算優先級)"""
        l = self.arith_expr()
        if self.cur.type in (TokenType.TK_EQ, TokenType.TK_LT, TokenType.TK_GT):
            op = "CMP_EQ" if self.cur.type == TokenType.TK_EQ else "CMP_LT" if self.cur.type == TokenType.TK_LT else "CMP_GT"
            self.consume(); r = self.arith_expr(); t = self.new_t()
            self.emit(op, l, r, t); return t
        return l

    # ================= 陳述句 (Statement) 流程控制 =================
    # EBNF: statement ::= "if" "(" expression ")" "{" statement* "}"[ "else" "{" statement* "}" ]
    #                   | "while" "(" expression ")" "{" statement* "}"
    #                   | "for" "("[expr_or_assign] ";" [expression] ";"[expr_or_assign] ")" "{" statement* "}"
    #                   | "break" ";"
    #                   | "continue" ";"
    #                   | "return" expression ";"
    #                   | expr_or_assign ";"
    def statement(self):
        """解析陳述句與控制流結構"""
        if self.cur.type == TokenType.TK_IF:
            self.consume(); self.expect(TokenType.TK_LPAREN, "預期 '('")
            cond = self.expression()
            self.expect(TokenType.TK_RPAREN, "預期 ')'"); self.expect(TokenType.TK_LBRACE, "預期 '{'")
            
            # 若條件為 False，需要跳轉 (JMP_F) 到 if 區塊外或是 else 區塊。先把跳轉目標標示為 "?" 等待回填
            jmp_f_idx = self.emit("JMP_F", cond, "-", "?") 
            while self.cur.type not in (TokenType.TK_RBRACE, TokenType.TK_EOF): self.statement()
            self.expect(TokenType.TK_RBRACE, "預期 '}'")
            
            # 檢查是否有 else 區塊
            if self.cur.type == TokenType.TK_ELSE:
                # 若 if 區塊成功執行完畢，則遇到無條件跳轉 (JMP) 以跨越 else 區塊
                jmp_end_idx = self.emit("JMP", "-", "-", "?") 
                
                # 如果前方的條件為 False，此處將其跳轉目標指向 else 的起點
                self.quads[jmp_f_idx].result = str(len(self.quads)) 
                
                self.consume(); self.expect(TokenType.TK_LBRACE, "預期 '{'")
                while self.cur.type not in (TokenType.TK_RBRACE, TokenType.TK_EOF): self.statement()
                self.expect(TokenType.TK_RBRACE, "預期 '}'")
                
                # if 區塊末尾的 JMP 指令回填，跳轉目標為 else 區塊之後 (整個結構結束)
                self.quads[jmp_end_idx].result = str(len(self.quads)) 
            else:
                # 若沒有 else，條件為 False 時直接跳到 if 區塊外
                self.quads[jmp_f_idx].result = str(len(self.quads))
            
        elif self.cur.type == TokenType.TK_WHILE:
            self.consume(); self.expect(TokenType.TK_LPAREN, "預期 '('")
            cond_idx = len(self.quads) # 紀錄條件判斷的位址 (continue 目標)
            cond = self.expression()
            self.expect(TokenType.TK_RPAREN, "預期 ')'"); self.expect(TokenType.TK_LBRACE, "預期 '{'")
            
            jmp_f_idx = self.emit("JMP_F", cond, "-", "?") # 條件不滿足則跳出迴圈
            # 推入 loop_stack 提供內部的 break/continue 使用
            self.loop_stack.append({'break':[], 'continue': cond_idx})
            
            while self.cur.type not in (TokenType.TK_RBRACE, TokenType.TK_EOF): self.statement()
            self.emit("JMP", "-", "-", str(cond_idx)) # 迴圈結束，無條件繞回頂部檢查
            self.expect(TokenType.TK_RBRACE, "預期 '}'")
            
            # 回填
            end_idx = len(self.quads)
            self.quads[jmp_f_idx].result = str(end_idx) # JMP_F 目標
            loop_ctx = self.loop_stack.pop()
            for b_idx in loop_ctx['break']: self.quads[b_idx].result = str(end_idx) # 處理內部被觸發的 break
                
        elif self.cur.type == TokenType.TK_FOR:
            """FOR 迴圈編織：Init -> Cond -> (JMP Body) -> Step -> (JMP Cond) -> Body -> (JMP Step)"""
            self.consume(); self.expect(TokenType.TK_LPAREN, "預期 '('")
            
            # 1. Init (初始化區)
            if self.cur.type != TokenType.TK_SEMICOLON: self.expr_or_assign()
            self.expect(TokenType.TK_SEMICOLON, "預期 ';'")
            
            # 2. Condition (條件檢查區)
            cond_idx = len(self.quads)
            if self.cur.type != TokenType.TK_SEMICOLON:
                cond = self.expression()
            else:
                cond = self.new_t(); self.emit("IMM", "1", "-", cond) # 如果無條件則設為 1 (True)
            
            jmp_f_idx = self.emit("JMP_F", cond, "-", "?")   # 失敗跳出
            jmp_body_idx = self.emit("JMP", "-", "-", "?")   # 條件滿足，則需跨過 Step 區間，先跳去執行 Body
            
            self.expect(TokenType.TK_SEMICOLON, "預期 ';'")
            
            # 3. Step (步進區)
            step_idx = len(self.quads)
            if self.cur.type != TokenType.TK_RPAREN: self.expr_or_assign()
            self.emit("JMP", "-", "-", str(cond_idx)) # 步進完成後，跳回 Cond 進行下一次檢查
            
            self.expect(TokenType.TK_RPAREN, "預期 ')'"); self.expect(TokenType.TK_LBRACE, "預期 '{'")
            
            # 4. Body (主體區)
            self.quads[jmp_body_idx].result = str(len(self.quads)) # 回填 JMP_BODY 目標到此處
            self.loop_stack.append({'break':[], 'continue': step_idx})
            
            while self.cur.type not in (TokenType.TK_RBRACE, TokenType.TK_EOF): self.statement()
            self.emit("JMP", "-", "-", str(step_idx)) # Body 執行完畢，無條件跳至 Step 步進區
            self.expect(TokenType.TK_RBRACE, "預期 '}'")
            
            # 5. 回填中斷區
            end_idx = len(self.quads)
            self.quads[jmp_f_idx].result = str(end_idx) # 回填 JMP_F 的中斷位址
            loop_ctx = self.loop_stack.pop()
            for b_idx in loop_ctx['break']: self.quads[b_idx].result = str(end_idx)

        elif self.cur.type == TokenType.TK_BREAK:
            self.consume()
            if not self.loop_stack: self.error("break 必須在迴圈內部使用")
            b_idx = self.emit("JMP", "-", "-", "?") # 中斷迴圈，目標待填
            self.loop_stack[-1]['break'].append(b_idx)
            self.expect(TokenType.TK_SEMICOLON, "預期 ';'")
            
        elif self.cur.type == TokenType.TK_CONTINUE:
            self.consume()
            if not self.loop_stack: self.error("continue 必須在迴圈內部使用")
            c_target = self.loop_stack[-1]['continue']
            self.emit("JMP", "-", "-", str(c_target)) # 跳回條件/步進區
            self.expect(TokenType.TK_SEMICOLON, "預期 ';'")
                
        elif self.cur.type == TokenType.TK_RETURN:
            self.consume(); res = self.expression(); self.emit("RET_VAL", res, "-", "-")
            self.expect(TokenType.TK_SEMICOLON, "預期 ';'")
            
        elif self.cur.type == TokenType.TK_ID:
            # 由於移除了專門的 print 陳述句，print("...") 會作為普通的函數呼叫落在此處
            self.expr_or_assign(); self.expect(TokenType.TK_SEMICOLON, "預期 ';'")
            
        else:
            self.error("無法辨識的陳述句或語法結構")

    # EBNF: program ::= ( function_def | statement )*
    # EBNF: function_def ::= "func" id "(" [ id ("," id)* ] ")" "{" statement* "}"
    def parse_program(self):
        """主解析循環：處理函數定義或全域陳述"""
        while self.cur.type != TokenType.TK_EOF:
            if self.cur.type == TokenType.TK_FUNC:
                self.consume()
                f_name = self.cur.text; self.consume()
                self.emit("FUNC_BEG", f_name, "-", "-")
                self.expect(TokenType.TK_LPAREN, "預期 '('")
                if self.cur.type != TokenType.TK_RPAREN: # 解析形參 (Formal Parameters)
                    while True:
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
# 4. 虛擬機 (Virtual Machine)：直譯中間碼
# =========================================================
class Frame:
    """函式執行框架 (Stack Frame)"""
    def __init__(self, ret_pc: int = 0, ret_var: str = ""):
        self.vars = {}           # 當前 Scope 的變數 (Memory)
        self.ret_pc = ret_pc     # 函式結束後回到的 PC
        self.ret_var = ret_var   # 函式回傳值要存入的外部呼叫點變數名
        self.incoming_args =[]  # 傳入的參數值列表
        self.formal_idx = 0      # 參數讀取計數

class VM:
    def __init__(self, quads: list, string_pool: list):
        self.quads = quads; self.string_pool = string_pool
        self.stack =[Frame()]; self.sp = 0 # 框架堆疊，索引 0 代表全域環境

    def get_var(self, name: str):
        """解析運算元：可能是數字、暫存變數或使用者定義變數"""
        if name.isdigit() or (name.startswith('-') and name[1:].isdigit()): return int(name)
        if name == "-": return 0
        return self.stack[self.sp].vars.get(name, 0)

    def set_var(self, name: str, val):
        self.stack[self.sp].vars[name] = val

    # =========================================================
    # 系統內建函數 (Native Built-ins)
    # 負責處理底層記憶體存取、型別轉換、作業系統與硬體 I/O 等無法靠語法完成的事項
    # =========================================================
    def system_call(self, f_name: str, args: list):
        """
        處理攔截到的系統函數呼叫
        回傳值: (is_native_function, return_value)
        """
        if f_name == "print":       # 【新增】將列印改為系統呼叫
            out_str = " ".join(str(arg) for arg in args)
            print("[程式輸出] >> " + out_str)
            return True, 0          # print 不回傳有效值，用 0 代替 None 以策安全
            
        elif f_name == "array":     # 動態初始化陣列
            if len(args) != 2: raise Exception("array 需 2 個參數 (長度, 預設值)")
            if not isinstance(args[0], int): raise Exception("array 長度需為整數")
            return True, [args[1]] * args[0]
            
        elif f_name == "len":       # 取得長度
            if len(args) != 1: raise Exception("len 需 1 個參數")
            return True, len(args[0])
            
        elif f_name == "push":      # 陣列尾端推入
            args[0].append(args[1])
            return True, args[0]    # 順便回傳修改後的陣列
            
        elif f_name == "pop":       # 陣列尾端彈出
            return True, args[0].pop()
            
        elif f_name == "keys":      # 取出字典所有鍵值
            return True, list(args[0].keys())
            
        elif f_name == "has_key":   # 判斷字典是否包含鍵值
            return True, 1 if args[1] in args[0] else 0
            
        elif f_name == "remove":    # 移除字典中特定鍵值
            if args[1] in args[0]:
                del args[0][args[1]]
            return True, args[0]
            
        elif f_name == "typeof":    # 取得型別名稱
            val = args[0]
            if isinstance(val, int): t_str = "int"
            elif isinstance(val, str): t_str = "string"
            elif isinstance(val, list): t_str = "array"
            elif isinstance(val, dict): t_str = "dict"
            else: t_str = "unknown"
            return True, t_str
            
        elif f_name == "int":       # 轉型為整數
            return True, int(args[0])
            
        elif f_name == "str":       # 轉型為字串
            return True, str(args[0])
            
        elif f_name == "ord":       # 字元轉 ASCII 編碼
            return True, ord(args[0])
            
        elif f_name == "chr":       # ASCII 編碼轉字元
            return True, chr(args[0])
            
        elif f_name == "input":     # 終端機讀取輸入
            msg = args[0] if len(args) > 0 else ""
            return True, input(str(msg))
            
        elif f_name == "time":      # 獲取當前 UNIX 時間戳
            return True, time.time()
            
        elif f_name == "random":    # 產生 0.0 ~ 1.0 亂數
            return True, random.random()
            
        elif f_name == "exit":      # 終止程式
            code = args[0] if len(args) > 0 else 0
            sys.exit(code)
            
        return False, None          # 若名稱不符，則代表它不是系統函數

    def run(self):
        pc = 0; param_stack =[] # param_stack 用於跨 Frame 暫存 CALL 之前的參數
        # 建立函式名稱到入口 PC 的映射表
        func_map = {q.arg1: i + 1 for i, q in enumerate(self.quads) if q.op == "FUNC_BEG"}

        print("\n=== VM 執行開始 ===")
        while pc < len(self.quads):
            q = self.quads[pc]
            try:
                # 函式定義在主執行緒中只會略過，它只能被 CALL 進去
                if q.op == "FUNC_BEG":
                    while self.quads[pc].op != "FUNC_END": pc += 1
                
                # 基本指派與運算 (保留獨立的運算指令)
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
                
                # 複合資料型態：陣列與字典
                elif q.op == "NEW_ARR": self.set_var(q.result,[])
                elif q.op == "INIT_ARR":
                    init_val = self.get_var(q.arg1)
                    arr_size = self.get_var(q.arg2)
                    if not isinstance(arr_size, int): raise Exception("陣列長度必須是整數")
                    self.set_var(q.result, [init_val] * arr_size)
                    
                elif q.op == "NEW_DICT": self.set_var(q.result, {})
                elif q.op == "APPEND_ITEM": self.get_var(q.arg1).append(self.get_var(q.result))
                elif q.op == "SET_ITEM": self.get_var(q.arg1)[self.get_var(q.arg2)] = self.get_var(q.result)
                elif q.op == "GET_ITEM": self.set_var(q.result, self.get_var(q.arg1)[self.get_var(q.arg2)])
                
                # 流程控制
                elif q.op == "JMP": pc = int(q.result) - 1
                elif q.op == "JMP_F":
                    # 如果條件為 0 (False)，就觸發跳轉
                    if self.get_var(q.arg1) == 0: pc = int(q.result) - 1
                
                # 函式呼叫機制
                elif q.op == "PARAM": param_stack.append(self.get_var(q.arg1)) # 準備發送的參數
                elif q.op == "CALL":
                    p_count = int(q.arg2)
                    f_name = self.get_var(q.arg1) if isinstance(self.get_var(q.arg1), str) else q.arg1
                    
                    # 取出準備傳入的參數列表
                    args = param_stack[-p_count:] if p_count > 0 else[]
                    
                    # =======================================================
                    # 首先，嘗試透過 system_call 攔截系統內建函數 (包含 print)
                    # =======================================================
                    is_native, ret_val = self.system_call(f_name, args)
                    if is_native:
                        if p_count > 0:
                            del param_stack[-p_count:] # 清空已消耗的參數
                        self.set_var(q.result, ret_val) # 將原生函數結果存入預期變數
                        pc += 1
                        continue # 直接跳下一個 PC，不必推入 Frame
                    
                    # =======================================================
                    # 如果不是系統函數，則尋找使用者定義函數並推入 Frame 準備跳轉
                    # =======================================================
                    target_pc = func_map.get(f_name)
                    if target_pc is None: raise Exception(f"找不到函數 '{f_name}'")
                    
                    # 建立新堆疊，推入執行緒
                    new_frame = Frame(ret_pc=pc + 1, ret_var=q.result)
                    if p_count > 0:
                        new_frame.incoming_args = args # 直接將取出的 args 交給新 Frame
                        del param_stack[-p_count:]
                    self.stack.append(new_frame); self.sp += 1; pc = target_pc; continue
                    
                elif q.op == "FORMAL": # 函式內部：依序將外部參數綁定至自己 Scope 的變數
                    frame = self.stack[self.sp]
                    self.set_var(q.arg1, frame.incoming_args[frame.formal_idx]); frame.formal_idx += 1
                elif q.op == "RET_VAL": # return 陳述句
                    ret_val = self.get_var(q.arg1); ret_address = self.stack[self.sp].ret_pc; target_var = self.stack[self.sp].ret_var
                    self.stack.pop(); self.sp -= 1 # 摧毀堆疊
                    self.set_var(target_var, ret_val); pc = ret_address; continue
                elif q.op == "FUNC_END": # 函式自然結束（無 return 情況）
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