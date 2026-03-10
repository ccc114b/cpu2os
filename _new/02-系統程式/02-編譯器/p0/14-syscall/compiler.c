/*
 * =========================================================
 * 簡易編譯器 - C 語言實現
 * 程式語言 EBNF (Extended Backus-Naur Form) 語法定義
 * =========================================================
 * program ::= ( function_def | statement )*
 *
 * function_def ::= "func" id "(" [ id ("," id)* ] ")" "{" statement* "}"
 *
 * statement ::= "if" "(" expression ")" "{" statement* "}"[ "else" "{" statement* "}" ]
 *             | "while" "(" expression ")" "{" statement* "}"
 *             | "for" "(" [expr_or_assign] ";" [expression] ";"[expr_or_assign] ")" "{" statement* "}"
 *             | "break" ";"
 *             | "continue" ";"
 *             | "return" expression ";"
 *             | expr_or_assign ";"
 * =========================================================
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <time.h>

#define MAX_TOKEN_LEN 256
#define MAX_QUADS 10000
#define MAX_STRING_POOL 1000
#define MAX_LOOP_STACK 100
#define MAX_CALL_STACK 100
#define MAX_VARS 1000
#define MAX_PARAMS 100
#define MAX_FUNC_MAP 100

// =========================================================
// 1. Token Types (詞彙標記類型)
// =========================================================
typedef enum {
    // 關鍵字
    TK_FUNC, TK_RETURN, TK_IF, TK_ELSE, TK_WHILE, TK_FOR,
    TK_BREAK, TK_CONTINUE,
    // 識別碼、常數
    TK_ID, TK_NUM, TK_STRING,
    // 符號
    TK_LPAREN, TK_RPAREN,       // ()
    TK_LBRACE, TK_RBRACE,       // {}
    TK_LBRACKET, TK_RBRACKET,   // []
    TK_DOT, TK_COLON,           // . :
    TK_COMMA, TK_SEMICOLON,     // , ;
    // 運算子
    TK_ASSIGN, TK_PLUS, TK_MINUS, TK_MUL, TK_DIV,
    TK_EQ, TK_LT, TK_GT,
    TK_EOF
} TokenType;

typedef struct {
    TokenType type;
    char text[MAX_TOKEN_LEN];
    int pos;
} Token;

// =========================================================
// 2. Quad (四位組中間碼)
// =========================================================
typedef struct {
    char op[16];
    char arg1[MAX_TOKEN_LEN];
    char arg2[MAX_TOKEN_LEN];
    char result[MAX_TOKEN_LEN];
} Quad;

// =========================================================
// 3. Value (動態值類型)
// =========================================================
typedef enum {
    VAL_INT,
    VAL_STRING,
    VAL_ARRAY,
    VAL_DICT
} ValueType;

typedef struct Value {
    ValueType type;
    union {
        int int_val;
        char* str_val;
        struct {
            struct Value** items;
            int size;
            int capacity;
        } arr;
        struct {
            char** keys;
            struct Value** values;
            int size;
            int capacity;
        } dict;
    } data;
} Value;

// =========================================================
// 4. Frame (函式堆疊框架)
// =========================================================
typedef struct {
    Value* vars[MAX_VARS];
    char var_names[MAX_VARS][MAX_TOKEN_LEN];
    int var_count;
    int ret_pc;
    char ret_var[MAX_TOKEN_LEN];
    Value* incoming_args[MAX_PARAMS];
    int incoming_args_count;
    int formal_idx;
} Frame;

// =========================================================
// 全域變數
// =========================================================
char* source_code;
int src_pos = 0;
Token cur_token;

Quad quads[MAX_QUADS];
int quad_count = 0;

char* string_pool[MAX_STRING_POOL];
int string_pool_count = 0;

int temp_count = 0;

// Loop stack for break/continue
typedef struct {
    int break_list[100];
    int break_count;
    int continue_target;
} LoopContext;

LoopContext loop_stack[MAX_LOOP_STACK];
int loop_stack_count = 0;

// VM 相關
Frame call_stack[MAX_CALL_STACK];
int sp = 0;

// 函數映射表
typedef struct {
    char name[MAX_TOKEN_LEN];
    int pc;
} FuncEntry;

FuncEntry func_map[MAX_FUNC_MAP];
int func_map_count = 0;

// =========================================================
// 錯誤報告
// =========================================================
void report_error(int pos, const char* msg) {
    int line = 1, col = 1;
    for (int i = 0; i < pos && source_code[i]; i++) {
        if (source_code[i] == '\n') {
            line++;
            col = 1;
        } else {
            col++;
        }
    }
    
    printf("\n❌ [語法錯誤] 第 %d 行, 第 %d 字元: %s\n", line, col, msg);
    
    // 找到錯誤所在行
    int line_start = pos;
    while (line_start > 0 && source_code[line_start - 1] != '\n') line_start--;
    int line_end = pos;
    while (source_code[line_end] && source_code[line_end] != '\n') line_end++;
    
    printf("  ");
    for (int i = line_start; i < line_end; i++) putchar(source_code[i]);
    printf("\n  ");
    for (int i = line_start; i < pos; i++) {
        putchar((source_code[i] == '\t') ? '\t' : ' ');
    }
    printf("^\n");
    exit(1);
}

// =========================================================
// Value 操作函數
// =========================================================
Value* create_int(int val) {
    Value* v = (Value*)malloc(sizeof(Value));
    v->type = VAL_INT;
    v->data.int_val = val;
    return v;
}

Value* create_string(const char* str) {
    Value* v = (Value*)malloc(sizeof(Value));
    v->type = VAL_STRING;
    v->data.str_val = strdup(str);
    return v;
}

Value* create_array() {
    Value* v = (Value*)malloc(sizeof(Value));
    v->type = VAL_ARRAY;
    v->data.arr.items = NULL;
    v->data.arr.size = 0;
    v->data.arr.capacity = 0;
    return v;
}

Value* create_dict() {
    Value* v = (Value*)malloc(sizeof(Value));
    v->type = VAL_DICT;
    v->data.dict.keys = NULL;
    v->data.dict.values = NULL;
    v->data.dict.size = 0;
    v->data.dict.capacity = 0;
    return v;
}

void array_append(Value* arr, Value* item) {
    if (arr->data.arr.size >= arr->data.arr.capacity) {
        int new_cap = (arr->data.arr.capacity == 0) ? 8 : arr->data.arr.capacity * 2;
        arr->data.arr.items = (Value**)realloc(arr->data.arr.items, new_cap * sizeof(Value*));
        arr->data.arr.capacity = new_cap;
    }
    arr->data.arr.items[arr->data.arr.size++] = item;
}

void dict_set(Value* dict, const char* key, Value* val) {
    // 檢查鍵是否存在
    for (int i = 0; i < dict->data.dict.size; i++) {
        if (strcmp(dict->data.dict.keys[i], key) == 0) {
            dict->data.dict.values[i] = val;
            return;
        }
    }
    
    // 新增鍵值對
    if (dict->data.dict.size >= dict->data.dict.capacity) {
        int new_cap = (dict->data.dict.capacity == 0) ? 8 : dict->data.dict.capacity * 2;
        dict->data.dict.keys = (char**)realloc(dict->data.dict.keys, new_cap * sizeof(char*));
        dict->data.dict.values = (Value**)realloc(dict->data.dict.values, new_cap * sizeof(Value*));
        dict->data.dict.capacity = new_cap;
    }
    dict->data.dict.keys[dict->data.dict.size] = strdup(key);
    dict->data.dict.values[dict->data.dict.size] = val;
    dict->data.dict.size++;
}

Value* dict_get(Value* dict, const char* key) {
    for (int i = 0; i < dict->data.dict.size; i++) {
        if (strcmp(dict->data.dict.keys[i], key) == 0) {
            return dict->data.dict.values[i];
        }
    }
    return create_int(0);
}

// =========================================================
// Lexer (詞法分析器)
// =========================================================
void next_token() {
    // 跳過空白和註解
    while (1) {
        while (source_code[src_pos] && isspace(source_code[src_pos])) src_pos++;
        
        if (!source_code[src_pos]) {
            cur_token.type = TK_EOF;
            cur_token.text[0] = '\0';
            cur_token.pos = src_pos;
            return;
        }
        
        // 單行註解 //
        if (source_code[src_pos] == '/' && source_code[src_pos + 1] == '/') {
            src_pos += 2;
            while (source_code[src_pos] && source_code[src_pos] != '\n') src_pos++;
            continue;
        }
        
        // 多行註解 /* */
        if (source_code[src_pos] == '/' && source_code[src_pos + 1] == '*') {
            src_pos += 2;
            while (source_code[src_pos] && !(source_code[src_pos] == '*' && source_code[src_pos + 1] == '/')) {
                src_pos++;
            }
            if (source_code[src_pos]) src_pos += 2;
            continue;
        }
        
        break;
    }
    
    int start = src_pos;
    cur_token.pos = start;
    
    // 字串
    if (source_code[src_pos] == '"') {
        src_pos++;
        int i = 0;
        while (source_code[src_pos] && source_code[src_pos] != '"') {
            cur_token.text[i++] = source_code[src_pos++];
        }
        cur_token.text[i] = '\0';
        if (source_code[src_pos] == '"') src_pos++;
        cur_token.type = TK_STRING;
        return;
    }
    
    // 數字
    if (isdigit(source_code[src_pos])) {
        int i = 0;
        while (isdigit(source_code[src_pos])) {
            cur_token.text[i++] = source_code[src_pos++];
        }
        cur_token.text[i] = '\0';
        cur_token.type = TK_NUM;
        return;
    }
    
    // 識別碼或關鍵字
    if (isalpha(source_code[src_pos]) || source_code[src_pos] == '_') {
        int i = 0;
        while (isalnum(source_code[src_pos]) || source_code[src_pos] == '_') {
            cur_token.text[i++] = source_code[src_pos++];
        }
        cur_token.text[i] = '\0';
        
        // 檢查關鍵字
        if (strcmp(cur_token.text, "func") == 0) cur_token.type = TK_FUNC;
        else if (strcmp(cur_token.text, "return") == 0) cur_token.type = TK_RETURN;
        else if (strcmp(cur_token.text, "if") == 0) cur_token.type = TK_IF;
        else if (strcmp(cur_token.text, "else") == 0) cur_token.type = TK_ELSE;
        else if (strcmp(cur_token.text, "while") == 0) cur_token.type = TK_WHILE;
        else if (strcmp(cur_token.text, "for") == 0) cur_token.type = TK_FOR;
        else if (strcmp(cur_token.text, "break") == 0) cur_token.type = TK_BREAK;
        else if (strcmp(cur_token.text, "continue") == 0) cur_token.type = TK_CONTINUE;
        else cur_token.type = TK_ID;
        return;
    }
    
    // 符號和運算子
    char ch = source_code[src_pos++];
    cur_token.text[0] = ch;
    cur_token.text[1] = '\0';
    
    switch (ch) {
        case '(': cur_token.type = TK_LPAREN; break;
        case ')': cur_token.type = TK_RPAREN; break;
        case '{': cur_token.type = TK_LBRACE; break;
        case '}': cur_token.type = TK_RBRACE; break;
        case '[': cur_token.type = TK_LBRACKET; break;
        case ']': cur_token.type = TK_RBRACKET; break;
        case '.': cur_token.type = TK_DOT; break;
        case ':': cur_token.type = TK_COLON; break;
        case ',': cur_token.type = TK_COMMA; break;
        case ';': cur_token.type = TK_SEMICOLON; break;
        case '+': cur_token.type = TK_PLUS; break;
        case '-': cur_token.type = TK_MINUS; break;
        case '*': cur_token.type = TK_MUL; break;
        case '/': cur_token.type = TK_DIV; break;
        case '<': cur_token.type = TK_LT; break;
        case '>': cur_token.type = TK_GT; break;
        case '=':
            if (source_code[src_pos] == '=') {
                src_pos++;
                cur_token.text[1] = '=';
                cur_token.text[2] = '\0';
                cur_token.type = TK_EQ;
            } else {
                cur_token.type = TK_ASSIGN;
            }
            break;
        default:
            report_error(start, "無法辨識的字元");
    }
}

// =========================================================
// Parser 輔助函數
// =========================================================
void emit(const char* op, const char* arg1, const char* arg2, const char* result) {
    strcpy(quads[quad_count].op, op);
    strcpy(quads[quad_count].arg1, arg1 ? arg1 : "");
    strcpy(quads[quad_count].arg2, arg2 ? arg2 : "");
    strcpy(quads[quad_count].result, result ? result : "");
    printf("%03d: %-12s %-10s %-10s %-10s\n", quad_count, op, 
           arg1 ? arg1 : "", arg2 ? arg2 : "", result ? result : "");
    quad_count++;
}

void new_temp(char* buf) {
    sprintf(buf, "t%d", temp_count++);
}

void expect(TokenType type, const char* msg) {
    if (cur_token.type != type) {
        report_error(cur_token.pos, msg);
    }
    next_token();
}

// =========================================================
// Parser 前向聲明
// =========================================================
char* expression();
void statement();
void expr_or_assign();

// =========================================================
// Parser - Primary
// =========================================================
#define MAX_TEMP_BUFFERS 100
static char temp_buffers[MAX_TEMP_BUFFERS][MAX_TOKEN_LEN];
static int temp_buffer_idx = 0;

char* get_temp_buffer() {
    char* buf = temp_buffers[temp_buffer_idx];
    temp_buffer_idx = (temp_buffer_idx + 1) % MAX_TEMP_BUFFERS;
    return buf;
}

char* primary() {
    char* temp = get_temp_buffer();
    
    if (cur_token.type == TK_NUM) {
        new_temp(temp);
        emit("IMM", cur_token.text, "", temp);
        next_token();
        return temp;
    }
    else if (cur_token.type == TK_STRING) {
        char idx_str[32];
        sprintf(idx_str, "%d", string_pool_count);
        string_pool[string_pool_count++] = strdup(cur_token.text);
        
        new_temp(temp);
        emit("LOAD_STR", idx_str, "", temp);
        next_token();
        return temp;
    }
    else if (cur_token.type == TK_ID) {
        strcpy(temp, cur_token.text);
        next_token();
        return temp;
    }
    else if (cur_token.type == TK_LBRACKET) {
        // 陣列字面量
        next_token();
        new_temp(temp);
        
        if (cur_token.type == TK_RBRACKET) {
            emit("NEW_ARR", "", "", temp);
            next_token();
            return temp;
        }
        
        char* first = expression();
        
        if (cur_token.type == TK_SEMICOLON) {
            // [init_val; size]
            next_token();
            char* size = expression();
            emit("INIT_ARR", first, size, temp);
            expect(TK_RBRACKET, "陣列預期 ']'");
            return temp;
        } else {
            // [elem1, elem2, ...]
            emit("NEW_ARR", "", "", temp);
            emit("APPEND_ITEM", temp, "", first);
            
            while (cur_token.type == TK_COMMA) {
                next_token();
                char* elem = expression();
                emit("APPEND_ITEM", temp, "", elem);
            }
            expect(TK_RBRACKET, "陣列預期 ']'");
            return temp;
        }
    }
    else if (cur_token.type == TK_LBRACE) {
        // 字典字面量
        next_token();
        new_temp(temp);
        emit("NEW_DICT", "", "", temp);
        
        if (cur_token.type != TK_RBRACE) {
            while (1) {
                char key_temp[MAX_TOKEN_LEN];
                
                if (cur_token.type == TK_ID || cur_token.type == TK_STRING) {
                    char idx_str[32];
                    sprintf(idx_str, "%d", string_pool_count);
                    string_pool[string_pool_count++] = strdup(cur_token.text);
                    
                    new_temp(key_temp);
                    emit("LOAD_STR", idx_str, "", key_temp);
                    next_token();
                } else {
                    report_error(cur_token.pos, "字典鍵必須是識別碼或字串");
                }
                
                expect(TK_COLON, "字典預期 ':'");
                char* val = expression();
                emit("SET_ITEM", temp, key_temp, val);
                
                if (cur_token.type != TK_COMMA) break;
                next_token();
            }
        }
        expect(TK_RBRACE, "字典預期 '}'");
        return temp;
    }
    else if (cur_token.type == TK_LPAREN) {
        next_token();
        char* expr = expression();
        expect(TK_RPAREN, "預期 ')'");
        return expr;
    }
    
    report_error(cur_token.pos, "無法解析的表達式");
    return NULL;
}

// =========================================================
// Parser - Factor
// =========================================================
char* factor() {
    char* result = get_temp_buffer();
    strcpy(result, primary());
    
    while (cur_token.type == TK_LBRACKET || cur_token.type == TK_DOT || cur_token.type == TK_LPAREN) {
        if (cur_token.type == TK_LBRACKET) {
            next_token();
            char* idx = expression();
            expect(TK_RBRACKET, "預期 ']'");
            
            char* temp = get_temp_buffer();
            new_temp(temp);
            emit("GET_ITEM", result, idx, temp);
            strcpy(result, temp);
        }
        else if (cur_token.type == TK_DOT) {
            next_token();
            char* key_temp = get_temp_buffer();
            char idx_str[32];
            
            sprintf(idx_str, "%d", string_pool_count);
            string_pool[string_pool_count++] = strdup(cur_token.text);
            
            new_temp(key_temp);
            emit("LOAD_STR", idx_str, "", key_temp);
            expect(TK_ID, "預期屬性名稱");
            
            char* temp = get_temp_buffer();
            new_temp(temp);
            emit("GET_ITEM", result, key_temp, temp);
            strcpy(result, temp);
        }
        else if (cur_token.type == TK_LPAREN) {
            next_token();
            int count = 0;
            
            if (cur_token.type != TK_RPAREN) {
                while (1) {
                    char* arg = expression();
                    emit("PARAM", arg, "", "");
                    count++;
                    if (cur_token.type != TK_COMMA) break;
                    next_token();
                }
            }
            expect(TK_RPAREN, "預期 ')'");
            
            char count_str[32];
            sprintf(count_str, "%d", count);
            char* temp = get_temp_buffer();
            new_temp(temp);
            emit("CALL", result, count_str, temp);
            strcpy(result, temp);
        }
    }
    
    return result;
}

// =========================================================
// Parser - Term
// =========================================================
char* term() {
    char* left = get_temp_buffer();
    strcpy(left, factor());
    
    while (cur_token.type == TK_MUL || cur_token.type == TK_DIV) {
        const char* op = (cur_token.type == TK_MUL) ? "MUL" : "DIV";
        next_token();
        char* right = factor();
        
        char* temp = get_temp_buffer();
        new_temp(temp);
        emit(op, left, right, temp);
        strcpy(left, temp);
    }
    
    return left;
}

// =========================================================
// Parser - Arithmetic Expression
// =========================================================
char* arith_expr() {
    char* left = get_temp_buffer();
    strcpy(left, term());
    
    while (cur_token.type == TK_PLUS || cur_token.type == TK_MINUS) {
        const char* op = (cur_token.type == TK_PLUS) ? "ADD" : "SUB";
        next_token();
        char* right = term();
        
        char* temp = get_temp_buffer();
        new_temp(temp);
        emit(op, left, right, temp);
        strcpy(left, temp);
    }
    
    return left;
}

// =========================================================
// Parser - Expression
// =========================================================
char* expression() {
    char* left = get_temp_buffer();
    strcpy(left, arith_expr());
    
    if (cur_token.type == TK_EQ || cur_token.type == TK_LT || cur_token.type == TK_GT) {
        const char* op;
        if (cur_token.type == TK_EQ) op = "CMP_EQ";
        else if (cur_token.type == TK_LT) op = "CMP_LT";
        else op = "CMP_GT";
        
        next_token();
        char* right = arith_expr();
        
        char* temp = get_temp_buffer();
        new_temp(temp);
        emit(op, left, right, temp);
        return temp;
    }
    
    return left;
}

// =========================================================
// Parser - Expression or Assignment
// =========================================================
void expr_or_assign() {
    if (cur_token.type != TK_ID) {
        expression();
        return;
    }
    
    char name[MAX_TOKEN_LEN];
    strcpy(name, cur_token.text);
    next_token();
    
    char obj[MAX_TOKEN_LEN];
    strcpy(obj, name);
    
    char* path[100];
    int path_count = 0;
    
    // 處理鏈式訪問
    while (cur_token.type == TK_LBRACKET || cur_token.type == TK_DOT || cur_token.type == TK_LPAREN) {
        if (cur_token.type == TK_LBRACKET) {
            next_token();
            path[path_count++] = strdup(expression());
            expect(TK_RBRACKET, "預期 ']'");
        }
        else if (cur_token.type == TK_DOT) {
            next_token();
            char key_temp[MAX_TOKEN_LEN];
            char idx_str[32];
            
            sprintf(idx_str, "%d", string_pool_count);
            string_pool[string_pool_count++] = strdup(cur_token.text);
            
            new_temp(key_temp);
            emit("LOAD_STR", idx_str, "", key_temp);
            expect(TK_ID, "預期屬性名稱");
            
            path[path_count++] = strdup(key_temp);
        }
        else if (cur_token.type == TK_LPAREN) {
            // 函數調用
            char temp[MAX_TOKEN_LEN];
            for (int i = 0; i < path_count; i++) {
                new_temp(temp);
                emit("GET_ITEM", obj, path[i], temp);
                strcpy(obj, temp);
            }
            path_count = 0;
            
            next_token();
            int count = 0;
            
            if (cur_token.type != TK_RPAREN) {
                while (1) {
                    char* arg = expression();
                    emit("PARAM", arg, "", "");
                    count++;
                    if (cur_token.type != TK_COMMA) break;
                    next_token();
                }
            }
            expect(TK_RPAREN, "預期 ')'");
            
            char count_str[32];
            sprintf(count_str, "%d", count);
            new_temp(temp);
            emit("CALL", obj, count_str, temp);
            strcpy(obj, temp);
        }
    }
    
    // 賦值
    if (cur_token.type == TK_ASSIGN) {
        next_token();
        char* val = expression();
        
        if (path_count == 0) {
            emit("STORE", val, "", obj);
        } else {
            char temp[MAX_TOKEN_LEN];
            for (int i = 0; i < path_count - 1; i++) {
                new_temp(temp);
                emit("GET_ITEM", obj, path[i], temp);
                strcpy(obj, temp);
            }
            emit("SET_ITEM", obj, path[path_count - 1], val);
        }
    }
}

// =========================================================
// Parser - Statement
// =========================================================
void statement() {
    if (cur_token.type == TK_IF) {
        next_token();
        expect(TK_LPAREN, "預期 '('");
        char* cond = expression();
        expect(TK_RPAREN, "預期 ')'");
        expect(TK_LBRACE, "預期 '{'");
        
        int jmp_f_idx = quad_count;
        emit("JMP_F", cond, "", "?");
        
        while (cur_token.type != TK_RBRACE && cur_token.type != TK_EOF) {
            statement();
        }
        expect(TK_RBRACE, "預期 '}'");
        
        if (cur_token.type == TK_ELSE) {
            int jmp_end_idx = quad_count;
            emit("JMP", "", "", "?");
            
            sprintf(quads[jmp_f_idx].result, "%d", quad_count);
            
            next_token();
            expect(TK_LBRACE, "預期 '{'");
            while (cur_token.type != TK_RBRACE && cur_token.type != TK_EOF) {
                statement();
            }
            expect(TK_RBRACE, "預期 '}'");
            
            sprintf(quads[jmp_end_idx].result, "%d", quad_count);
        } else {
            sprintf(quads[jmp_f_idx].result, "%d", quad_count);
        }
    }
    else if (cur_token.type == TK_WHILE) {
        next_token();
        expect(TK_LPAREN, "預期 '('");
        
        int cond_idx = quad_count;
        char* cond = expression();
        
        expect(TK_RPAREN, "預期 ')'");
        expect(TK_LBRACE, "預期 '{'");
        
        int jmp_f_idx = quad_count;
        emit("JMP_F", cond, "", "?");
        
        loop_stack[loop_stack_count].break_count = 0;
        loop_stack[loop_stack_count].continue_target = cond_idx;
        loop_stack_count++;
        
        while (cur_token.type != TK_RBRACE && cur_token.type != TK_EOF) {
            statement();
        }
        
        char cond_str[32];
        sprintf(cond_str, "%d", cond_idx);
        emit("JMP", "", "", cond_str);
        
        expect(TK_RBRACE, "預期 '}'");
        
        int end_idx = quad_count;
        sprintf(quads[jmp_f_idx].result, "%d", end_idx);
        
        loop_stack_count--;
        for (int i = 0; i < loop_stack[loop_stack_count].break_count; i++) {
            sprintf(quads[loop_stack[loop_stack_count].break_list[i]].result, "%d", end_idx);
        }
    }
    else if (cur_token.type == TK_FOR) {
        next_token();
        expect(TK_LPAREN, "預期 '('");
        
        // Init
        if (cur_token.type != TK_SEMICOLON) {
            expr_or_assign();
        }
        expect(TK_SEMICOLON, "預期 ';'");
        
        // Condition
        int cond_idx = quad_count;
        char* cond = NULL;
        if (cur_token.type != TK_SEMICOLON) {
            cond = expression();
        } else {
            char temp[MAX_TOKEN_LEN];
            new_temp(temp);
            emit("IMM", "1", "", temp);
            cond = temp;
        }
        
        int jmp_f_idx = quad_count;
        emit("JMP_F", cond, "", "?");
        
        int jmp_body_idx = quad_count;
        emit("JMP", "", "", "?");
        
        expect(TK_SEMICOLON, "預期 ';'");
        
        // Step
        int step_idx = quad_count;
        if (cur_token.type != TK_RPAREN) {
            expr_or_assign();
        }
        
        char cond_str[32];
        sprintf(cond_str, "%d", cond_idx);
        emit("JMP", "", "", cond_str);
        
        expect(TK_RPAREN, "預期 ')'");
        expect(TK_LBRACE, "預期 '{'");
        
        sprintf(quads[jmp_body_idx].result, "%d", quad_count);
        
        loop_stack[loop_stack_count].break_count = 0;
        loop_stack[loop_stack_count].continue_target = step_idx;
        loop_stack_count++;
        
        while (cur_token.type != TK_RBRACE && cur_token.type != TK_EOF) {
            statement();
        }
        
        char step_str[32];
        sprintf(step_str, "%d", step_idx);
        emit("JMP", "", "", step_str);
        
        expect(TK_RBRACE, "預期 '}'");
        
        int end_idx = quad_count;
        sprintf(quads[jmp_f_idx].result, "%d", end_idx);
        
        loop_stack_count--;
        for (int i = 0; i < loop_stack[loop_stack_count].break_count; i++) {
            sprintf(quads[loop_stack[loop_stack_count].break_list[i]].result, "%d", end_idx);
        }
    }
    else if (cur_token.type == TK_BREAK) {
        next_token();
        if (loop_stack_count == 0) {
            report_error(cur_token.pos, "break 必須在迴圈內使用");
        }
        int idx = quad_count;
        emit("JMP", "", "", "?");
        loop_stack[loop_stack_count - 1].break_list[loop_stack[loop_stack_count - 1].break_count++] = idx;
        expect(TK_SEMICOLON, "預期 ';'");
    }
    else if (cur_token.type == TK_CONTINUE) {
        next_token();
        if (loop_stack_count == 0) {
            report_error(cur_token.pos, "continue 必須在迴圈內使用");
        }
        char target[32];
        sprintf(target, "%d", loop_stack[loop_stack_count - 1].continue_target);
        emit("JMP", "", "", target);
        expect(TK_SEMICOLON, "預期 ';'");
    }
    else if (cur_token.type == TK_RETURN) {
        next_token();
        char* res = expression();
        emit("RET_VAL", res, "", "");
        expect(TK_SEMICOLON, "預期 ';'");
    }
    else if (cur_token.type == TK_ID) {
        expr_or_assign();
        expect(TK_SEMICOLON, "預期 ';'");
    }
    else {
        report_error(cur_token.pos, "無法辨識的陳述句");
    }
}

// =========================================================
// Parser - Program
// =========================================================
void parse_program() {
    while (cur_token.type != TK_EOF) {
        if (cur_token.type == TK_FUNC) {
            next_token();
            char func_name[MAX_TOKEN_LEN];
            strcpy(func_name, cur_token.text);
            expect(TK_ID, "預期函數名稱");
            
            emit("FUNC_BEG", func_name, "", "");
            
            expect(TK_LPAREN, "預期 '('");
            if (cur_token.type != TK_RPAREN) {
                while (1) {
                    emit("FORMAL", cur_token.text, "", "");
                    expect(TK_ID, "預期參數名稱");
                    if (cur_token.type != TK_COMMA) break;
                    next_token();
                }
            }
            expect(TK_RPAREN, "預期 ')'");
            expect(TK_LBRACE, "預期 '{'");
            
            while (cur_token.type != TK_RBRACE && cur_token.type != TK_EOF) {
                statement();
            }
            
            emit("FUNC_END", "", "", "");
            expect(TK_RBRACE, "預期 '}'");
        } else {
            statement();
        }
    }
}

// =========================================================
// VM - 變數操作
// =========================================================
Value* get_var(const char* name) {
    // 檢查是否為數字
    if (isdigit(name[0]) || (name[0] == '-' && isdigit(name[1]))) {
        return create_int(atoi(name));
    }
    
    // 在當前框架中查找
    Frame* frame = &call_stack[sp];
    for (int i = 0; i < frame->var_count; i++) {
        if (strcmp(frame->var_names[i], name) == 0) {
            return frame->vars[i];
        }
    }
    
    // 未找到，返回 0
    return create_int(0);
}

void set_var(const char* name, Value* val) {
    Frame* frame = &call_stack[sp];
    
    // 查找是否已存在
    for (int i = 0; i < frame->var_count; i++) {
        if (strcmp(frame->var_names[i], name) == 0) {
            frame->vars[i] = val;
            return;
        }
    }
    
    // 新增變數
    strcpy(frame->var_names[frame->var_count], name);
    frame->vars[frame->var_count] = val;
    frame->var_count++;
}

// =========================================================
// 輔助函數：打印 Value
// =========================================================
void print_value(Value* v) {
    if (v->type == VAL_INT) {
        printf("%d", v->data.int_val);
    } else if (v->type == VAL_STRING) {
        printf("%s", v->data.str_val);
    } else if (v->type == VAL_ARRAY) {
        printf("[");
        for (int j = 0; j < v->data.arr.size; j++) {
            if (j > 0) printf(", ");
            print_value(v->data.arr.items[j]);
        }
        printf("]");
    } else if (v->type == VAL_DICT) {
        printf("{");
        for (int j = 0; j < v->data.dict.size; j++) {
            if (j > 0) printf(", ");
            printf("\"%s\":", v->data.dict.keys[j]);
            print_value(v->data.dict.values[j]);
        }
        printf("}");
    }
}

// =========================================================
// VM - 系統函數
// =========================================================
int system_call(const char* func_name, Value** args, int arg_count, Value** ret_val) {
    if (strcmp(func_name, "print") == 0) {
        for (int i = 0; i < arg_count; i++) {
            if (i > 0) printf(" ");
            print_value(args[i]);
        }
        printf("\n");
        *ret_val = create_int(0);
        return 1;
    }
    else if (strcmp(func_name, "len") == 0) {
        if (arg_count != 1) return 0;
        int len = 0;
        if (args[0]->type == VAL_ARRAY) {
            len = args[0]->data.arr.size;
        } else if (args[0]->type == VAL_DICT) {
            len = args[0]->data.dict.size;
        } else if (args[0]->type == VAL_STRING) {
            len = strlen(args[0]->data.str_val);
        }
        *ret_val = create_int(len);
        return 1;
    }
    else if (strcmp(func_name, "array") == 0) {
        if (arg_count != 2) return 0;
        int size = args[0]->data.int_val;
        Value* arr = create_array();
        for (int i = 0; i < size; i++) {
            array_append(arr, args[1]);
        }
        *ret_val = arr;
        return 1;
    }
    else if (strcmp(func_name, "push") == 0) {
        if (arg_count != 2) return 0;
        array_append(args[0], args[1]);
        *ret_val = args[0];
        return 1;
    }
    else if (strcmp(func_name, "pop") == 0) {
        if (arg_count != 1) return 0;
        if (args[0]->data.arr.size > 0) {
            *ret_val = args[0]->data.arr.items[--args[0]->data.arr.size];
        } else {
            *ret_val = create_int(0);
        }
        return 1;
    }
    else if (strcmp(func_name, "keys") == 0) {
        if (arg_count != 1) return 0;
        Value* arr = create_array();
        if (args[0]->type == VAL_DICT) {
            for (int i = 0; i < args[0]->data.dict.size; i++) {
                array_append(arr, create_string(args[0]->data.dict.keys[i]));
            }
        }
        *ret_val = arr;
        return 1;
    }
    else if (strcmp(func_name, "has_key") == 0) {
        if (arg_count != 2) return 0;
        int found = 0;
        if (args[0]->type == VAL_DICT && args[1]->type == VAL_STRING) {
            for (int i = 0; i < args[0]->data.dict.size; i++) {
                if (strcmp(args[0]->data.dict.keys[i], args[1]->data.str_val) == 0) {
                    found = 1;
                    break;
                }
            }
        }
        *ret_val = create_int(found);
        return 1;
    }
    else if (strcmp(func_name, "remove") == 0) {
        if (arg_count != 2) return 0;
        if (args[0]->type == VAL_DICT && args[1]->type == VAL_STRING) {
            for (int i = 0; i < args[0]->data.dict.size; i++) {
                if (strcmp(args[0]->data.dict.keys[i], args[1]->data.str_val) == 0) {
                    // 移除此鍵值對
                    free(args[0]->data.dict.keys[i]);
                    for (int j = i; j < args[0]->data.dict.size - 1; j++) {
                        args[0]->data.dict.keys[j] = args[0]->data.dict.keys[j + 1];
                        args[0]->data.dict.values[j] = args[0]->data.dict.values[j + 1];
                    }
                    args[0]->data.dict.size--;
                    break;
                }
            }
        }
        *ret_val = args[0];
        return 1;
    }
    else if (strcmp(func_name, "typeof") == 0) {
        if (arg_count != 1) return 0;
        const char* type_name;
        if (args[0]->type == VAL_INT) type_name = "int";
        else if (args[0]->type == VAL_STRING) type_name = "string";
        else if (args[0]->type == VAL_ARRAY) type_name = "array";
        else if (args[0]->type == VAL_DICT) type_name = "dict";
        else type_name = "unknown";
        *ret_val = create_string(type_name);
        return 1;
    }
    else if (strcmp(func_name, "ord") == 0) {
        if (arg_count != 1) return 0;
        if (args[0]->type == VAL_STRING && args[0]->data.str_val[0] != '\0') {
            *ret_val = create_int((int)args[0]->data.str_val[0]);
        } else {
            *ret_val = create_int(0);
        }
        return 1;
    }
    else if (strcmp(func_name, "chr") == 0) {
        if (arg_count != 1) return 0;
        char buf[2];
        buf[0] = (char)args[0]->data.int_val;
        buf[1] = '\0';
        *ret_val = create_string(buf);
        return 1;
    }
    else if (strcmp(func_name, "input") == 0) {
        // 簡化版：輸出提示訊息，然後讀取一行輸入
        if (arg_count > 0 && args[0]->type == VAL_STRING) {
            printf("%s", args[0]->data.str_val);
            fflush(stdout);
        }
        
        char buf[1024];
        if (fgets(buf, sizeof(buf), stdin)) {
            // 移除換行符
            int len = strlen(buf);
            if (len > 0 && buf[len-1] == '\n') {
                buf[len-1] = '\0';
            }
            *ret_val = create_string(buf);
        } else {
            *ret_val = create_string("");
        }
        return 1;
    }
    else if (strcmp(func_name, "exit") == 0) {
        int code = (arg_count > 0) ? args[0]->data.int_val : 0;
        exit(code);
    }
    else if (strcmp(func_name, "int") == 0) {
        if (arg_count != 1) return 0;
        if (args[0]->type == VAL_STRING) {
            *ret_val = create_int(atoi(args[0]->data.str_val));
        } else {
            *ret_val = args[0];
        }
        return 1;
    }
    else if (strcmp(func_name, "str") == 0) {
        if (arg_count != 1) return 0;
        char buf[256];
        if (args[0]->type == VAL_INT) {
            sprintf(buf, "%d", args[0]->data.int_val);
            *ret_val = create_string(buf);
        } else {
            *ret_val = args[0];
        }
        return 1;
    }
    else if (strcmp(func_name, "time") == 0) {
        *ret_val = create_int((int)time(NULL));
        return 1;
    }
    else if (strcmp(func_name, "random") == 0) {
        *ret_val = create_int(rand() % 100);
        return 1;
    }
    
    return 0;
}

// =========================================================
// VM - 執行
// =========================================================
void run_vm() {
    int pc = 0;
    Value* param_stack[MAX_PARAMS];
    int param_count = 0;
    
    // 建立函數映射表
    for (int i = 0; i < quad_count; i++) {
        if (strcmp(quads[i].op, "FUNC_BEG") == 0) {
            strcpy(func_map[func_map_count].name, quads[i].arg1);
            func_map[func_map_count].pc = i + 1;
            func_map_count++;
        }
    }
    
    // 初始化全域框架
    call_stack[0].var_count = 0;
    call_stack[0].ret_pc = -1;
    call_stack[0].incoming_args_count = 0;
    call_stack[0].formal_idx = 0;
    sp = 0;
    
    printf("\n=== VM 執行開始 ===\n");
    
    while (pc < quad_count) {
        Quad* q = &quads[pc];
        
        if (strcmp(q->op, "FUNC_BEG") == 0) {
            while (strcmp(quads[pc].op, "FUNC_END") != 0) pc++;
        }
        else if (strcmp(q->op, "IMM") == 0) {
            set_var(q->result, create_int(atoi(q->arg1)));
        }
        else if (strcmp(q->op, "LOAD_STR") == 0) {
            int idx = atoi(q->arg1);
            set_var(q->result, create_string(string_pool[idx]));
        }
        else if (strcmp(q->op, "ADD") == 0) {
            Value* a = get_var(q->arg1);
            Value* b = get_var(q->arg2);
            
            // 如果有一個是字串，則進行字串串接
            if (a->type == VAL_STRING || b->type == VAL_STRING) {
                char buf[1024];
                char a_str[512], b_str[512];
                
                if (a->type == VAL_INT) sprintf(a_str, "%d", a->data.int_val);
                else if (a->type == VAL_STRING) strcpy(a_str, a->data.str_val);
                else strcpy(a_str, "");
                
                if (b->type == VAL_INT) sprintf(b_str, "%d", b->data.int_val);
                else if (b->type == VAL_STRING) strcpy(b_str, b->data.str_val);
                else strcpy(b_str, "");
                
                sprintf(buf, "%s%s", a_str, b_str);
                set_var(q->result, create_string(buf));
            } else {
                set_var(q->result, create_int(a->data.int_val + b->data.int_val));
            }
        }
        else if (strcmp(q->op, "SUB") == 0) {
            Value* a = get_var(q->arg1);
            Value* b = get_var(q->arg2);
            set_var(q->result, create_int(a->data.int_val - b->data.int_val));
        }
        else if (strcmp(q->op, "MUL") == 0) {
            Value* a = get_var(q->arg1);
            Value* b = get_var(q->arg2);
            set_var(q->result, create_int(a->data.int_val * b->data.int_val));
        }
        else if (strcmp(q->op, "DIV") == 0) {
            Value* a = get_var(q->arg1);
            Value* b = get_var(q->arg2);
            int divisor = (b->data.int_val != 0) ? b->data.int_val : 1;
            set_var(q->result, create_int(a->data.int_val / divisor));
        }
        else if (strcmp(q->op, "CMP_EQ") == 0) {
            Value* a = get_var(q->arg1);
            Value* b = get_var(q->arg2);
            set_var(q->result, create_int(a->data.int_val == b->data.int_val ? 1 : 0));
        }
        else if (strcmp(q->op, "CMP_LT") == 0) {
            Value* a = get_var(q->arg1);
            Value* b = get_var(q->arg2);
            set_var(q->result, create_int(a->data.int_val < b->data.int_val ? 1 : 0));
        }
        else if (strcmp(q->op, "CMP_GT") == 0) {
            Value* a = get_var(q->arg1);
            Value* b = get_var(q->arg2);
            set_var(q->result, create_int(a->data.int_val > b->data.int_val ? 1 : 0));
        }
        else if (strcmp(q->op, "STORE") == 0) {
            set_var(q->result, get_var(q->arg1));
        }
        else if (strcmp(q->op, "NEW_ARR") == 0) {
            set_var(q->result, create_array());
        }
        else if (strcmp(q->op, "INIT_ARR") == 0) {
            Value* init_val = get_var(q->arg1);
            int size = get_var(q->arg2)->data.int_val;
            Value* arr = create_array();
            for (int i = 0; i < size; i++) {
                array_append(arr, init_val);
            }
            set_var(q->result, arr);
        }
        else if (strcmp(q->op, "NEW_DICT") == 0) {
            set_var(q->result, create_dict());
        }
        else if (strcmp(q->op, "APPEND_ITEM") == 0) {
            Value* arr = get_var(q->arg1);
            Value* item = get_var(q->result);
            array_append(arr, item);
        }
        else if (strcmp(q->op, "SET_ITEM") == 0) {
            Value* obj = get_var(q->arg1);
            Value* key = get_var(q->arg2);
            Value* val = get_var(q->result);
            
            if (obj->type == VAL_ARRAY) {
                int idx = key->data.int_val;
                if (idx >= 0 && idx < obj->data.arr.size) {
                    obj->data.arr.items[idx] = val;
                }
            } else if (obj->type == VAL_DICT) {
                dict_set(obj, key->data.str_val, val);
            }
        }
        else if (strcmp(q->op, "GET_ITEM") == 0) {
            Value* obj = get_var(q->arg1);
            Value* key = get_var(q->arg2);
            
            if (obj->type == VAL_ARRAY) {
                int idx = key->data.int_val;
                if (idx >= 0 && idx < obj->data.arr.size) {
                    set_var(q->result, obj->data.arr.items[idx]);
                } else {
                    set_var(q->result, create_int(0));
                }
            } else if (obj->type == VAL_DICT) {
                set_var(q->result, dict_get(obj, key->data.str_val));
            }
        }
        else if (strcmp(q->op, "JMP") == 0) {
            pc = atoi(q->result) - 1;
        }
        else if (strcmp(q->op, "JMP_F") == 0) {
            Value* cond = get_var(q->arg1);
            if (cond->data.int_val == 0) {
                pc = atoi(q->result) - 1;
            }
        }
        else if (strcmp(q->op, "PARAM") == 0) {
            param_stack[param_count++] = get_var(q->arg1);
        }
        else if (strcmp(q->op, "CALL") == 0) {
            int p_count = atoi(q->arg2);
            const char* f_name = q->arg1;
            
            // 嘗試獲取函數名稱（可能是變數）
            Value* name_val = get_var(q->arg1);
            if (name_val->type == VAL_STRING) {
                f_name = name_val->data.str_val;
            }
            
            // 檢查系統函數
            Value* ret_val;
            if (system_call(f_name, param_stack + param_count - p_count, p_count, &ret_val)) {
                param_count -= p_count;
                set_var(q->result, ret_val);
                pc++;
                continue;
            }
            
            // 查找用戶函數
            int target_pc = -1;
            for (int i = 0; i < func_map_count; i++) {
                if (strcmp(func_map[i].name, f_name) == 0) {
                    target_pc = func_map[i].pc;
                    break;
                }
            }
            
            if (target_pc == -1) {
                printf("[VM 錯誤] 找不到函數 '%s'\n", f_name);
                exit(1);
            }
            
            // 創建新框架
            sp++;
            call_stack[sp].var_count = 0;
            call_stack[sp].ret_pc = pc + 1;
            strcpy(call_stack[sp].ret_var, q->result);
            call_stack[sp].formal_idx = 0;
            
            // 複製參數
            call_stack[sp].incoming_args_count = p_count;
            for (int i = 0; i < p_count; i++) {
                call_stack[sp].incoming_args[i] = param_stack[param_count - p_count + i];
            }
            param_count -= p_count;
            
            pc = target_pc;
            continue;
        }
        else if (strcmp(q->op, "FORMAL") == 0) {
            Frame* frame = &call_stack[sp];
            set_var(q->arg1, frame->incoming_args[frame->formal_idx++]);
        }
        else if (strcmp(q->op, "RET_VAL") == 0) {
            Value* ret_val = get_var(q->arg1);
            int ret_pc = call_stack[sp].ret_pc;
            char ret_var[MAX_TOKEN_LEN];
            strcpy(ret_var, call_stack[sp].ret_var);
            
            sp--;
            set_var(ret_var, ret_val);
            pc = ret_pc;
            continue;
        }
        else if (strcmp(q->op, "FUNC_END") == 0) {
            if (sp > 0) {
                int ret_pc = call_stack[sp].ret_pc;
                char ret_var[MAX_TOKEN_LEN];
                strcpy(ret_var, call_stack[sp].ret_var);
                
                sp--;
                set_var(ret_var, create_int(0));
                pc = ret_pc;
                continue;
            }
        }
        
        pc++;
    }
    
    printf("=== VM 執行完畢 ===\n\n記憶體狀態 (全域變數):\n");
    for (int i = 0; i < call_stack[0].var_count; i++) {
        if (call_stack[0].var_names[i][0] != 't') {
            printf("[%s] = ", call_stack[0].var_names[i]);
            print_value(call_stack[0].vars[i]);
            printf("\n");
        }
    }
}

// =========================================================
// Main
// =========================================================
int main(int argc, char* argv[]) {
    if (argc < 2) {
        printf("用法: %s <source_file>\n", argv[0]);
        return 1;
    }
    
    // 讀取原始碼
    FILE* fp = fopen(argv[1], "r");
    if (!fp) {
        printf("無法開啟檔案: %s\n", argv[1]);
        return 1;
    }
    
    fseek(fp, 0, SEEK_END);
    long size = ftell(fp);
    fseek(fp, 0, SEEK_SET);
    
    source_code = (char*)malloc(size + 1);
    fread(source_code, 1, size, fp);
    source_code[size] = '\0';
    fclose(fp);
    
    // 編譯
    printf("編譯器生成的中間碼 (PC: Quadruples):\n");
    printf("--------------------------------------------\n");
    
    next_token();
    parse_program();
    
    // 執行
    srand(time(NULL));
    run_vm();
    
    return 0;
}