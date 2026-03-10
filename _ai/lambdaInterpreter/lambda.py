import sys
import re
sys.setrecursionlimit(10000)

# --- 1. AST 定義 ---
class Node: pass

class Variable(Node):
    def __init__(self, name): self.name = name
    def __repr__(self): return self.name

class Abstraction(Node):
    def __init__(self, param, body):
        self.param = param
        self.body = body
    def __repr__(self): return f"(\\{self.param}.{self.body})"

class Application(Node):
    def __init__(self, lhs, rhs):
        self.lhs = lhs
        self.rhs = rhs
    def __repr__(self): return f"({self.lhs} {self.rhs})"

# --- 2. Parser (遞迴下降法) ---
class Parser:
    def __init__(self, tokens):
        self.tokens = tokens
        self.pos = 0

    def next_token(self):
        return self.tokens[self.pos] if self.pos < len(self.tokens) else None

    def consume(self, expected=None):
        token = self.next_token()
        if expected and token != expected:
            raise SyntaxError(f"Expected {expected}, got {token}")
        self.pos += 1
        return token

    def parse_expression(self):
        # 處理 \x. M
        if self.next_token() == '\\':
            self.consume('\\')
            var = self.consume()
            self.consume('.')
            body = self.parse_expression()
            return Abstraction(var, body)
        
        # 處理 Application (左結合)
        expr = self.parse_atom()
        while self.next_token() and self.next_token() not in [')', '.', '=']:
            rhs = self.parse_atom()
            expr = Application(expr, rhs)
        return expr

    def parse_atom(self):
        token = self.next_token()
        if token == '(':
            self.consume('(')
            expr = self.parse_expression()
            self.consume(')')
            return expr
        elif token and re.match(r'[a-zA-Z_]\w*', token):
            return Variable(self.consume())
        raise SyntaxError(f"Unexpected token: {token}")

def tokenize(s):
    return re.findall(r'\\|\.|\(|\)|[a-zA-Z_]\w*|=|[;]', s)

class Interpreter:
    def __init__(self):
        self.env = {}
        self.counter = 0

    def fresh_var(self, old_name):
        self.counter += 1
        return f"{old_name}{self.counter}"

    def get_free_vars(self, node):
        if isinstance(node, Variable): return {node.name}
        if isinstance(node, Abstraction): return self.get_free_vars(node.body) - {node.param}
        if isinstance(node, Application): return self.get_free_vars(node.lhs) | self.get_free_vars(node.rhs)
        return set()

    def substitute(self, node, var, replacement):
        if isinstance(node, Variable):
            return replacement if node.name == var else node
        if isinstance(node, Abstraction):
            if node.param == var: return node
            if node.param in self.get_free_vars(replacement):
                new_param = self.fresh_var(node.param)
                new_body = self.substitute(node.body, node.param, Variable(new_param))
                return Abstraction(new_param, self.substitute(new_body, var, replacement))
            return Abstraction(node.param, self.substitute(node.body, var, replacement))
        if isinstance(node, Application):
            return Application(self.substitute(node.lhs, var, replacement),
                               self.substitute(node.rhs, var, replacement))

    # --- 核心修改：惰性求值 (Call-by-Name) ---
    def eval(self, node):
        if isinstance(node, Variable) and node.name in self.env:
            return self.eval(self.env[node.name])
            
        if isinstance(node, Application):
            lhs = self.eval(node.lhs)
            if isinstance(lhs, Abstraction):
                # Beta-reduction: 把右邊的參數「原封不動」塞進左邊，不提前求值 rhs
                return self.eval(self.substitute(lhs.body, lhs.param, node.rhs))
            return Application(lhs, self.eval(node.rhs))
            
        if isinstance(node, Abstraction):
            # 【關鍵】不要在這裡執行 self.eval(node.body)！
            # 只有當函數被應用 (Application) 時才去化簡內部。
            return node
            
        return node

    # --- 新增：強制展開 (Normal Order Reduction) ---
    def normalize(self, node):
        """用來在程式最後一步，把未展開的函數徹底計算乾淨以印出結果"""
        node = self.eval(node)
        if isinstance(node, Abstraction):
            return Abstraction(node.param, self.normalize(node.body))
        if isinstance(node, Application):
            return Application(self.normalize(node.lhs), self.normalize(node.rhs))
        return node

    def preprocess(self, code):
        import re
        code = re.sub(r'\(\*.*?\*\)', '', code, flags=re.DOTALL)
        return code

    def run(self, code):
        import re
        code = self.preprocess(code)
        tokens = re.findall(r'\\|\.|\(|\)|[a-zA-Z_]\w*|=|[;]', code)
        
        # ... 解析環境變數的邏輯不變 ...
        i = 0
        last_expr = None
        from __main__ import Parser # 確保你能抓到 Parser
        while i < len(tokens):
            if i + 1 < len(tokens) and tokens[i+1] == '=':
                name = tokens[i]
                start = i + 2
                try:
                    end = tokens.index(';', start)
                    expr_tokens = tokens[start:end]
                    self.env[name] = Parser(expr_tokens).parse_expression()
                    i = end + 1
                except ValueError:
                    print(f"Error: Definition of '{name}' must end with ';'")
                    return None
            else:
                try:
                    last_expr = Parser(tokens[i:]).parse_expression()
                    break
                except Exception as e:
                    print(f"Parse Error: {e}")
                    return None
        
        # 【修改】這裡改為呼叫 normalize
        return self.normalize(last_expr) if last_expr else "No expression to evaluate."

# --- 檔案讀取邏輯 ---
def main():
    if len(sys.argv) < 2:
        print("Usage: python interpreter.py <filename.lc>")
        return

    filename = sys.argv[1]
    try:
        with open(filename, 'r', encoding='utf-8') as f:
            source_code = f.read()
        
        interp = Interpreter()
        result = interp.run(source_code)
        
        print("--- Evaluation Result ---")
        print(result)
        
    except FileNotFoundError:
        print(f"Error: File '{filename}' not found.")
    except Exception as e:
        print(f"An unexpected error occurred: {e}")

if __name__ == "__main__":
    main()