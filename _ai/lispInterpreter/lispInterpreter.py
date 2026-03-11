import sys
import math
import operator as op

# --- 1. 定義環境 (Environment) ---
class Env(dict):
    def __init__(self, parms=(), args=(), outer=None):
        self.update(zip(parms, args))
        self.outer = outer
    def find(self, var):
        if var in self: return self
        if self.outer is None: raise NameError(f"Symbol '{var}' not found.")
        return self.outer.find(var)

def standard_env():
    env = Env()
    env.update({
        '+': op.add, '-': op.sub, '*': op.mul, '/': op.truediv,
        '=': op.eq, '>': op.gt, '<': op.lt, '>=': op.ge, '<=': op.le,
        'print': print, 'begin': lambda *x: x[-1]
    })
    return env

global_env = standard_env()

# --- 2. 解析器 (Parser) ---
def tokenize(chars):
    return chars.replace('(', ' ( ').replace(')', ' ) ').split()

def read_from_tokens(tokens):
    if not tokens: return None
    token = tokens.pop(0)
    if token == '(':
        L = []
        while tokens[0] != ')':
            L.append(read_from_tokens(tokens))
        tokens.pop(0)
        return L
    else:
        return atom(token)

def atom(token):
    try: return int(token)
    except ValueError:
        try: return float(token)
        except ValueError:
            return str(token)

# --- 3. 求值器 (Evaluator) ---
class Procedure:
    def __init__(self, parms, body, env):
        self.parms, self.body, self.env = parms, body, env
    def __call__(self, *args):
        return eval_ast(self.body, Env(self.parms, args, self.env))

def eval_ast(x, env=global_env):
    if isinstance(x, str):                # 變數
        return env.find(x)[x]
    elif not isinstance(x, list):         # 數字
        return x
    
    op_name, *args = x
    
    if op_name == 'if':
        (test, conseq, alt) = args
        exp = (conseq if eval_ast(test, env) else alt)
        return eval_ast(exp, env)
    elif op_name == 'defun':
        name, parms, body = args
        env[name] = Procedure(parms, body, env)
        return name
    else:                                 # 函數呼叫
        proc = eval_ast(op_name, env)
        vals = [eval_ast(arg, env) for arg in args]
        return proc(*vals)

# --- 4. 檔案讀取與主程式 ---
def run_file(filename):
    try:
        with open(filename, 'r', encoding='utf-8') as f:
            script = f.read()
        
        # 為了處理檔案中多個表達式，我們將其封裝在 (begin ...) 中
        tokens = tokenize(f"(begin {script} )")
        ast = read_from_tokens(tokens)
        
        # 執行 AST
        # 注意：這裡的 AST 結構是 ['begin', [...], [...]]
        # 因為 'begin' 已在 standard_env 定義，直接 eval 即可
        eval_ast(ast)
        
    except FileNotFoundError:
        print(f"錯誤：找不到檔案 '{filename}'")
    except Exception as e:
        print(f"執行時發生錯誤：{e}")

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("用法: python lispInterpreter.py <filename.lisp>")
    else:
        run_file(sys.argv[1])
