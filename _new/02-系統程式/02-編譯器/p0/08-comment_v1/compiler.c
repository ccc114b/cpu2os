#include <stdio.h>
#include <ctype.h>
#include <string.h>
#include <stdlib.h>

// =========================================================
// 1. 中間碼 (Quadruples) 資料結構
// 原理：四元組是中間碼的一種，格式為 (Op, Arg1, Arg2, Result)。
// 它將複雜的運算式拆解為最簡單的指令，方便虛擬機循序執行或進行後續優化。
// =========================================================
typedef struct {
    char op[16];     // 操作碼 (例如: ADD, CALL, JMP_F)
    char arg1[32];   // 第一個運算元
    char arg2[32];   // 第二個運算元
    char result[32]; // 存放結果的標記或跳轉地址
} Quad;

Quad quads[1000];    // 存放編譯出來的所有四元組
int quad_count = 0;  // 紀錄四元組的數量（也作為 PC 指令地址）

// 生成中間碼並印出
void emit(const char* op, const char* a1, const char* a2, const char* res) {
    strcpy(quads[quad_count].op, op);
    strcpy(quads[quad_count].arg1, a1);
    strcpy(quads[quad_count].arg2, a2);
    strcpy(quads[quad_count].result, res);
    printf("%03d: %-10s %-10s %-10s %-10s\n", quad_count, op, a1, a2, res);
    quad_count++;
}

// =========================================================
// 2. 詞法分析 (Lexer)
// 原理：將原始字串（Stream of Characters）切分成標記（Tokens）。
// 它會自動過濾空白與註解，並辨識關鍵字（func, if, return）或識別碼（ID）。
// =========================================================
typedef enum {
    TK_FUNC, TK_RETURN, TK_IF, TK_ID, TK_NUM, 
    TK_LPAREN, TK_RPAREN, TK_LBRACE, TK_RBRACE, TK_COMMA, TK_SEMICOLON,
    TK_ASSIGN, TK_PLUS, TK_MINUS, TK_MUL, TK_DIV, 
    TK_EQ, TK_LT, TK_GT, TK_EOF
} TokenType;

typedef struct { TokenType type; char text[32]; } Token;
Token cur_token;     // 當前讀取到的 Token
const char *src;     // 指向原始碼字串的指針

void next_token() {
    while (1) {
        while (isspace(*src)) src++; // 忽略空格、換行
        if (*src == '\0') { cur_token.type = TK_EOF; return; }

        // 註解處理邏輯：跳過 // 或 /* ... */ 之間的內容
        // EBNF: comment = "//" { all_characters } newline
        //               | "/*" { all_characters } "*/" ;

        if (*src == '/') {
            if (*(src + 1) == '/') { 
                src += 2; while (*src != '\n' && *src != '\0') src++; continue; 
            } else if (*(src + 1) == '*') {
                src += 2; 
                while (!(*src == '*' && *(src + 1) == '/') && *src != '\0') src++;
                if (*src != '\0') src += 2; continue; 
            }
        }
        break;
    }

    // 辨識數字 (NUM)
    if (isdigit(*src)) { // EBNF: number = digit { digit } ;
        int i = 0; while (isdigit(*src)) cur_token.text[i++] = *src++;
        cur_token.text[i] = '\0'; cur_token.type = TK_NUM;
    } 
    // 辨識識別碼 (ID) 與 關鍵字 (Keyword)
    else if (isalpha(*src) || *src == '_') { // EBNF: identifier    = letter { letter | digit | "_" } ;
        int i = 0; while (isalnum(*src) || *src == '_') cur_token.text[i++] = *src++;
        cur_token.text[i] = '\0';
        if (strcmp(cur_token.text, "func") == 0) cur_token.type = TK_FUNC;
        else if (strcmp(cur_token.text, "return") == 0) cur_token.type = TK_RETURN;
        else if (strcmp(cur_token.text, "if") == 0) cur_token.type = TK_IF;
        else cur_token.type = TK_ID;
    } 
    // 辨識運算符與符號
    else {
        cur_token.text[0] = *src; cur_token.text[1] = '\0';
        switch (*src++) {
            case '(': cur_token.type = TK_LPAREN; break;
            case ')': cur_token.type = TK_RPAREN; break;
            case '{': cur_token.type = TK_LBRACE; break;
            case '}': cur_token.type = TK_RBRACE; break;
            case '+': cur_token.type = TK_PLUS; break;
            case '-': cur_token.type = TK_MINUS; break;
            case '*': cur_token.type = TK_MUL; break;
            case '/': cur_token.type = TK_DIV; break;
            case ',': cur_token.type = TK_COMMA; break;
            case ';': cur_token.type = TK_SEMICOLON; break;
            case '<': cur_token.type = TK_LT; break;
            case '>': cur_token.type = TK_GT; break;
            case '=':
                if (*src == '=') { src++; cur_token.type = TK_EQ; strcpy(cur_token.text, "=="); }
                else cur_token.type = TK_ASSIGN; break;
        }
    }
}

// =========================================================
// 3. 語法解析 (Parser) - 遞迴下降法
// 原理：根據語法規則，將 Token 組合成邏輯結構。
// 同時生成「暫存變數 (t1, t2...)」來輔助運算式的扁平化。
// =========================================================
int t_idx = 0;
void new_t(char *s) { sprintf(s, "t%d", ++t_idx); } // 生成唯一暫存變數名
void expression(char *res);
void statement();

// 處理最基本的單元：數字、變數、或函數呼叫
/* EBNF: factor = number
              | identifier [ "(" [ argument_list ] ")" ]
              | "(" expression ")" ;
*/
void factor(char *res) {
    if (cur_token.type == TK_NUM) { // EBNF: number
        new_t(res); emit("IMM", cur_token.text, "-", res); next_token();
    } else if (cur_token.type == TK_ID) { // EBNF: identifier [ "(" [ argument_list ] ")" ]
        char name[32]; strcpy(name, cur_token.text); next_token();
        if (cur_token.type == TK_LPAREN) { // 處理函數呼叫 add(1, 2)
            next_token(); int count = 0;
            while (cur_token.type != TK_RPAREN) {
                char arg[32]; expression(arg);
                emit("PARAM", arg, "-", "-"); count++; // 參數傳遞指令
                if (cur_token.type == TK_COMMA) next_token();
            }
            next_token(); new_t(res); char c_str[10]; sprintf(c_str, "%d", count);
            emit("CALL", name, c_str, res); // 跳轉執行指令
        } else strcpy(res, name);
    } else if (cur_token.type == TK_LPAREN) {// EBNF: "(" expression ")" ;
        next_token(); expression(res); next_token();
    }
}

// 處理乘除法 (優先級高)
// term = factor { ( "*" | "/" ) factor } ;
void term(char *res) {
    char l[32], r[32], t[32]; factor(l);
    while (cur_token.type == TK_MUL || cur_token.type == TK_DIV) {
        char op[10]; strcpy(op, cur_token.type == TK_MUL ? "MUL" : "DIV");
        next_token(); factor(r); new_t(t); emit(op, l, r, t); strcpy(l, t);
    }
    strcpy(res, l);
}

// 處理加減法 (優先級中)
// EBNF: arith_expr = term { ( "+" | "-" ) term } ;
void arith_expr(char *res) {
    char l[32], r[32], t[32]; term(l);
    while (cur_token.type == TK_PLUS || cur_token.type == TK_MINUS) {
        char op[10]; strcpy(op, cur_token.type == TK_PLUS ? "ADD" : "SUB");
        next_token(); term(r); new_t(t); emit(op, l, r, t); strcpy(l, t);
    }
    strcpy(res, l);
}

// 處理邏輯判斷 (優先級低)
// EBNF: expression = arith_expr [ ( "==" | "<" | ">" ) arith_expr ] ;
void expression(char *res) {
    char l[32], r[32], t[32]; arith_expr(l);
    if (cur_token.type == TK_EQ || cur_token.type == TK_LT || cur_token.type == TK_GT) {
        char op[10];
        if (cur_token.type == TK_EQ) strcpy(op, "CMP_EQ");
        else if (cur_token.type == TK_LT) strcpy(op, "CMP_LT");
        else strcpy(op, "CMP_GT");
        next_token(); arith_expr(r); new_t(t); emit(op, l, r, t); strcpy(res, t);
    } else strcpy(res, l);
}

// 處理陳述句 (Assignment, If, Return)
// EBNF: statement = if_statement | assignment_statement | return_statement ;
void statement() {
    // EBNF: if_statement  = "if" "(" expression ")" "{" { statement } "}" ;
    if (cur_token.type == TK_IF) {
        next_token(); next_token(); char cond[32]; expression(cond);
        next_token(); next_token(); 
        int jmp_idx = quad_count; // 記下 JMP_F 的位置
        emit("JMP_F", cond, "-", "?"); // 先填問號，等解析完 If 塊再回填 (Backpatching)
        while (cur_token.type != TK_RBRACE) statement();
        next_token(); 
        sprintf(quads[jmp_idx].result, "%d", quad_count); // 回填真實跳轉地址
    // EBNF: assignment_statement = identifier "=" expression [ ";" ] ;
    } else if (cur_token.type == TK_ID) {
        char name[32]; strcpy(name, cur_token.text); next_token();
        if (cur_token.type == TK_ASSIGN) {
            next_token(); char res[32]; expression(res);
            emit("STORE", res, "-", name);
            if (cur_token.type == TK_SEMICOLON) next_token();
        }
    // EBNF: return_statement = "return" expression [ ";" ] ;
    } else if (cur_token.type == TK_RETURN) {
        next_token(); char res[32]; expression(res);
        emit("RET_VAL", res, "-", "-");
        if (cur_token.type == TK_SEMICOLON) next_token();
    }
}

// 主程式解析：區分函數定義與全域陳述句
// EBNF: program = { function_def | statement } ;
void parse_program() {
    while (cur_token.type != TK_EOF) {
        // EBNF: function_def  = "func" identifier "(" [ parameter_list ] ")" "{" { statement } "}" ;
        if (cur_token.type == TK_FUNC) {
            next_token(); char f_name[32]; strcpy(f_name, cur_token.text);
            emit("FUNC_BEG", f_name, "-", "-");
            next_token(); next_token();
            // EBNF: parameter_list = identifier { "," identifier } ;
            while (cur_token.type == TK_ID) {
                emit("FORMAL", cur_token.text, "-", "-"); next_token(); // 接收參數指令
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
// 原理：模擬 CPU 的行為。具備「堆疊幀 (Stack Frame)」來實現遞迴。
// 每一層呼叫都有獨立的變數域（Names & Values），確保變數不會衝突。
// =========================================================
typedef struct {
    char names[100][32];  // 區域變數名稱
    int values[100];      // 區域變數值
    int count;            // 目前幀的變數數量
    int ret_pc;           // 回傳後應繼續執行的 PC 位置
    char ret_var[32];     // 結果應回填給 Caller 的哪個變數
    int incoming_args[10]; // 傳進來的參數值
    int formal_idx;       // 處理參數的計數器
} Frame;

Frame stack[1000]; // 呼叫堆疊
int sp = 0;        // Stack Pointer (0 為全域環境)

// 輔助函式：從目前的 Frame 中尋找變數值
int get_var(const char *name) {
    if (isdigit(name[0])) return atoi(name); // 數字直接回傳
    for (int i = 0; i < stack[sp].count; i++) 
        if (strcmp(stack[sp].names[i], name) == 0) return stack[sp].values[i];
    return 0;
}

// 輔助函式：更新或新建目前 Frame 的變數
void set_var(const char *name, int val) {
    for (int i = 0; i < stack[sp].count; i++) 
        if (strcmp(stack[sp].names[i], name) == 0) { stack[sp].values[i] = val; return; }
    strcpy(stack[sp].names[stack[sp].count], name);
    stack[sp].values[stack[sp].count++] = val;
}

void vm() {
    int pc = 0; // Program Counter
    int param_stack[1000]; int param_sp = 0; // 暫存 CALL 傳遞過來的參數
    int func_pc[100]; char func_names[100][32]; int f_count = 0;
    
    // 預先掃描，記住所有函數的進入點位置
    for (int i = 0; i < quad_count; i++) {
        if (strcmp(quads[i].op, "FUNC_BEG") == 0) {
            strcpy(func_names[f_count], quads[i].arg1); func_pc[f_count++] = i + 1;
        }
    }

    stack[sp].count = 0;
    printf("\n=== VM 執行開始 ===\n");

    while (pc < quad_count) {
        Quad q = quads[pc];
        // 遇到函數定義段落，直接跳到結束（函數必須透過 CALL 執行）
        if (strcmp(q.op, "FUNC_BEG") == 0) {
            while (strcmp(quads[pc].op, "FUNC_END") != 0) pc++;
        }
        else if (strcmp(q.op, "IMM") == 0) set_var(q.result, atoi(q.arg1));
        else if (strcmp(q.op, "ADD") == 0) set_var(q.result, get_var(q.arg1) + get_var(q.arg2));
        else if (strcmp(q.op, "SUB") == 0) set_var(q.result, get_var(q.arg1) - get_var(q.arg2));
        else if (strcmp(q.op, "MUL") == 0) set_var(q.result, get_var(q.arg1) * get_var(q.arg2));
        else if (strcmp(q.op, "DIV") == 0) set_var(q.result, get_var(q.arg1) / get_var(q.arg2));
        else if (strcmp(q.op, "CMP_EQ") == 0) set_var(q.result, get_var(q.arg1) == get_var(q.arg2));
        else if (strcmp(q.op, "CMP_LT") == 0) set_var(q.result, get_var(q.arg1) < get_var(q.arg2));
        else if (strcmp(q.op, "CMP_GT") == 0) set_var(q.result, get_var(q.arg1) > get_var(q.arg2));
        else if (strcmp(q.op, "STORE") == 0) set_var(q.result, get_var(q.arg1));
        else if (strcmp(q.op, "JMP_F") == 0) { // 條件跳轉：如果條件為 0，PC 跳到目標位置
            if (get_var(q.arg1) == 0) { pc = atoi(q.result) - 1; }
        }
        else if (strcmp(q.op, "PARAM") == 0) {
            param_stack[param_sp++] = get_var(q.arg1); // 暫存參數
        }
        else if (strcmp(q.op, "CALL") == 0) {
            int p_count = atoi(q.arg2); int target_pc = -1;
            for (int i = 0; i < f_count; i++) 
                if (strcmp(func_names[i], q.arg1) == 0) target_pc = func_pc[i];
            
            sp++; // 開闢新的堆疊幀 (進入函數)
            stack[sp].count = 0; stack[sp].ret_pc = pc + 1;
            strcpy(stack[sp].ret_var, q.result);
            stack[sp].formal_idx = 0;
            // 從參數暫存區把值拿過來
            for(int i=0; i<p_count; i++) stack[sp].incoming_args[i] = param_stack[param_sp - p_count + i];
            param_sp -= p_count; pc = target_pc; continue;
        }
        else if (strcmp(q.op, "FORMAL") == 0) {
            set_var(q.arg1, stack[sp].incoming_args[stack[sp].formal_idx++]);
        }
        else if (strcmp(q.op, "RET_VAL") == 0) {
            int ret_val = get_var(q.arg1); int ret_address = stack[sp].ret_pc;
            char target_var[32]; strcpy(target_var, stack[sp].ret_var);
            sp--; // 銷毀當前堆疊幀 (回到 Caller)
            set_var(target_var, ret_val); // 將回傳值寫入 Caller 的變數空間
            pc = ret_address; continue;
        }
        pc++;
    }

    // 執行結束後印出結果
    printf("=== VM 執行完畢 ===\n\n全域變數結果:\n");
    for (int i = 0; i < stack[0].count; i++) {
        if (stack[0].names[i][0] != 't')
            printf(">> %s = %d\n", stack[0].names[i], stack[0].values[i]);
    }
}

// =========================================================
// 讀取檔案與主程式
// =========================================================
int main(int argc, char *argv[]) {
    if (argc < 2) {
        printf("用法: %s <source_file>\n", argv[0]);
        return 1;
    }

    FILE *f = fopen(argv[1], "rb");
    if (!f) { printf("無法開啟檔案: %s\n", argv[1]); return 1; }
    fseek(f, 0, SEEK_END);
    long length = ftell(f);
    fseek(f, 0, SEEK_SET);
    char *buffer = malloc(length + 1);
    fread(buffer, 1, length, f);
    buffer[length] = '\0';
    fclose(f);

    src = buffer;
    printf("編譯器生成的中間碼 (PC: Quadruples):\n");
    printf("--------------------------------------------\n");
    
    next_token();
    parse_program();
    vm();
    
    free(buffer);
    return 0;
}