#include <stdio.h>
#include <ctype.h>
#include <string.h>
#include <stdlib.h>

// =========================================================
// 1. 中間碼 (Quadruples) 資料結構
// =========================================================
typedef struct {
    char op[16];
    char arg1[32];
    char arg2[32];
    char result[32];
} Quad;

Quad quads[1000];
int quad_count = 0;

void emit(const char* op, const char* a1, const char* a2, const char* res) {
    strcpy(quads[quad_count].op, op);
    strcpy(quads[quad_count].arg1, a1);
    strcpy(quads[quad_count].arg2, a2);
    strcpy(quads[quad_count].result, res);
    printf("%02d: %-10s %-10s %-10s %-10s\n", quad_count, op, a1, a2, res);
    quad_count++;
}

// =========================================================
// 2. 詞法分析 (Lexer)
// =========================================================
typedef enum {
    TK_FUNC, TK_RETURN, TK_ID, TK_NUM, TK_LPAREN, TK_RPAREN, 
    TK_LBRACE, TK_RBRACE, TK_COMMA, TK_ASSIGN, TK_SEMICOLON,
    TK_PLUS, TK_MINUS, TK_MUL, TK_DIV, TK_EOF
} TokenType;

typedef struct { TokenType type; char text[32]; } Token;
Token cur_token;
const char *src;

void next_token() {
    while (isspace(*src)) src++;
    if (*src == '\0') { cur_token.type = TK_EOF; return; }

    if (isdigit(*src)) {
        int i = 0; while (isdigit(*src)) cur_token.text[i++] = *src++;
        cur_token.text[i] = '\0'; cur_token.type = TK_NUM;
    } else if (isalpha(*src) || *src == '_') {
        int i = 0; while (isalnum(*src) || *src == '_') cur_token.text[i++] = *src++;
        cur_token.text[i] = '\0';
        if (strcmp(cur_token.text, "func") == 0) cur_token.type = TK_FUNC;
        else if (strcmp(cur_token.text, "return") == 0) cur_token.type = TK_RETURN;
        else cur_token.type = TK_ID;
    } else {
        cur_token.text[0] = *src; cur_token.text[1] = '\0';
        switch (*src++) {
            case '(': cur_token.type = TK_LPAREN; break;
            case ')': cur_token.type = TK_RPAREN; break;
            case '{': cur_token.type = TK_LBRACE; break;
            case '}': cur_token.type = TK_RBRACE; break;
            case '=': cur_token.type = TK_ASSIGN; break;
            case '+': cur_token.type = TK_PLUS; break;
            case '-': cur_token.type = TK_MINUS; break;
            case '*': cur_token.type = TK_MUL; break;
            case '/': cur_token.type = TK_DIV; break;
            case ',': cur_token.type = TK_COMMA; break;
            case ';': cur_token.type = TK_SEMICOLON; break;
        }
    }
}

// =========================================================
// 3. 語法解析 (Parser)
// =========================================================
int t_idx = 0;
void new_t(char *s) { sprintf(s, "t%d", ++t_idx); }
void expression(char *res);

void factor(char *res) {
    if (cur_token.type == TK_NUM) {
        new_t(res); emit("IMM", cur_token.text, "-", res);
        next_token();
    } else if (cur_token.type == TK_ID) {
        char name[32]; strcpy(name, cur_token.text);
        next_token();
        if (cur_token.type == TK_LPAREN) { // Function Call
            next_token();
            int count = 0;
            while (cur_token.type != TK_RPAREN) {
                char arg[32]; expression(arg);
                emit("PARAM", arg, "-", "-"); count++;
                if (cur_token.type == TK_COMMA) next_token();
            }
            next_token();
            new_t(res); char c_str[10]; sprintf(c_str, "%d", count);
            emit("CALL", name, c_str, res);
        } else strcpy(res, name); // Variable
    } else if (cur_token.type == TK_LPAREN) {
        next_token(); expression(res); next_token();
    }
}

void term(char *res) {
    char l[32], r[32], t[32]; factor(l);
    while (cur_token.type == TK_MUL || cur_token.type == TK_DIV) {
        char op[10]; strcpy(op, cur_token.type == TK_MUL ? "MUL" : "DIV");
        next_token(); factor(r); new_t(t); emit(op, l, r, t); strcpy(l, t);
    }
    strcpy(res, l);
}

void expression(char *res) {
    char l[32], r[32], t[32]; term(l);
    while (cur_token.type == TK_PLUS || cur_token.type == TK_MINUS) {
        char op[10]; strcpy(op, cur_token.type == TK_PLUS ? "ADD" : "SUB");
        next_token(); term(r); new_t(t); emit(op, l, r, t); strcpy(l, t);
    }
    strcpy(res, l);
}

void statement() {
    if (cur_token.type == TK_ID) {
        char name[32]; strcpy(name, cur_token.text);
        next_token();
        if (cur_token.type == TK_ASSIGN) {
            next_token(); char res[32]; expression(res);
            emit("STORE", res, "-", name);
            if(cur_token.type == TK_SEMICOLON) next_token();
        }
    } else if (cur_token.type == TK_RETURN) {
        next_token(); char res[32]; expression(res);
        emit("RET_VAL", res, "-", "-");
        if(cur_token.type == TK_SEMICOLON) next_token();
    }
}

void parse_program() {
    while (cur_token.type != TK_EOF) {
        if (cur_token.type == TK_FUNC) {
            next_token(); char f_name[32]; strcpy(f_name, cur_token.text);
            emit("FUNC_BEG", f_name, "-", "-");
            next_token(); next_token();
            while (cur_token.type == TK_ID) {
                emit("FORMAL", cur_token.text, "-", "-"); next_token();
                if (cur_token.type == TK_COMMA) next_token();
            }
            next_token(); next_token();
            while (cur_token.type != TK_RBRACE) statement();
            emit("FUNC_END", f_name, "-", "-"); next_token();
        } else statement();
    }
}

// =========================================================
// 4. 虛擬機 (Virtual Machine)
// =========================================================
typedef struct {
    char names[100][32]; // 變數名稱
    int values[100];     // 變數數值
    int count;           // 當前變數數量
    int ret_pc;          // 呼叫完畢後要跳回的 PC
    char ret_var[32];    // 存放回傳值的變數名稱
} Frame;

Frame stack[100]; // 呼叫堆疊
int sp = 0;       // 堆疊指標 (0 是全域作用域)

// 從當前堆疊幀取得變數值
int get_var(const char *name) {
    if (isdigit(name[0])) return atoi(name); // 如果是數字直接轉型
    for (int i = 0; i < stack[sp].count; i++) {
        if (strcmp(stack[sp].names[i], name) == 0) return stack[sp].values[i];
    }
    return 0; // 找不到預設為 0
}

// 設定變數值到當前堆疊幀
void set_var(const char *name, int val) {
    for (int i = 0; i < stack[sp].count; i++) {
        if (strcmp(stack[sp].names[i], name) == 0) {
            stack[sp].values[i] = val; return;
        }
    }
    // 新增變數
    strcpy(stack[sp].names[stack[sp].count], name);
    stack[sp].values[stack[sp].count] = val;
    stack[sp].count++;
}

// 執行四元組的虛擬機
void vm() {
    int pc = 0;             // 程式計數器 Program Counter
    int param_buf[10];      // 傳遞參數的暫存區
    int param_count = 0, formal_idx = 0;
    int ret_val_reg = 0;    // 模擬 CPU 的回傳值暫存器
    
    // 預先掃描：記錄函數的進入點 (pc)
    int func_pc[100];
    char func_names[100][32];
    int f_count = 0;
    for (int i = 0; i < quad_count; i++) {
        if (strcmp(quads[i].op, "FUNC_BEG") == 0) {
            strcpy(func_names[f_count], quads[i].arg1);
            func_pc[f_count++] = i + 1; // 進入點是 FUNC_BEG 的下一行
        }
    }

    stack[sp].count = 0; // 初始化全域環境
    printf("\n=== VM 執行開始 ===\n");

    while (pc < quad_count) {
        Quad q = quads[pc];

        // 遇到函數定義，直接跳過（只有 CALL 的時候才進去執行）
        if (strcmp(q.op, "FUNC_BEG") == 0) {
            while (strcmp(quads[pc].op, "FUNC_END") != 0) pc++;
            pc++; continue;
        }

        if (strcmp(q.op, "IMM") == 0) set_var(q.result, atoi(q.arg1));
        else if (strcmp(q.op, "ADD") == 0) set_var(q.result, get_var(q.arg1) + get_var(q.arg2));
        else if (strcmp(q.op, "SUB") == 0) set_var(q.result, get_var(q.arg1) - get_var(q.arg2));
        else if (strcmp(q.op, "MUL") == 0) set_var(q.result, get_var(q.arg1) * get_var(q.arg2));
        else if (strcmp(q.op, "DIV") == 0) set_var(q.result, get_var(q.arg1) / get_var(q.arg2));
        else if (strcmp(q.op, "STORE") == 0) set_var(q.result, get_var(q.arg1));
        else if (strcmp(q.op, "PARAM") == 0) {
            param_buf[param_count++] = get_var(q.arg1);
        }
        else if (strcmp(q.op, "CALL") == 0) {
            // 尋找函數進入點
            int target_pc = -1;
            for (int i = 0; i < f_count; i++) {
                if (strcmp(func_names[i], q.arg1) == 0) { target_pc = func_pc[i]; break; }
            }
            // Push Stack (建立新的函數作用域)
            sp++;
            stack[sp].count = 0;
            stack[sp].ret_pc = pc + 1;         // 記住回來的位置
            strcpy(stack[sp].ret_var, q.result); // 記住回傳值要塞給誰
            formal_idx = 0;
            pc = target_pc; // 執行跳轉
            continue;
        }
        else if (strcmp(q.op, "FORMAL") == 0) {
            set_var(q.arg1, param_buf[formal_idx++]);
            if (formal_idx == param_count) param_count = 0; // 參數領取完畢
        }
        else if (strcmp(q.op, "RET_VAL") == 0) {
            ret_val_reg = get_var(q.arg1); // 把結果放進虛擬暫存器
        }
        else if (strcmp(q.op, "FUNC_END") == 0) {
            // Pop Stack (銷毀局部作用域並返回)
            int ret_address = stack[sp].ret_pc;
            char target_var[32]; strcpy(target_var, stack[sp].ret_var);
            sp--; // 回到上一層 (caller)
            set_var(target_var, ret_val_reg); // 寫入結果
            pc = ret_address;
            continue;
        }
        pc++;
    }

    printf("=== VM 執行完畢 ===\n\n全域變數結果:\n");
    for (int i = 0; i < stack[0].count; i++) {
        // 過濾掉 t1, t2 等編譯器生成的暫存變數
        if (stack[0].names[i][0] != 't') {
            printf(">> %s = %d\n", stack[0].names[i], stack[0].values[i]);
        }
    }
}

// =========================================================
// 主程式
// =========================================================
int main() {
    char code[] = "func add(a, b) { sum = a + b; return sum; } \n"
                  "x = add(10, 20); \n"
                  "y = x * 2;";
    src = code;
    printf("編譯原始碼:\n%s\n\n", code);
    printf("編譯器生成的中間碼 (PC: Quadruples):\n");
    printf("--------------------------------------------\n");
    
    next_token();
    parse_program(); // 產生四元組
    
    vm(); // 執行虛擬機
    
    return 0;
}