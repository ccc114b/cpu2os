#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <stdbool.h>
#include <stdint.h>

// =========================================================
// 基礎工具與資料結構定義
// =========================================================

// 動態字串緩衝區
typedef struct {
    char* data;
    size_t len;
    size_t cap;
} StrBuf;

void StrBuf_init(StrBuf* sb) {
    sb->cap = 32;
    sb->len = 0;
    sb->data = (char*)malloc(sb->cap);
    sb->data[0] = '\0';
}

void StrBuf_append(StrBuf* sb, const char* str) {
    size_t slen = strlen(str);
    if (sb->len + slen + 1 > sb->cap) {
        while (sb->len + slen + 1 > sb->cap) sb->cap *= 2;
        sb->data = (char*)realloc(sb->data, sb->cap);
    }
    strcpy(sb->data + sb->len, str);
    sb->len += slen;
}

// =========================================================
// 錯誤回報工具
// =========================================================
void report_error(const char* src, size_t pos, const char* msg) {
    const char* current = src;
    size_t current_pos = 0;
    size_t line_idx = 0;
    const char* line_start = src;

    while (*current) {
        if (current_pos == pos) break;
        if (*current == '\n') {
            line_idx++;
            line_start = current + 1;
        }
        current++;
        current_pos++;
    }

    // 尋找行尾
    const char* line_end = line_start;
    while (*line_end && *line_end != '\n') line_end++;

    size_t col_idx = current_pos - (line_start - src);

    printf("\n❌ [語法錯誤] 第 %zu 行, 第 %zu 字元: %s\n  ", line_idx + 1, col_idx + 1, msg);
    for (const char* p = line_start; p < line_end; p++) putchar(*p);
    printf("\n  ");

    for (size_t i = 0; i < col_idx; i++) {
        if (line_start[i] == '\t') putchar('\t');
        else putchar(' ');
    }
    printf("^\n");

    exit(1);
}

// =========================================================
// 1. 詞彙標記與中間碼
// =========================================================
typedef enum {
    TK_FUNC, TK_RETURN, TK_IF, TK_PRINT,
    TK_WHILE, TK_FOR, TK_BREAK, TK_CONTINUE,
    TK_ID, TK_NUM, TK_STRING,
    TK_LPAREN, TK_RPAREN, TK_LBRACE, TK_RBRACE, TK_LBRACKET, TK_RBRACKET,
    TK_DOT, TK_COLON, TK_COMMA, TK_SEMICOLON,
    TK_ASSIGN, TK_PLUS, TK_MINUS, TK_MUL, TK_DIV,
    TK_EQ, TK_LT, TK_GT,
    TK_EOF
} TokenType;

typedef struct {
    TokenType t_type;
    char* text;
    size_t pos;
} Token;

typedef struct {
    char* op;
    char* arg1;
    char* arg2;
    char* result;
} Quad;

// =========================================================
// 虛擬機值定義 (宣告於前以便 Map 使用)
// =========================================================
typedef enum { VAL_INT, VAL_STR, VAL_ARR, VAL_DICT, VAL_NIL } ValueType;

struct VArray;
struct VDict;

typedef struct {
    ValueType type;
    union {
        int64_t i;
        char* s;
        struct VArray* arr;
        struct VDict* dict;
    } as;
} Value;

// 簡單字串 Map (變數表 / 字典 / 函數映射)
typedef struct {
    char* key;
    Value val;
} MapEntry;

typedef struct {
    MapEntry* data;
    size_t len;
    size_t cap;
} MapVec;

void Map_init(MapVec* map) {
    map->len = 0; map->cap = 8;
    map->data = (MapEntry*)malloc(map->cap * sizeof(MapEntry));
}

Value Map_get(MapVec* map, const char* key) {
    for (size_t i = 0; i < map->len; i++) {
        if (strcmp(map->data[i].key, key) == 0) return map->data[i].val;
    }
    return (Value){ VAL_NIL, .as={0} };
}

void Map_set(MapVec* map, const char* key, Value val) {
    for (size_t i = 0; i < map->len; i++) {
        if (strcmp(map->data[i].key, key) == 0) {
            map->data[i].val = val;
            return;
        }
    }
    if (map->len >= map->cap) {
        map->cap *= 2;
        map->data = (MapEntry*)realloc(map->data, map->cap * sizeof(MapEntry));
    }
    map->data[map->len].key = strdup(key);
    map->data[map->len].val = val;
    map->len++;
}

// 陣列與字典結構
typedef struct VArray {
    Value* data;
    size_t len;
    size_t cap;
} VArray;

typedef struct VDict {
    MapVec map;
} VDict;

char* value_to_string(Value v);

// =========================================================
// 2. 詞法分析 (Lexer)
// =========================================================
typedef struct {
    const char* src_str;
    size_t src_len;
    size_t pos;
    Token cur_token;
} Lexer;

void Lexer_init(Lexer* lexer, const char* src) {
    lexer->src_str = src;
    lexer->src_len = strlen(src);
    lexer->pos = 0;
}

bool is_id_char(unsigned char c) {
    return isalnum(c) || c == '_' || c >= 128; // 允許中文字元等非 ASCII 字元作為識別碼
}

void Lexer_next_token(Lexer* lexer) {
    while (1) {
        while (lexer->pos < lexer->src_len && isspace(lexer->src_str[lexer->pos])) {
            lexer->pos++;
        }
        if (lexer->pos >= lexer->src_len) {
            lexer->cur_token = (Token){ TK_EOF, strdup(""), lexer->pos };
            return;
        }

        if (lexer->src_str[lexer->pos] == '/') {
            if (lexer->pos + 1 < lexer->src_len && lexer->src_str[lexer->pos + 1] == '/') {
                lexer->pos += 2;
                while (lexer->pos < lexer->src_len && lexer->src_str[lexer->pos] != '\n') lexer->pos++;
                continue;
            } else if (lexer->pos + 1 < lexer->src_len && lexer->src_str[lexer->pos + 1] == '*') {
                lexer->pos += 2;
                while (lexer->pos + 1 < lexer->src_len && !(lexer->src_str[lexer->pos] == '*' && lexer->src_str[lexer->pos + 1] == '/')) {
                    lexer->pos++;
                }
                if (lexer->pos + 1 < lexer->src_len) lexer->pos += 2;
                continue;
            }
        }
        break;
    }

    size_t start = lexer->pos;
    char ch = lexer->src_str[lexer->pos];

    // 字串
    if (ch == '"') {
        lexer->pos++;
        size_t start_str = lexer->pos;
        while (lexer->pos < lexer->src_len && lexer->src_str[lexer->pos] != '"') lexer->pos++;
        if (lexer->pos >= lexer->src_len) {
            report_error(lexer->src_str, start, "字串缺少結尾的雙引號 '\"'");
        }
        size_t len = lexer->pos - start_str;
        char* text = (char*)malloc(len + 1);
        strncpy(text, lexer->src_str + start_str, len);
        text[len] = '\0';
        lexer->pos++;
        lexer->cur_token = (Token){ TK_STRING, text, start };
        return;
    }

    // 數字
    if (isdigit(ch)) {
        while (lexer->pos < lexer->src_len && isdigit(lexer->src_str[lexer->pos])) lexer->pos++;
        size_t len = lexer->pos - start;
        char* text = (char*)malloc(len + 1);
        strncpy(text, lexer->src_str + start, len);
        text[len] = '\0';
        lexer->cur_token = (Token){ TK_NUM, text, start };
        return;
    }

    // 識別碼與關鍵字
    if (isalpha(ch) || ch == '_' || (unsigned char)ch >= 128) {
        while (lexer->pos < lexer->src_len && is_id_char(lexer->src_str[lexer->pos])) lexer->pos++;
        size_t len = lexer->pos - start;
        char* text = (char*)malloc(len + 1);
        strncpy(text, lexer->src_str + start, len);
        text[len] = '\0';

        TokenType t_type = TK_ID;
        if (strcmp(text, "func") == 0) t_type = TK_FUNC;
        else if (strcmp(text, "return") == 0) t_type = TK_RETURN;
        else if (strcmp(text, "if") == 0) t_type = TK_IF;
        else if (strcmp(text, "print") == 0) t_type = TK_PRINT;
        else if (strcmp(text, "while") == 0) t_type = TK_WHILE;
        else if (strcmp(text, "for") == 0) t_type = TK_FOR;
        else if (strcmp(text, "break") == 0) t_type = TK_BREAK;
        else if (strcmp(text, "continue") == 0) t_type = TK_CONTINUE;

        lexer->cur_token = (Token){ t_type, text, start };
        return;
    }

    // 符號
    lexer->pos++;
    TokenType t_type;
    char text[3] = {ch, '\0', '\0'};

    switch (ch) {
        case '(': t_type = TK_LPAREN; break;
        case ')': t_type = TK_RPAREN; break;
        case '{': t_type = TK_LBRACE; break;
        case '}': t_type = TK_RBRACE; break;
        case '[': t_type = TK_LBRACKET; break;
        case ']': t_type = TK_RBRACKET; break;
        case '.': t_type = TK_DOT; break;
        case ':': t_type = TK_COLON; break;
        case '+': t_type = TK_PLUS; break;
        case '-': t_type = TK_MINUS; break;
        case '*': t_type = TK_MUL; break;
        case '/': t_type = TK_DIV; break;
        case ',': t_type = TK_COMMA; break;
        case ';': t_type = TK_SEMICOLON; break;
        case '<': t_type = TK_LT; break;
        case '>': t_type = TK_GT; break;
        case '=':
            if (lexer->pos < lexer->src_len && lexer->src_str[lexer->pos] == '=') {
                lexer->pos++;
                strcpy(text, "==");
                t_type = TK_EQ;
            } else {
                t_type = TK_ASSIGN;
            }
            break;
        default: {
            char err[64];
            sprintf(err, "無法辨識的字元: '%c'", ch);
            report_error(lexer->src_str, start, err);
        }
    }
    lexer->cur_token = (Token){ t_type, strdup(text), start };
}

// =========================================================
// 3. 語法解析 (Parser)
// =========================================================
typedef struct {
    size_t* breaks;
    size_t breaks_len;
    size_t breaks_cap;
    size_t continue_target;
} LoopCtx;

typedef struct {
    Lexer* lexer;
    Quad* quads;
    size_t quads_len;
    size_t quads_cap;
    char** string_pool;
    size_t pool_len;
    size_t pool_cap;
    LoopCtx* loop_stack;
    size_t loop_len;
    size_t loop_cap;
    size_t t_idx;
} Parser;

void Parser_init(Parser* p, Lexer* lexer) {
    p->lexer = lexer;
    p->quads_len = 0; p->quads_cap = 64;
    p->quads = (Quad*)malloc(p->quads_cap * sizeof(Quad));
    p->pool_len = 0; p->pool_cap = 32;
    p->string_pool = (char**)malloc(p->pool_cap * sizeof(char*));
    p->loop_len = 0; p->loop_cap = 8;
    p->loop_stack = (LoopCtx*)malloc(p->loop_cap * sizeof(LoopCtx));
    p->t_idx = 0;
}

Token* Parser_cur(Parser* p) {
    return &p->lexer->cur_token;
}

void Parser_consume(Parser* p) {
    Lexer_next_token(p->lexer);
}

void Parser_error(Parser* p, const char* msg) {
    char err[256];
    Token* cur = Parser_cur(p);
    sprintf(err, "%s (目前讀到: '%s')", msg, cur->text);
    report_error(p->lexer->src_str, cur->pos, err);
}

void Parser_expect(Parser* p, TokenType expected_type, const char* err_msg) {
    if (Parser_cur(p)->t_type == expected_type) {
        Parser_consume(p);
    } else {
        Parser_error(p, err_msg);
    }
}

char* Parser_new_t(Parser* p) {
    char buf[32];
    p->t_idx++;
    sprintf(buf, "t%zu", p->t_idx);
    return strdup(buf);
}

size_t Parser_emit(Parser* p, const char* op, const char* a1, const char* a2, const char* res) {
    if (p->quads_len >= p->quads_cap) {
        p->quads_cap *= 2;
        p->quads = (Quad*)realloc(p->quads, p->quads_cap * sizeof(Quad));
    }
    size_t idx = p->quads_len;
    p->quads[idx] = (Quad){ strdup(op), strdup(a1), strdup(a2), strdup(res) };
    p->quads_len++;
    printf("%03zu: %-12s %-10s %-10s %-10s\n", idx, op, a1, a2, res);
    return idx;
}

char* Parser_expression(Parser* p);

void Parser_expr_or_assign(Parser* p) {
    char* name = strdup(Parser_cur(p)->text);
    Parser_consume(p);
    
    char* obj = strdup(name);
    char** path = malloc(sizeof(char*) * 32);
    size_t path_len = 0;

    TokenType t;
    while ((t = Parser_cur(p)->t_type) == TK_LBRACKET || t == TK_DOT || t == TK_LPAREN) {
        if (t == TK_LBRACKET) {
            Parser_consume(p);
            char* idx = Parser_expression(p);
            Parser_expect(p, TK_RBRACKET, "預期 ']'");
            path[path_len++] = idx;
        } else if (t == TK_DOT) {
            Parser_consume(p);
            if (Parser_cur(p)->t_type != TK_ID) Parser_error(p, "預期屬性名稱");
            char* key_str = strdup(Parser_cur(p)->text);
            Parser_consume(p);
            
            char* k = Parser_new_t(p);
            size_t pool_idx = p->pool_len;
            if (p->pool_len >= p->pool_cap) { p->pool_cap *= 2; p->string_pool = realloc(p->string_pool, p->pool_cap * sizeof(char*)); }
            p->string_pool[p->pool_len++] = key_str;
            
            char pool_idx_str[32];
            sprintf(pool_idx_str, "%zu", pool_idx);
            Parser_emit(p, "LOAD_STR", pool_idx_str, "-", k);
            path[path_len++] = k;
        } else if (t == TK_LPAREN) {
            for (size_t i = 0; i < path_len; i++) {
                char* t_var = Parser_new_t(p);
                Parser_emit(p, "GET_ITEM", obj, path[i], t_var);
                obj = t_var;
            }
            path_len = 0;
            Parser_consume(p);
            size_t count = 0;
            if (Parser_cur(p)->t_type != TK_RPAREN) {
                while (1) {
                    char* arg = Parser_expression(p);
                    Parser_emit(p, "PARAM", arg, "-", "-");
                    count++;
                    if (Parser_cur(p)->t_type == TK_COMMA) Parser_consume(p);
                    else break;
                }
            }
            Parser_expect(p, TK_RPAREN, "預期 ')'");
            char* t_var = Parser_new_t(p);
            char count_str[32];
            sprintf(count_str, "%zu", count);
            Parser_emit(p, "CALL", obj, count_str, t_var);
            obj = t_var;
        }
    }

    if (Parser_cur(p)->t_type == TK_ASSIGN) {
        Parser_consume(p);
        char* val = Parser_expression(p);
        if (path_len == 0) {
            Parser_emit(p, "STORE", val, "-", obj);
        } else {
            for (size_t i = 0; i < path_len - 1; i++) {
                char* t_var = Parser_new_t(p);
                Parser_emit(p, "GET_ITEM", obj, path[i], t_var);
                obj = t_var;
            }
            Parser_emit(p, "SET_ITEM", obj, path[path_len - 1], val);
        }
    }
    free(path); // 釋放指標陣列，但指標內容交由系統管理
}

char* Parser_primary(Parser* p) {
    TokenType t = Parser_cur(p)->t_type;
    if (t == TK_NUM) {
        char* t_var = Parser_new_t(p);
        Parser_emit(p, "IMM", Parser_cur(p)->text, "-", t_var);
        Parser_consume(p);
        return t_var;
    } else if (t == TK_STRING) {
        char* t_var = Parser_new_t(p);
        size_t pool_idx = p->pool_len;
        if (p->pool_len >= p->pool_cap) { p->pool_cap *= 2; p->string_pool = realloc(p->string_pool, p->pool_cap * sizeof(char*)); }
        p->string_pool[p->pool_len++] = strdup(Parser_cur(p)->text);
        char pool_idx_str[32];
        sprintf(pool_idx_str, "%zu", pool_idx);
        Parser_emit(p, "LOAD_STR", pool_idx_str, "-", t_var);
        Parser_consume(p);
        return t_var;
    } else if (t == TK_ID) {
        char* name = strdup(Parser_cur(p)->text);
        Parser_consume(p);
        return name;
    } else if (t == TK_LBRACKET) {
        Parser_consume(p);
        char* t_var = Parser_new_t(p);
        Parser_emit(p, "NEW_ARR", "-", "-", t_var);
        if (Parser_cur(p)->t_type != TK_RBRACKET) {
            while (1) {
                char* val = Parser_expression(p);
                Parser_emit(p, "APPEND_ITEM", t_var, "-", val);
                if (Parser_cur(p)->t_type == TK_COMMA) Parser_consume(p);
                else break;
            }
        }
        Parser_expect(p, TK_RBRACKET, "陣列預期要有 ']' 結尾");
        return t_var;
    } else if (t == TK_LBRACE) {
        Parser_consume(p);
        char* t_var = Parser_new_t(p);
        Parser_emit(p, "NEW_DICT", "-", "-", t_var);
        if (Parser_cur(p)->t_type != TK_RBRACE) {
            while (1) {
                char* k;
                if (Parser_cur(p)->t_type == TK_ID) {
                    char* key_str = strdup(Parser_cur(p)->text);
                    Parser_consume(p);
                    k = Parser_new_t(p);
                    size_t pool_idx = p->pool_len;
                    if (p->pool_len >= p->pool_cap) { p->pool_cap *= 2; p->string_pool = realloc(p->string_pool, p->pool_cap * sizeof(char*)); }
                    p->string_pool[p->pool_len++] = key_str;
                    char pool_idx_str[32];
                    sprintf(pool_idx_str, "%zu", pool_idx);
                    Parser_emit(p, "LOAD_STR", pool_idx_str, "-", k);
                } else if (Parser_cur(p)->t_type == TK_STRING) {
                    k = Parser_primary(p);
                } else {
                    Parser_error(p, "字典的鍵必須是字串或識別碼");
                }
                Parser_expect(p, TK_COLON, "字典預期要有 ':'");
                char* val = Parser_expression(p);
                Parser_emit(p, "SET_ITEM", t_var, k, val);
                if (Parser_cur(p)->t_type == TK_COMMA) Parser_consume(p);
                else break;
            }
        }
        Parser_expect(p, TK_RBRACE, "字典預期要有 '}' 結尾");
        return t_var;
    } else if (t == TK_LPAREN) {
        Parser_consume(p);
        char* res = Parser_expression(p);
        Parser_expect(p, TK_RPAREN, "預期要有 ')'");
        return res;
    }
    Parser_error(p, "表達式中出現預期外的語法結構");
    return NULL;
}

char* Parser_factor(Parser* p) {
    char* res = Parser_primary(p);
    TokenType t;
    while ((t = Parser_cur(p)->t_type) == TK_LBRACKET || t == TK_DOT || t == TK_LPAREN) {
        if (t == TK_LBRACKET) {
            Parser_consume(p);
            char* idx = Parser_expression(p);
            Parser_expect(p, TK_RBRACKET, "預期 ']'");
            char* t_var = Parser_new_t(p);
            Parser_emit(p, "GET_ITEM", res, idx, t_var);
            res = t_var;
        } else if (t == TK_DOT) {
            Parser_consume(p);
            char* key_str = strdup(Parser_cur(p)->text);
            Parser_consume(p);
            char* k = Parser_new_t(p);
            size_t pool_idx = p->pool_len;
            if (p->pool_len >= p->pool_cap) { p->pool_cap *= 2; p->string_pool = realloc(p->string_pool, p->pool_cap * sizeof(char*)); }
            p->string_pool[p->pool_len++] = key_str;
            char pool_idx_str[32];
            sprintf(pool_idx_str, "%zu", pool_idx);
            Parser_emit(p, "LOAD_STR", pool_idx_str, "-", k);
            
            char* t_var = Parser_new_t(p);
            Parser_emit(p, "GET_ITEM", res, k, t_var);
            res = t_var;
        } else if (t == TK_LPAREN) {
            Parser_consume(p);
            size_t count = 0;
            if (Parser_cur(p)->t_type != TK_RPAREN) {
                while (1) {
                    char* arg = Parser_expression(p);
                    Parser_emit(p, "PARAM", arg, "-", "-");
                    count++;
                    if (Parser_cur(p)->t_type == TK_COMMA) Parser_consume(p);
                    else break;
                }
            }
            Parser_expect(p, TK_RPAREN, "預期 ')'");
            char* t_var = Parser_new_t(p);
            char count_str[32];
            sprintf(count_str, "%zu", count);
            Parser_emit(p, "CALL", res, count_str, t_var);
            res = t_var;
        }
    }
    return res;
}

char* Parser_term(Parser* p) {
    char* l = Parser_factor(p);
    TokenType t;
    while ((t = Parser_cur(p)->t_type) == TK_MUL || t == TK_DIV) {
        const char* op = (t == TK_MUL) ? "MUL" : "DIV";
        Parser_consume(p);
        char* r = Parser_factor(p);
        char* t_var = Parser_new_t(p);
        Parser_emit(p, op, l, r, t_var);
        l = t_var;
    }
    return l;
}

char* Parser_arith_expr(Parser* p) {
    char* l = Parser_term(p);
    TokenType t;
    while ((t = Parser_cur(p)->t_type) == TK_PLUS || t == TK_MINUS) {
        const char* op = (t == TK_PLUS) ? "ADD" : "SUB";
        Parser_consume(p);
        char* r = Parser_term(p);
        char* t_var = Parser_new_t(p);
        Parser_emit(p, op, l, r, t_var);
        l = t_var;
    }
    return l;
}

char* Parser_expression(Parser* p) {
    char* l = Parser_arith_expr(p);
    TokenType t = Parser_cur(p)->t_type;
    if (t == TK_EQ || t == TK_LT || t == TK_GT) {
        const char* op = (t == TK_EQ) ? "CMP_EQ" : (t == TK_LT) ? "CMP_LT" : "CMP_GT";
        Parser_consume(p);
        char* r = Parser_arith_expr(p);
        char* t_var = Parser_new_t(p);
        Parser_emit(p, op, l, r, t_var);
        return t_var;
    }
    return l;
}

void Parser_statement(Parser* p) {
    TokenType t = Parser_cur(p)->t_type;
    if (t == TK_IF) {
        Parser_consume(p); Parser_expect(p, TK_LPAREN, "預期 '('");
        char* cond = Parser_expression(p);
        Parser_expect(p, TK_RPAREN, "預期 ')'"); Parser_expect(p, TK_LBRACE, "預期 '{'");
        
        size_t jmp_f_idx = Parser_emit(p, "JMP_F", cond, "-", "?");
        while (Parser_cur(p)->t_type != TK_RBRACE && Parser_cur(p)->t_type != TK_EOF) {
            Parser_statement(p);
        }
        Parser_expect(p, TK_RBRACE, "預期 '}'");
        
        char end_idx_str[32];
        sprintf(end_idx_str, "%zu", p->quads_len);
        p->quads[jmp_f_idx].result = strdup(end_idx_str);
    } else if (t == TK_WHILE) {
        Parser_consume(p); Parser_expect(p, TK_LPAREN, "預期 '('");
        size_t cond_idx = p->quads_len;
        char* cond = Parser_expression(p);
        Parser_expect(p, TK_RPAREN, "預期 ')'"); Parser_expect(p, TK_LBRACE, "預期 '{'");
        
        size_t jmp_f_idx = Parser_emit(p, "JMP_F", cond, "-", "?");
        
        if (p->loop_len >= p->loop_cap) { p->loop_cap *= 2; p->loop_stack = realloc(p->loop_stack, p->loop_cap * sizeof(LoopCtx)); }
        LoopCtx ctx;
        ctx.breaks_cap = 4; ctx.breaks_len = 0;
        ctx.breaks = (size_t*)malloc(ctx.breaks_cap * sizeof(size_t));
        ctx.continue_target = cond_idx;
        p->loop_stack[p->loop_len++] = ctx;
        
        while (Parser_cur(p)->t_type != TK_RBRACE && Parser_cur(p)->t_type != TK_EOF) {
            Parser_statement(p);
        }
        
        char cond_idx_str[32];
        sprintf(cond_idx_str, "%zu", cond_idx);
        Parser_emit(p, "JMP", "-", "-", cond_idx_str);
        Parser_expect(p, TK_RBRACE, "預期 '}'");
        
        char end_idx_str[32];
        sprintf(end_idx_str, "%zu", p->quads_len);
        p->quads[jmp_f_idx].result = strdup(end_idx_str);
        
        LoopCtx pop_ctx = p->loop_stack[--p->loop_len];
        for (size_t i = 0; i < pop_ctx.breaks_len; i++) {
            p->quads[pop_ctx.breaks[i]].result = strdup(end_idx_str);
        }
        free(pop_ctx.breaks);
    } else if (t == TK_FOR) {
        Parser_consume(p); Parser_expect(p, TK_LPAREN, "預期 '('");
        if (Parser_cur(p)->t_type != TK_SEMICOLON) Parser_expr_or_assign(p);
        Parser_expect(p, TK_SEMICOLON, "預期 ';'");
        
        size_t cond_idx = p->quads_len;
        char* cond;
        if (Parser_cur(p)->t_type != TK_SEMICOLON) {
            cond = Parser_expression(p);
        } else {
            cond = Parser_new_t(p);
            Parser_emit(p, "IMM", "1", "-", cond);
        }
        
        size_t jmp_f_idx = Parser_emit(p, "JMP_F", cond, "-", "?");
        size_t jmp_body_idx = Parser_emit(p, "JMP", "-", "-", "?");
        
        Parser_expect(p, TK_SEMICOLON, "預期 ';'");
        
        size_t step_idx = p->quads_len;
        if (Parser_cur(p)->t_type != TK_RPAREN) Parser_expr_or_assign(p);
        
        char cond_idx_str[32];
        sprintf(cond_idx_str, "%zu", cond_idx);
        Parser_emit(p, "JMP", "-", "-", cond_idx_str);
        
        Parser_expect(p, TK_RPAREN, "預期 ')'"); Parser_expect(p, TK_LBRACE, "預期 '{'");
        
        char len_str[32];
        sprintf(len_str, "%zu", p->quads_len);
        p->quads[jmp_body_idx].result = strdup(len_str);
        
        if (p->loop_len >= p->loop_cap) { p->loop_cap *= 2; p->loop_stack = realloc(p->loop_stack, p->loop_cap * sizeof(LoopCtx)); }
        LoopCtx ctx;
        ctx.breaks_cap = 4; ctx.breaks_len = 0;
        ctx.breaks = (size_t*)malloc(ctx.breaks_cap * sizeof(size_t));
        ctx.continue_target = step_idx;
        p->loop_stack[p->loop_len++] = ctx;
        
        while (Parser_cur(p)->t_type != TK_RBRACE && Parser_cur(p)->t_type != TK_EOF) {
            Parser_statement(p);
        }
        
        char step_idx_str[32];
        sprintf(step_idx_str, "%zu", step_idx);
        Parser_emit(p, "JMP", "-", "-", step_idx_str);
        Parser_expect(p, TK_RBRACE, "預期 '}'");
        
        char end_idx_str[32];
        sprintf(end_idx_str, "%zu", p->quads_len);
        p->quads[jmp_f_idx].result = strdup(end_idx_str);
        
        LoopCtx pop_ctx = p->loop_stack[--p->loop_len];
        for (size_t i = 0; i < pop_ctx.breaks_len; i++) {
            p->quads[pop_ctx.breaks[i]].result = strdup(end_idx_str);
        }
        free(pop_ctx.breaks);
    } else if (t == TK_BREAK) {
        Parser_consume(p);
        if (p->loop_len == 0) Parser_error(p, "break 必須在迴圈內部使用");
        size_t b_idx = Parser_emit(p, "JMP", "-", "-", "?");
        LoopCtx* ctx = &p->loop_stack[p->loop_len - 1];
        if (ctx->breaks_len >= ctx->breaks_cap) { ctx->breaks_cap *= 2; ctx->breaks = realloc(ctx->breaks, ctx->breaks_cap * sizeof(size_t)); }
        ctx->breaks[ctx->breaks_len++] = b_idx;
        Parser_expect(p, TK_SEMICOLON, "預期 ';'");
    } else if (t == TK_CONTINUE) {
        Parser_consume(p);
        if (p->loop_len == 0) Parser_error(p, "continue 必須在迴圈內部使用");
        char c_target[32];
        sprintf(c_target, "%zu", p->loop_stack[p->loop_len - 1].continue_target);
        Parser_emit(p, "JMP", "-", "-", c_target);
        Parser_expect(p, TK_SEMICOLON, "預期 ';'");
    } else if (t == TK_ID) {
        Parser_expr_or_assign(p);
        Parser_expect(p, TK_SEMICOLON, "預期 ';'");
    } else if (t == TK_RETURN) {
        Parser_consume(p);
        char* res = Parser_expression(p);
        Parser_emit(p, "RET_VAL", res, "-", "-");
        Parser_expect(p, TK_SEMICOLON, "預期 ';'");
    } else if (t == TK_PRINT) {
        Parser_consume(p); Parser_expect(p, TK_LPAREN, "預期 '('");
        if (Parser_cur(p)->t_type != TK_RPAREN) {
            while (1) {
                char* val = Parser_expression(p);
                Parser_emit(p, "PRINT_VAL", val, "-", "-");
                if (Parser_cur(p)->t_type == TK_COMMA) Parser_consume(p);
                else break;
            }
        }
        Parser_emit(p, "PRINT_NL", "-", "-", "-");
        Parser_expect(p, TK_RPAREN, "預期 ')'"); Parser_expect(p, TK_SEMICOLON, "預期 ';'");
    } else {
        Parser_error(p, "無法辨識的陳述句或語法結構");
    }
}

void Parser_parse_program(Parser* p) {
    while (Parser_cur(p)->t_type != TK_EOF) {
        if (Parser_cur(p)->t_type == TK_FUNC) {
            Parser_consume(p);
            char* f_name = strdup(Parser_cur(p)->text); Parser_consume(p);
            Parser_emit(p, "FUNC_BEG", f_name, "-", "-");
            Parser_expect(p, TK_LPAREN, "預期 '('");
            if (Parser_cur(p)->t_type != TK_RPAREN) {
                while (1) {
                    Parser_emit(p, "FORMAL", Parser_cur(p)->text, "-", "-");
                    Parser_consume(p);
                    if (Parser_cur(p)->t_type == TK_COMMA) Parser_consume(p);
                    else break;
                }
            }
            Parser_expect(p, TK_RPAREN, "預期 ')'"); Parser_expect(p, TK_LBRACE, "預期 '{'");
            while (Parser_cur(p)->t_type != TK_RBRACE && Parser_cur(p)->t_type != TK_EOF) {
                Parser_statement(p);
            }
            Parser_emit(p, "FUNC_END", f_name, "-", "-");
            Parser_expect(p, TK_RBRACE, "預期 '}'");
        } else {
            Parser_statement(p);
        }
    }
}

// =========================================================
// 字串化工具 (Value Display)
// =========================================================
int compare_dict_keys(const void* a, const void* b) {
    MapEntry* ea = (MapEntry*)a;
    MapEntry* eb = (MapEntry*)b;
    return strcmp(ea->key, eb->key);
}

char* value_to_string(Value v) {
    StrBuf sb; StrBuf_init(&sb);
    char buf[64];
    
    switch (v.type) {
        case VAL_INT:
            sprintf(buf, "%lld", (long long)v.as.i);
            StrBuf_append(&sb, buf);
            break;
        case VAL_STR:
            StrBuf_append(&sb, v.as.s);
            break;
        case VAL_ARR:
            StrBuf_append(&sb, "[");
            for (size_t i = 0; i < v.as.arr->len; i++) {
                if (i > 0) StrBuf_append(&sb, ", ");
                char* child = value_to_string(v.as.arr->data[i]);
                StrBuf_append(&sb, child);
                free(child);
            }
            StrBuf_append(&sb, "]");
            break;
        case VAL_DICT: {
            StrBuf_append(&sb, "{");
            size_t n = v.as.dict->map.len;
            MapEntry* sorted = (MapEntry*)malloc(n * sizeof(MapEntry));
            memcpy(sorted, v.as.dict->map.data, n * sizeof(MapEntry));
            qsort(sorted, n, sizeof(MapEntry), compare_dict_keys);
            
            for (size_t i = 0; i < n; i++) {
                if (i > 0) StrBuf_append(&sb, ", ");
                sprintf(buf, "'%s': ", sorted[i].key);
                StrBuf_append(&sb, buf);
                char* child = value_to_string(sorted[i].val);
                StrBuf_append(&sb, child);
                free(child);
            }
            free(sorted);
            StrBuf_append(&sb, "}");
            break;
        }
        case VAL_NIL:
            StrBuf_append(&sb, "0");
            break;
    }
    return sb.data;
}

// =========================================================
// 4. 虛擬機 (Virtual Machine)
// =========================================================
typedef struct {
    MapVec vars;
    size_t ret_pc;
    char* ret_var;
    Value* incoming_args;
    size_t incoming_len;
    size_t formal_idx;
} Frame;

typedef struct {
    Quad* quads;
    size_t quads_len;
    char** string_pool;
    
    Frame* stack;
    size_t stack_len;
    size_t stack_cap;
    
    char** print_buf;
    size_t print_len;
    size_t print_cap;
} VM;

void VM_init(VM* vm, Quad* quads, size_t quads_len, char** pool) {
    vm->quads = quads;
    vm->quads_len = quads_len;
    vm->string_pool = pool;
    
    vm->stack_cap = 64; vm->stack_len = 0;
    vm->stack = (Frame*)malloc(vm->stack_cap * sizeof(Frame));
    
    Frame global_frame;
    Map_init(&global_frame.vars);
    global_frame.ret_pc = 0;
    global_frame.ret_var = strdup("");
    global_frame.incoming_args = NULL;
    global_frame.incoming_len = 0;
    global_frame.formal_idx = 0;
    vm->stack[vm->stack_len++] = global_frame;
    
    vm->print_cap = 16; vm->print_len = 0;
    vm->print_buf = (char**)malloc(vm->print_cap * sizeof(char*));
}

Value VM_get_var(VM* vm, const char* name) {
    char* end;
    long long parsed = strtoll(name, &end, 10);
    if (*end == '\0') {
        return (Value){ VAL_INT, .as.i = parsed };
    }
    if (name[0] == '-' && name[1] != '\0') {
        long long neg_parsed = strtoll(name + 1, &end, 10);
        if (*end == '\0') return (Value){ VAL_INT, .as.i = -neg_parsed };
    }
    if (strcmp(name, "-") == 0) return (Value){ VAL_INT, .as.i = 0 };
    
    Frame* current = &vm->stack[vm->stack_len - 1];
    return Map_get(&current->vars, name);
}

void VM_set_var(VM* vm, const char* name, Value val) {
    Frame* current = &vm->stack[vm->stack_len - 1];
    Map_set(&current->vars, name, val);
}

void VM_run(VM* vm) {
    size_t pc = 0;
    
    Value* param_stack = malloc(256 * sizeof(Value));
    size_t param_len = 0;
    
    MapVec func_map; Map_init(&func_map);
    
    for (size_t i = 0; i < vm->quads_len; i++) {
        if (strcmp(vm->quads[i].op, "FUNC_BEG") == 0) {
            Map_set(&func_map, vm->quads[i].arg1, (Value){ VAL_INT, .as.i = i + 1 });
        }
    }
    
    printf("\n=== VM 執行開始 ===\n");
    
    while (pc < vm->quads_len) {
        Quad* q = &vm->quads[pc];
        const char* op = q->op;
        
        if (strcmp(op, "FUNC_BEG") == 0) {
            while (strcmp(vm->quads[pc].op, "FUNC_END") != 0) pc++;
        }
        else if (strcmp(op, "IMM") == 0) {
            VM_set_var(vm, q->result, (Value){ VAL_INT, .as.i = atoll(q->arg1) });
        }
        else if (strcmp(op, "LOAD_STR") == 0) {
            size_t idx = atoll(q->arg1);
            VM_set_var(vm, q->result, (Value){ VAL_STR, .as.s = strdup(vm->string_pool[idx]) });
        }
        else if (strcmp(op, "ADD") == 0) {
            long long a = VM_get_var(vm, q->arg1).as.i;
            long long b = VM_get_var(vm, q->arg2).as.i;
            VM_set_var(vm, q->result, (Value){ VAL_INT, .as.i = a + b });
        }
        else if (strcmp(op, "SUB") == 0) {
            long long a = VM_get_var(vm, q->arg1).as.i;
            long long b = VM_get_var(vm, q->arg2).as.i;
            VM_set_var(vm, q->result, (Value){ VAL_INT, .as.i = a - b });
        }
        else if (strcmp(op, "MUL") == 0) {
            long long a = VM_get_var(vm, q->arg1).as.i;
            long long b = VM_get_var(vm, q->arg2).as.i;
            VM_set_var(vm, q->result, (Value){ VAL_INT, .as.i = a * b });
        }
        else if (strcmp(op, "DIV") == 0) {
            long long a = VM_get_var(vm, q->arg1).as.i;
            long long b = VM_get_var(vm, q->arg2).as.i;
            VM_set_var(vm, q->result, (Value){ VAL_INT, .as.i = a / (b == 0 ? 1 : b) });
        }
        else if (strcmp(op, "CMP_EQ") == 0) {
            long long a = VM_get_var(vm, q->arg1).as.i;
            long long b = VM_get_var(vm, q->arg2).as.i;
            VM_set_var(vm, q->result, (Value){ VAL_INT, .as.i = (a == b) ? 1 : 0 });
        }
        else if (strcmp(op, "CMP_LT") == 0) {
            long long a = VM_get_var(vm, q->arg1).as.i;
            long long b = VM_get_var(vm, q->arg2).as.i;
            VM_set_var(vm, q->result, (Value){ VAL_INT, .as.i = (a < b) ? 1 : 0 });
        }
        else if (strcmp(op, "CMP_GT") == 0) {
            long long a = VM_get_var(vm, q->arg1).as.i;
            long long b = VM_get_var(vm, q->arg2).as.i;
            VM_set_var(vm, q->result, (Value){ VAL_INT, .as.i = (a > b) ? 1 : 0 });
        }
        else if (strcmp(op, "STORE") == 0) {
            VM_set_var(vm, q->result, VM_get_var(vm, q->arg1));
        }
        else if (strcmp(op, "NEW_ARR") == 0) {
            VArray* arr = (VArray*)malloc(sizeof(VArray));
            arr->len = 0; arr->cap = 8;
            arr->data = (Value*)malloc(arr->cap * sizeof(Value));
            VM_set_var(vm, q->result, (Value){ VAL_ARR, .as.arr = arr });
        }
        else if (strcmp(op, "NEW_DICT") == 0) {
            VDict* dict = (VDict*)malloc(sizeof(VDict));
            Map_init(&dict->map);
            VM_set_var(vm, q->result, (Value){ VAL_DICT, .as.dict = dict });
        }
        else if (strcmp(op, "APPEND_ITEM") == 0) {
            Value obj = VM_get_var(vm, q->arg1);
            if (obj.type == VAL_ARR) {
                VArray* arr = obj.as.arr;
                if (arr->len >= arr->cap) { arr->cap *= 2; arr->data = realloc(arr->data, arr->cap * sizeof(Value)); }
                arr->data[arr->len++] = VM_get_var(vm, q->result);
            }
        }
        else if (strcmp(op, "SET_ITEM") == 0) {
            Value obj = VM_get_var(vm, q->arg1);
            Value val = VM_get_var(vm, q->result);
            if (obj.type == VAL_ARR) {
                size_t idx = VM_get_var(vm, q->arg2).as.i;
                obj.as.arr->data[idx] = val;
            } else if (obj.type == VAL_DICT) {
                char* key = VM_get_var(vm, q->arg2).as.s;
                Map_set(&obj.as.dict->map, key, val);
            }
        }
        else if (strcmp(op, "GET_ITEM") == 0) {
            Value obj = VM_get_var(vm, q->arg1);
            Value val = (Value){ VAL_NIL, .as={0} };
            if (obj.type == VAL_ARR) {
                size_t idx = VM_get_var(vm, q->arg2).as.i;
                val = obj.as.arr->data[idx];
            } else if (obj.type == VAL_DICT) {
                char* key = VM_get_var(vm, q->arg2).as.s;
                val = Map_get(&obj.as.dict->map, key);
            }
            VM_set_var(vm, q->result, val);
        }
        else if (strcmp(op, "JMP") == 0) {
            pc = atoll(q->result) - 1;
        }
        else if (strcmp(op, "JMP_F") == 0) {
            if (VM_get_var(vm, q->arg1).as.i == 0) {
                pc = atoll(q->result) - 1;
            }
        }
        else if (strcmp(op, "PRINT_VAL") == 0) {
            if (vm->print_len >= vm->print_cap) { vm->print_cap *= 2; vm->print_buf = realloc(vm->print_buf, vm->print_cap * sizeof(char*)); }
            vm->print_buf[vm->print_len++] = value_to_string(VM_get_var(vm, q->arg1));
        }
        else if (strcmp(op, "PRINT_NL") == 0) {
            printf("[程式輸出] >> ");
            for (size_t i = 0; i < vm->print_len; i++) {
                if (i > 0) printf(" ");
                printf("%s", vm->print_buf[i]);
                free(vm->print_buf[i]);
            }
            printf("\n");
            vm->print_len = 0;
        }
        else if (strcmp(op, "PARAM") == 0) {
            param_stack[param_len++] = VM_get_var(vm, q->arg1);
        }
        else if (strcmp(op, "CALL") == 0) {
            size_t p_count = atoll(q->arg2);
            Value f_val = VM_get_var(vm, q->arg1);
            const char* f_name = (f_val.type == VAL_STR) ? f_val.as.s : q->arg1;
            
            Value target_val = Map_get(&func_map, f_name);
            if (target_val.type == VAL_INT && target_val.as.i != 0) {
                Frame new_frame;
                Map_init(&new_frame.vars);
                new_frame.ret_pc = pc + 1;
                new_frame.ret_var = strdup(q->result);
                new_frame.incoming_len = p_count;
                new_frame.incoming_args = NULL;
                new_frame.formal_idx = 0;
                
                if (p_count > 0) {
                    new_frame.incoming_args = (Value*)malloc(p_count * sizeof(Value));
                    size_t start = param_len - p_count;
                    for (size_t i = 0; i < p_count; i++) {
                        new_frame.incoming_args[i] = param_stack[start + i];
                    }
                    param_len -= p_count;
                }
                
                if (vm->stack_len >= vm->stack_cap) { vm->stack_cap *= 2; vm->stack = realloc(vm->stack, vm->stack_cap * sizeof(Frame)); }
                vm->stack[vm->stack_len++] = new_frame;
                pc = target_val.as.i;
                continue;
            } else {
                printf("找不到函數 '%s'\n", f_name);
                exit(1);
            }
        }
        else if (strcmp(op, "FORMAL") == 0) {
            Frame* frame = &vm->stack[vm->stack_len - 1];
            Value arg_val = frame->incoming_args[frame->formal_idx];
            Map_set(&frame->vars, q->arg1, arg_val);
            frame->formal_idx++;
        }
        else if (strcmp(op, "RET_VAL") == 0) {
            Value ret_val = VM_get_var(vm, q->arg1);
            Frame frame = vm->stack[--vm->stack_len];
            size_t ret_address = frame.ret_pc;
            char* target_var = frame.ret_var;
            VM_set_var(vm, target_var, ret_val);
            pc = ret_address;
            free(frame.incoming_args);
            continue;
        }
        else if (strcmp(op, "FUNC_END") == 0) {
            if (vm->stack_len > 1) {
                Frame frame = vm->stack[--vm->stack_len];
                size_t ret_address = frame.ret_pc;
                char* target_var = frame.ret_var;
                VM_set_var(vm, target_var, (Value){ VAL_NIL, .as={0} });
                pc = ret_address;
                free(frame.incoming_args);
                continue;
            }
        }
        else {
            printf("未知的指令: %s\n", op);
            exit(1);
        }
        pc++;
    }

    printf("=== VM 執行完畢 ===\n\n記憶體狀態 (全域變數):\n");
    MapVec* global_vars = &vm->stack[0].vars;
    for (size_t i = 0; i < global_vars->len; i++) {
        if (global_vars->data[i].key[0] != 't') {
            char* s = value_to_string(global_vars->data[i].val);
            printf("[%s] = %s\n", global_vars->data[i].key, s);
            free(s);
        }
    }
}

// =========================================================
// 主程式
// =========================================================
int main(int argc, char** argv) {
    if (argc < 2) {
        printf("用法: %s <source_file>\n", argv[0]);
        exit(1);
    }

    FILE* f = fopen(argv[1], "rb");
    if (!f) {
        printf("無法開啟檔案: %s\n", argv[1]);
        exit(1);
    }

    fseek(f, 0, SEEK_END);
    long fsize = ftell(f);
    fseek(f, 0, SEEK_SET);

    char* source_code = (char*)malloc(fsize + 1);
    fread(source_code, 1, fsize, f);
    source_code[fsize] = '\0';
    fclose(f);

    printf("編譯器生成的中間碼 (PC: Quadruples):\n");
    printf("--------------------------------------------\n");

    Lexer lexer;
    Lexer_init(&lexer, source_code);
    Lexer_next_token(&lexer);

    Parser parser;
    Parser_init(&parser, &lexer);
    Parser_parse_program(&parser);

    VM vm;
    VM_init(&vm, parser.quads, parser.quads_len, parser.string_pool);
    VM_run(&vm);

    // 備註：此腳本引擎實作依賴作業系統在執行完畢時回收動態配置的記憶體 (AST / VM Objects)。
    free(source_code);
    return 0;
}