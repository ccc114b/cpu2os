import sys
import re
import os

class MiniBasic:
    def __init__(self):
        self.lines = {}
        self.vars = {}
        self.line_numbers = []
        self.pc = 0

    def load_file(self, filename):
        with open(filename, encoding="utf-8") as f:
            for line in f:
                line = line.strip()
                if not line:
                    continue
                m = re.match(r'^(\d+)\s+(.*)$', line)
                if m:
                    num = int(m.group(1))
                    stmt = m.group(2).strip()
                    self.lines[num] = stmt

        self.line_numbers = sorted(self.lines.keys())

    def evaluate(self, expr):

        temp = expr

        for v in self.vars:
            temp = re.sub(rf'\b{v}\b', str(self.vars[v]), temp)

        temp = temp.replace("<>", "!=")
        temp = re.sub(r'(?<![<>!=])=(?!=)', '==', temp)

        try:
            return eval(temp, {"__builtins__": {}}, {
                "INT": lambda x: int(x)
            })
        except:
            return expr

    def exec_statement(self, stmt):

        # LET
        m = re.match(r'^(?:LET\s+)?([A-Z][A-Z0-9]*)\s*=\s*(.*)$', stmt, re.I)
        if m:
            var, expr = m.groups()
            self.vars[var] = self.evaluate(expr)
            return

        # PRINT
        if stmt.upper().startswith("PRINT"):
            parts = stmt[5:].split(',')
            out = []
            for p in parts:
                p = p.strip()
                if p.startswith('"') and p.endswith('"'):
                    out.append(p[1:-1])
                else:
                    out.append(str(self.evaluate(p)))
            print(" ".join(out))
            return

        # INPUT
        if stmt.upper().startswith("INPUT"):
            var = stmt[5:].strip()
            val = input("? ")
            try:
                val = int(val)
            except:
                pass
            self.vars[var] = val
            return

        # GOTO
        if stmt.upper().startswith("GOTO"):
            target = int(stmt[4:].strip())
            self.pc = self.line_numbers.index(target)
            return "jump"

        # END
        if stmt.upper() == "END":
            sys.exit(0)

    def run(self):

        while self.pc < len(self.line_numbers):

            lnum = self.line_numbers[self.pc]
            stmt = self.lines[lnum]

            # IF
            m = re.match(r'IF\s+(.*?)\s+THEN\s+(.*)', stmt, re.I)
            if m:
                cond, action = m.groups()

                if self.evaluate(cond):

                    # THEN number
                    if re.fullmatch(r'\d+', action):
                        self.pc = self.line_numbers.index(int(action))
                        continue

                    # THEN statement
                    res = self.exec_statement(action)
                    if res == "jump":
                        continue

                self.pc += 1
                continue

            # normal statement
            res = self.exec_statement(stmt)
            if res == "jump":
                continue

            self.pc += 1


if __name__ == "__main__":

    if len(sys.argv) < 2:
        print("usage: python basic.py program.bas")
        sys.exit(1)

    basic = MiniBasic()
    basic.load_file(sys.argv[1])
    basic.run()