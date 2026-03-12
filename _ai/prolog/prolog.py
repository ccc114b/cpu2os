import sys
import re

# --- 核心資料結構 ---
class Var:
    def __init__(self, name): self.name = name
    def __repr__(self): return f"{self.name}"
    def __hash__(self): return hash(self.name)
    def __eq__(self, other): return isinstance(other, Var) and self.name == other.name

class Term:
    def __init__(self, name, args=None):
        self.name = name
        self.args = args or[]
        
    def __repr__(self):
        # 修正：支援陣列 (List) 的美化輸出
        if self.name == '[]': return "[]"
        if self.name == '[|]':
            elems =[]
            curr = self
            while isinstance(curr, Term) and curr.name == '[|]':
                elems.append(str(curr.args[0]))
                curr = curr.args[1]
            if isinstance(curr, Term) and curr.name == '[]':
                return "[" + ", ".join(elems) + "]"
            else:
                return "[" + ", ".join(elems) + "|" + str(curr) + "]"
                
        if not self.args: return str(self.name)
        return f"{self.name}({', '.join(map(str, self.args))})"
        
    def __eq__(self, other):
        return isinstance(other, Term) and self.name == other.name and self.args == other.args

class Clause:
    def __init__(self, head, body=None):
        self.head = head
        self.body = body or[]

# --- 解析器 ---
class Parser:
    def __init__(self, text):
        text = re.sub(r'%.*', '', text)
        # 修正：正規表達式加入 \[ | \] | \| 來捕捉陣列符號
        self.tokens = re.findall(r"'[^']*'|[A-Z_][a-zA-Z0-9_]*|[a-z][a-zA-Z0-9_]*|\d+|:-|\\=|\(|\)|\[|\]|\||,|\.|>=|=<|<=|>|<|=|\+|-|\*|/", text)
        self.pos = 0

    def next_token(self):
        if self.pos < len(self.tokens):
            t = self.tokens[self.pos]; self.pos += 1
            return t
        return None

    def peek(self):
        return self.tokens[self.pos] if self.pos < len(self.tokens) else None

    def parse_term(self):
        token = self.next_token()
        if token is None: return None
        
        # 修正：加入陣列 (List) 解析邏輯
        if token == '[':
            if self.peek() == ']':
                self.next_token() # skip ']'
                return Term('[]')
            
            elements =[]
            while True:
                elements.append(self.parse_term())
                if self.peek() == ',':
                    self.next_token() # skip ','
                else:
                    break
            
            tail = Term('[]')
            if self.peek() == '|':
                self.next_token() # skip '|'
                tail = self.parse_term()
                
            if self.peek() == ']':
                self.next_token() # skip ']'
                
            # 將 [a, b | Tail] 轉換為 Prolog 內部結構 [|](a, [|](b, Tail))
            res = tail
            for e in reversed(elements):
                res = Term('[|]', [e, res])
            return res

        is_var = False
        if token.startswith("'") and token.endswith("'"):
            token = token[1:-1]
        elif (token[0].isupper() or token == '_') and not token.isnumeric():
            is_var = True

        if is_var:
            return Var(token)

        if self.peek() == '(':
            self.next_token() # skip '('
            args =[]
            while True:
                args.append(self.parse_term())
                if self.peek() == ')': break
                if self.next_token() != ',': break
            self.next_token() # skip ')'
            return Term(token, args)
        return Term(token)

    def parse_clause(self):
        left = self.parse_term()
        if self.peek() == ':-':
            self.next_token()
            body =[]
            while True:
                body.append(self.parse_clause_item())
                if self.peek() == '.': break
                if self.next_token() != ',': break
            self.next_token() # skip '.'
            return Clause(left, body)
        if self.peek() == '.': self.next_token()
        return Clause(left)

    def parse_clause_item(self):
        left = self.parse_term()
        if self.peek() in['>', '<', 'is', '=', '>=', '<=', '=<', '\\=']:
            op = self.next_token()
            right = self.parse_expression()
            return Term(op, [left, right])
        return left

    def parse_expression(self):
        left = self.parse_term()
        if self.peek() in ['+', '-', '*', '/']:
            op = self.next_token()
            right = self.parse_expression()
            return Term(op, [left, right])
        return left

    def parse_all(self):
        clauses =[]
        while self.pos < len(self.tokens):
            clauses.append(self.parse_clause())
        return clauses

# --- 執行引擎 ---
def eval_expr(expr, subst):
    expr = resolve(expr, subst)
    if isinstance(expr, Term):
        if not expr.args:
            try: return int(expr.name)
            except: return expr.name
        args = [eval_expr(a, subst) for a in expr.args]
        ops = {'+': lambda a,b: a+b, '-': lambda a,b: a-b, '*': lambda a,b: a*b, '/': lambda a,b: a//b}
        if expr.name in ops: return ops[expr.name](args[0], args[1])
    try: return int(str(expr))
    except: return str(expr)

def unify(x, y, subst):
    subst = subst.copy()
    x, y = resolve(x, subst), resolve(y, subst)
    
    if x == y: return subst 
    
    if isinstance(x, Var) and x.name == '_': return subst
    if isinstance(y, Var) and y.name == '_': return subst
    
    if isinstance(x, Var):
        subst[x.name] = y; return subst
    if isinstance(y, Var):
        subst[y.name] = x; return subst
        
    if isinstance(x, Term) and isinstance(y, Term):
        if x.name != y.name or len(x.args) != len(y.args): return None
        for a, b in zip(x.args, y.args):
            subst = unify(a, b, subst)
            if subst is None: return None
        return subst
    return None

def resolve(x, subst):
    while isinstance(x, Var) and x.name in subst: 
        x = subst[x.name]
    # 修正：加入遞迴解開參數，確保陣列內部的變數在輸出時也能替換為真實的值
    if isinstance(x, Term):
        return Term(x.name, [resolve(a, subst) for a in x.args])
    return x

def solve(goals, clauses, subst, level=0):
    if not goals: yield subst; return
    first, rest = goals[0], goals[1:]

    if isinstance(first, Term):
        if first.name == 'write':
            val = resolve(first.args[0], subst)
            print(val, end='')
            yield from solve(rest, clauses, subst, level + 1)
            return
        if first.name == 'nl':
            print()
            yield from solve(rest, clauses, subst, level + 1)
            return
        if first.name == '\\=':
            if unify(first.args[0], first.args[1], subst) is None:
                yield from solve(rest, clauses, subst, level + 1)
            return
        if first.name == 'is':
            val = eval_expr(first.args[1], subst)
            new_subst = unify(first.args[0], Term(str(val)), subst)
            if new_subst is not None: yield from solve(rest, clauses, new_subst, level + 1)
            return
        if first.name in ['>', '<', '>=', '<=', '=<']:
            v1, v2 = eval_expr(first.args[0], subst), eval_expr(first.args[1], subst)
            check = {'>': v1 > v2, '<': v1 < v2, '>=': v1 >= v2, '<=': v1 <= v2, '=<': v1 <= v2}
            if check.get(first.name): yield from solve(rest, clauses, subst, level + 1)
            return
        if first.name == 'append' and len(first.args) == 3:
            a1 = resolve(first.args[0], subst)
            a2 = resolve(first.args[1], subst)
            a3 = resolve(first.args[2], subst)
            # append([], L, L)
            s = unify(a1, Term('[]'), subst)
            if s is not None:
                s2 = unify(a2, a3, s)
                if s2 is not None:
                    yield from solve(rest, clauses, s2, level + 1)
            # append([H|T], L, [H|R]) :- append(T, L, R)
            h = Var(f"_AH{level}")
            t = Var(f"_AT{level}")
            r = Var(f"_AR{level}")
            s = unify(a1, Term('[|]', [h, t]), subst)
            if s is not None:
                s2 = unify(a3, Term('[|]', [h, r]), s)
                if s2 is not None:
                    new_goal = Term('append', [t, a2, r])
                    yield from solve([new_goal] + rest, clauses, s2, level + 1)
            return

    for clause in clauses:
        mapping = {v: Var(f"{v}_{level}") for v in get_vars(clause)}
        head, body = rename_vars(clause.head, mapping),[rename_vars(b, mapping) for b in clause.body]
        new_subst = unify(first, head, subst)
        if new_subst is not None:
            yield from solve(body + rest, clauses, new_subst, level + 1)

def get_vars(obj):
    res = set()
    if isinstance(obj, Clause):
        res.update(get_vars(obj.head)); [res.update(get_vars(b)) for b in obj.body]
    elif isinstance(obj, Term): [res.update(get_vars(a)) for a in obj.args]
    elif isinstance(obj, Var): 
        if obj.name != '_': res.add(obj.name)
    return res

def rename_vars(term, mapping):
    if isinstance(term, Var): return mapping.get(term.name, term)
    if isinstance(term, Term): return Term(term.name, [rename_vars(a, mapping) for a in term.args])
    return term

def main():
    if len(sys.argv) < 2: return
    with open(sys.argv[1], 'r') as f: db = Parser(f.read()).parse_all()
    print(f"Loaded {len(db)} clauses.")
    while True:
        try:
            line = input("?- ").strip()
            if not line or line in ["exit", "quit"]: break
            if not line.endswith('.'): line += '.'
            goals =[Parser(line).parse_clause_item()]
            found = False
            for sol in solve(goals, db, {}):
                found = True
                q_vars = get_vars(goals[0])
                res = ", ".join([f"{v} = {resolve(Var(v), sol)}" for v in q_vars])
                if res: print(f" ({res})")
                else: print("true.")
            if not found: print("false.")
        except Exception as e: print(f"Error: {e}")

if __name__ == "__main__": main()