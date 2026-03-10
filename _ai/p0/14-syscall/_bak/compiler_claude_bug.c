#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <stdbool.h>

#define MAX_TOKEN_LEN 256
#define MAX_STRING_POOL 100
#define MAX_VARS 100
#define MAX_STACK_DEPTH 50
#define MAX_LOOP_DEPTH 20
#define MAX_PARAMS 20

void report_error(const char *src, int pos, const char *msg) {
    printf("\n❌ [語法錯誤] 位置 %d: %s\n", pos, msg);
    exit(1);
}

typedef enum {
    TK_FUNC, TK_RETURN, TK_IF, TK_PRINT, TK_WHILE, TK_FOR, TK_BREAK, TK_CONTINUE,
    TK_ID, TK_NUM, TK_STRING,
    TK_LPAREN, TK_RPAREN, TK_LBRACE, TK_RBRACE, TK_LBRACKET, TK_RBRACKET,
    TK_DOT, TK_COLON, TK_COMMA, TK_SEMICOLON,
    TK_ASSIGN, TK_PLUS, TK_MINUS, TK_MUL, TK_DIV,
    TK_EQ, TK_LT, TK_GT, TK_EOF
} TokenType;

typedef struct {
    TokenType type;
    char text[MAX_TOKEN_LEN];
    int pos;
} Token;

typedef struct {
    char op[16];
    char arg1[MAX_TOKEN_LEN];
    char arg2[MAX_TOKEN_LEN];
    char result[MAX_TOKEN_LEN];
} Quad;

typedef struct {
    const char *src;
    int pos;
    Token cur_token;
} Lexer;

void lexer_skip_whitespace(Lexer *lex) {
    while (lex->src[lex->pos] != '\0' && isspace(lex->src[lex->pos])) {
        lex->pos++;
    }
}

void lexer_next_token(Lexer *lex);

void lexer_init(Lexer *lex, const char *src) {
    lex->src = src;
    lex->pos = 0;
    lexer_next_token(lex);
}

void lexer_next_token(Lexer *lex) {
    while (true) {
        lexer_skip_whitespace(lex);
        
        if (lex->src[lex->pos] == '\0') {
            lex->cur_token.type = TK_EOF;
            lex->cur_token.text[0] = '\0';
            lex->cur_token.pos = lex->pos;
            return;
        }
        
        if (lex->src[lex->pos] == '/') {
            if (lex->src[lex->pos + 1] == '/') {
                lex->pos += 2;
                while (lex->src[lex->pos] != '\0' && lex->src[lex->pos] != '\n') lex->pos++;
                continue;
            } else if (lex->src[lex->pos + 1] == '*') {
                lex->pos += 2;
                while (lex->src[lex->pos] != '\0' && lex->src[lex->pos + 1] != '\0' &&
                       !(lex->src[lex->pos] == '*' && lex->src[lex->pos + 1] == '/')) lex->pos++;
                if (lex->src[lex->pos] != '\0' && lex->src[lex->pos + 1] != '\0') lex->pos += 2;
                continue;
            }
        }
        break;
    }
    
    int start = lex->pos;
    
    if (lex->src[lex->pos] == '"') {
        lex->pos++;
        int str_start = lex->pos;
        while (lex->src[lex->pos] != '\0' && lex->src[lex->pos] != '"') lex->pos++;
        if (lex->src[lex->pos] == '\0') report_error(lex->src, start, "字串缺少結尾的雙引號");
        int len = lex->pos - str_start;
        strncpy(lex->cur_token.text, &lex->src[str_start], len);
        lex->cur_token.text[len] = '\0';
        lex->pos++;
        lex->cur_token.type = TK_STRING;
        lex->cur_token.pos = start;
        return;
    }
    
    if (isdigit(lex->src[lex->pos])) {
        while (lex->src[lex->pos] != '\0' && isdigit(lex->src[lex->pos])) lex->pos++;
        int len = lex->pos - start;
        strncpy(lex->cur_token.text, &lex->src[start], len);
        lex->cur_token.text[len] = '\0';
        lex->cur_token.type = TK_NUM;
        lex->cur_token.pos = start;
        return;
    }
    
    if (isalpha(lex->src[lex->pos]) || lex->src[lex->pos] == '_') {
        while (lex->src[lex->pos] != '\0' && (isalnum(lex->src[lex->pos]) || lex->src[lex->pos] == '_')) lex->pos++;
        int len = lex->pos - start;
        strncpy(lex->cur_token.text, &lex->src[start], len);
        lex->cur_token.text[len] = '\0';
        lex->cur_token.pos = start;
        
        if (strcmp(lex->cur_token.text, "func") == 0) lex->cur_token.type = TK_FUNC;
        else if (strcmp(lex->cur_token.text, "return") == 0) lex->cur_token.type = TK_RETURN;
        else if (strcmp(lex->cur_token.text, "if") == 0) lex->cur_token.type = TK_IF;
        else if (strcmp(lex->cur_token.text, "print") == 0) lex->cur_token.type = TK_PRINT;
        else if (strcmp(lex->cur_token.text, "while") == 0) lex->cur_token.type = TK_WHILE;
        else if (strcmp(lex->cur_token.text, "for") == 0) lex->cur_token.type = TK_FOR;
        else if (strcmp(lex->cur_token.text, "break") == 0) lex->cur_token.type = TK_BREAK;
        else if (strcmp(lex->cur_token.text, "continue") == 0) lex->cur_token.type = TK_CONTINUE;
        else lex->cur_token.type = TK_ID;
        return;
    }
    
    char ch = lex->src[lex->pos++];
    lex->cur_token.text[0] = ch;
    lex->cur_token.text[1] = '\0';
    lex->cur_token.pos = start;
    
    switch (ch) {
        case '(': lex->cur_token.type = TK_LPAREN; break;
        case ')': lex->cur_token.type = TK_RPAREN; break;
        case '{': lex->cur_token.type = TK_LBRACE; break;
        case '}': lex->cur_token.type = TK_RBRACE; break;
        case '[': lex->cur_token.type = TK_LBRACKET; break;
        case ']': lex->cur_token.type = TK_RBRACKET; break;
        case '.': lex->cur_token.type = TK_DOT; break;
        case ':': lex->cur_token.type = TK_COLON; break;
        case ',': lex->cur_token.type = TK_COMMA; break;
        case ';': lex->cur_token.type = TK_SEMICOLON; break;
        case '+': lex->cur_token.type = TK_PLUS; break;
        case '-': lex->cur_token.type = TK_MINUS; break;
        case '*': lex->cur_token.type = TK_MUL; break;
        case '/': lex->cur_token.type = TK_DIV; break;
        case '<': lex->cur_token.type = TK_LT; break;
        case '>': lex->cur_token.type = TK_GT; break;
        case '=':
            if (lex->src[lex->pos] != '\0' && lex->src[lex->pos] == '=') {
                lex->pos++;
                strcpy(lex->cur_token.text, "==");
                lex->cur_token.type = TK_EQ;
            } else {
                lex->cur_token.type = TK_ASSIGN;
            }
            break;
        default: {
            char err_msg[100];
            sprintf(err_msg, "無法辨識的字元: '%c'", ch);
            report_error(lex->src, start, err_msg);
        }
    }
}

typedef struct {
    int break_targets[100];
    int break_count;
    int continue_target;
} LoopContext;

typedef struct {
    Lexer *lexer;
    Quad *quads;
    int quad_count;
    int quad_capacity;
    char (*string_pool)[MAX_TOKEN_LEN];
    int string_count;
    LoopContext loop_stack[MAX_LOOP_DEPTH];
    int loop_depth;
    int t_idx;
} Parser;

void parser_error(Parser *p, const char *msg) {
    char full_msg[512];
    sprintf(full_msg, "%s (目前讀到: '%s')", msg, p->lexer->cur_token.text);
    report_error(p->lexer->src, p->lexer->cur_token.pos, full_msg);
}

void parser_consume(Parser *p) {
    lexer_next_token(p->lexer);
}

void parser_expect(Parser *p, TokenType type, const char *msg) {
    if (p->lexer->cur_token.type == type) {
        parser_consume(p);
    } else {
        parser_error(p, msg);
    }
}

void parser_new_t(Parser *p, char *result) {
    p->t_idx++;
    sprintf(result, "t%d", p->t_idx);
}

int parser_emit(Parser *p, const char *op, const char *a1, const char *a2, const char *res) {
    if (p->quad_count >= p->quad_capacity) {
        p->quad_capacity *= 2;
        p->quads = (Quad *)realloc(p->quads, p->quad_capacity * sizeof(Quad));
        if (!p->quads) {
            printf("記憶體配置失敗\n");
            exit(1);
        }
    }
    
    int idx = p->quad_count;
    strcpy(p->quads[idx].op, op);
    strcpy(p->quads[idx].arg1, a1);
    strcpy(p->quads[idx].arg2, a2);
    strcpy(p->quads[idx].result, res);
    p->quad_count++;
    printf("%03d: %-12s %-10s %-10s %-10s\n", idx, op, a1, a2, res);
    return idx;
}

char* parser_expression(Parser *p, char *result);
void parser_statement(Parser *p);
char* parser_primary(Parser *p, char *result);
char* parser_term(Parser *p, char *result);
char* parser_arith(Parser *p, char *result);

char* parser_primary(Parser *p, char *result) {
    if (p->lexer->cur_token.type == TK_NUM) {
        parser_new_t(p, result);
        parser_emit(p, "IMM", p->lexer->cur_token.text, "-", result);
        parser_consume(p);
        return result;
    } else if (p->lexer->cur_token.type == TK_STRING) {
        int str_idx = p->string_count;
        strcpy(p->string_pool[str_idx], p->lexer->cur_token.text);
        p->string_count++;
        
        parser_new_t(p, result);
        char idx_str[16];
        sprintf(idx_str, "%d", str_idx);
        parser_emit(p, "LOAD_STR", idx_str, "-", result);
        parser_consume(p);
        return result;
    } else if (p->lexer->cur_token.type == TK_ID) {
        char name[MAX_TOKEN_LEN];
        strcpy(name, p->lexer->cur_token.text);
        parser_consume(p);
        
        if (p->lexer->cur_token.type == TK_LPAREN) {
            parser_consume(p);
            int param_count = 0;
            
            if (p->lexer->cur_token.type != TK_RPAREN) {
                while (true) {
                    char arg[MAX_TOKEN_LEN];
                    parser_new_t(p, arg);
                    parser_expression(p, arg);
                    parser_emit(p, "PARAM", arg, "-", "-");
                    param_count++;
                    
                    if (p->lexer->cur_token.type == TK_COMMA) {
                        parser_consume(p);
                    } else {
                        break;
                    }
                }
            }
            
            parser_expect(p, TK_RPAREN, "預期 ')'");
            parser_new_t(p, result);
            char count_str[16];
            sprintf(count_str, "%d", param_count);
            parser_emit(p, "CALL", name, count_str, result);
            return result;
        } else if (p->lexer->cur_token.type == TK_LBRACKET) {
            parser_consume(p);
            char idx[MAX_TOKEN_LEN];
            parser_new_t(p, idx);
            parser_expression(p, idx);
            parser_expect(p, TK_RBRACKET, "預期 ']'");
            parser_new_t(p, result);
            parser_emit(p, "GET_ITEM", name, idx, result);
            return result;
        } else {
            // 單純的變數載入
            parser_new_t(p, result);
            parser_emit(p, "STORE", name, "-", result);
            return result;
        }
    } else if (p->lexer->cur_token.type == TK_LPAREN) {
        parser_consume(p);
        parser_expression(p, result);
        parser_expect(p, TK_RPAREN, "預期 ')'");
        return result;
    } else {
        parser_error(p, "預期運算式");
        return result;
    }
}

char* parser_term(Parser *p, char *result) {
    char left[MAX_TOKEN_LEN];
    parser_new_t(p, left);
    parser_primary(p, left);
    strcpy(result, left);
    
    while (p->lexer->cur_token.type == TK_MUL || p->lexer->cur_token.type == TK_DIV) {
        TokenType op = p->lexer->cur_token.type;
        parser_consume(p);
        
        char right[MAX_TOKEN_LEN];
        parser_new_t(p, right);
        parser_primary(p, right);
        
        char temp[MAX_TOKEN_LEN];
        parser_new_t(p, temp);
        
        if (op == TK_MUL) {
            parser_emit(p, "MUL", result, right, temp);
        } else {
            parser_emit(p, "DIV", result, right, temp);
        }
        strcpy(result, temp);
    }
    
    return result;
}

char* parser_arith(Parser *p, char *result) {
    char left[MAX_TOKEN_LEN];
    parser_new_t(p, left);
    parser_term(p, left);
    strcpy(result, left);
    
    while (p->lexer->cur_token.type == TK_PLUS || p->lexer->cur_token.type == TK_MINUS) {
        TokenType op = p->lexer->cur_token.type;
        parser_consume(p);
        
        char right[MAX_TOKEN_LEN];
        parser_new_t(p, right);
        parser_term(p, right);
        
        char temp[MAX_TOKEN_LEN];
        parser_new_t(p, temp);
        
        if (op == TK_PLUS) {
            parser_emit(p, "ADD", result, right, temp);
        } else {
            parser_emit(p, "SUB", result, right, temp);
        }
        strcpy(result, temp);
    }
    
    return result;
}

char* parser_expression(Parser *p, char *result) {
    char left[MAX_TOKEN_LEN];
    parser_new_t(p, left);
    parser_arith(p, left);
    strcpy(result, left);
    
    if (p->lexer->cur_token.type == TK_EQ || 
        p->lexer->cur_token.type == TK_LT || 
        p->lexer->cur_token.type == TK_GT) {
        TokenType op = p->lexer->cur_token.type;
        parser_consume(p);
        
        char right[MAX_TOKEN_LEN];
        parser_new_t(p, right);
        parser_arith(p, right);
        
        char temp[MAX_TOKEN_LEN];
        parser_new_t(p, temp);
        
        if (op == TK_EQ) {
            parser_emit(p, "CMP_EQ", result, right, temp);
        } else if (op == TK_LT) {
            parser_emit(p, "CMP_LT", result, right, temp);
        } else {
            parser_emit(p, "CMP_GT", result, right, temp);
        }
        strcpy(result, temp);
    }
    
    return result;
}

void parser_expr_or_assign(Parser *p) {
    char name[MAX_TOKEN_LEN];
    strcpy(name, p->lexer->cur_token.text);
    parser_consume(p);
    
    if (p->lexer->cur_token.type == TK_ASSIGN) {
        parser_consume(p);
        char val[MAX_TOKEN_LEN];
        parser_new_t(p, val);
        parser_expression(p, val);
        parser_emit(p, "STORE", val, "-", name);
    }
}

void parser_statement(Parser *p) {
    if (p->lexer->cur_token.type == TK_IF) {
        parser_consume(p);
        parser_expect(p, TK_LPAREN, "預期 '('");
        
        char cond[MAX_TOKEN_LEN];
        parser_new_t(p, cond);
        parser_expression(p, cond);
        parser_expect(p, TK_RPAREN, "預期 ')'");
        
        int jmp_f_idx = parser_emit(p, "JMP_F", cond, "-", "?");
        
        parser_expect(p, TK_LBRACE, "預期 '{'");
        while (p->lexer->cur_token.type != TK_RBRACE && p->lexer->cur_token.type != TK_EOF) {
            parser_statement(p);
        }
        parser_expect(p, TK_RBRACE, "預期 '}'");
        
        char end_str[16];
        sprintf(end_str, "%d", p->quad_count);
        strcpy(p->quads[jmp_f_idx].result, end_str);
    } else if (p->lexer->cur_token.type == TK_ID) {
        parser_expr_or_assign(p);
        parser_expect(p, TK_SEMICOLON, "預期 ';'");
    } else if (p->lexer->cur_token.type == TK_RETURN) {
        parser_consume(p);
        char res[MAX_TOKEN_LEN];
        parser_new_t(p, res);
        parser_expression(p, res);
        parser_emit(p, "RET_VAL", res, "-", "-");
        parser_expect(p, TK_SEMICOLON, "預期 ';'");
    } else if (p->lexer->cur_token.type == TK_PRINT) {
        parser_consume(p);
        parser_expect(p, TK_LPAREN, "預期 '('");
        
        if (p->lexer->cur_token.type != TK_RPAREN) {
            while (true) {
                char val[MAX_TOKEN_LEN];
                parser_new_t(p, val);
                parser_expression(p, val);
                parser_emit(p, "PRINT_VAL", val, "-", "-");
                
                if (p->lexer->cur_token.type == TK_COMMA) {
                    parser_consume(p);
                } else {
                    break;
                }
            }
        }
        
        parser_emit(p, "PRINT_NL", "-", "-", "-");
        parser_expect(p, TK_RPAREN, "預期 ')'");
        parser_expect(p, TK_SEMICOLON, "預期 ';'");
    } else {
        parser_error(p, "無法辨識的陳述句");
    }
}

void parser_parse_program(Parser *p) {
    while (p->lexer->cur_token.type != TK_EOF) {
        if (p->lexer->cur_token.type == TK_FUNC) {
            parser_consume(p);
            char f_name[MAX_TOKEN_LEN];
            strcpy(f_name, p->lexer->cur_token.text);
            parser_consume(p);
            
            parser_emit(p, "FUNC_BEG", f_name, "-", "-");
            parser_expect(p, TK_LPAREN, "預期 '('");
            
            if (p->lexer->cur_token.type != TK_RPAREN) {
                while (true) {
                    parser_emit(p, "FORMAL", p->lexer->cur_token.text, "-", "-");
                    parser_consume(p);
                    
                    if (p->lexer->cur_token.type == TK_COMMA) {
                        parser_consume(p);
                    } else {
                        break;
                    }
                }
            }
            
            parser_expect(p, TK_RPAREN, "預期 ')'");
            parser_expect(p, TK_LBRACE, "預期 '{'");
            
            while (p->lexer->cur_token.type != TK_RBRACE && p->lexer->cur_token.type != TK_EOF) {
                parser_statement(p);
            }
            
            parser_emit(p, "FUNC_END", f_name, "-", "-");
            parser_expect(p, TK_RBRACE, "預期 '}'");
        } else {
            parser_statement(p);
        }
    }
}

Parser* parser_create(Lexer *lex) {
    Parser *p = (Parser *)malloc(sizeof(Parser));
    if (!p) {
        printf("記憶體配置失敗\n");
        exit(1);
    }
    
    p->lexer = lex;
    p->quad_capacity = 100;
    p->quads = (Quad *)malloc(p->quad_capacity * sizeof(Quad));
    p->string_pool = (char (*)[MAX_TOKEN_LEN])malloc(MAX_STRING_POOL * sizeof(char[MAX_TOKEN_LEN]));
    
    if (!p->quads || !p->string_pool) {
        printf("記憶體配置失敗\n");
        exit(1);
    }
    
    p->quad_count = 0;
    p->string_count = 0;
    p->loop_depth = 0;
    p->t_idx = 0;
    
    return p;
}

void parser_destroy(Parser *p) {
    if (p) {
        free(p->quads);
        free(p->string_pool);
        free(p);
    }
}

typedef struct {
    char name[MAX_TOKEN_LEN];
    int value;
} Variable;

typedef struct {
    Variable vars[MAX_VARS];
    int var_count;
    int ret_pc;
    char ret_var[MAX_TOKEN_LEN];
    int incoming_args[MAX_PARAMS];
    int arg_count;
    int formal_idx;
} Frame;

typedef struct {
    Quad *quads;
    int quad_count;
    char (*string_pool)[MAX_TOKEN_LEN];
    int string_count;
    Frame stack[MAX_STACK_DEPTH];
    int sp;
    char print_buf[10000];
} VM;

int vm_get_var_value(VM *vm, const char *name) {
    if (isdigit(name[0]) || (name[0] == '-' && isdigit(name[1]))) {
        return atoi(name);
    }
    if (strcmp(name, "-") == 0) return 0;
    
    Frame *frame = &vm->stack[vm->sp];
    for (int i = 0; i < frame->var_count; i++) {
        if (strcmp(frame->vars[i].name, name) == 0) {
            return frame->vars[i].value;
        }
    }
    
    return 0;
}

void vm_set_var(VM *vm, const char *name, int val) {
    Frame *frame = &vm->stack[vm->sp];
    
    for (int i = 0; i < frame->var_count; i++) {
        if (strcmp(frame->vars[i].name, name) == 0) {
            frame->vars[i].value = val;
            return;
        }
    }
    
    if (frame->var_count < MAX_VARS) {
        strcpy(frame->vars[frame->var_count].name, name);
        frame->vars[frame->var_count].value = val;
        frame->var_count++;
    }
}

void vm_run(VM *vm) {
    int pc = 0;
    int param_stack[MAX_PARAMS];
    int param_sp = 0;
    
    typedef struct {
        char name[MAX_TOKEN_LEN];
        int pc;
    } FuncEntry;
    FuncEntry func_map[100];
    int func_count = 0;
    
    for (int i = 0; i < vm->quad_count; i++) {
        if (strcmp(vm->quads[i].op, "FUNC_BEG") == 0) {
            strcpy(func_map[func_count].name, vm->quads[i].arg1);
            func_map[func_count].pc = i + 1;
            func_count++;
        }
    }
    
    printf("\n=== VM 執行開始 ===\n");
    
    while (pc < vm->quad_count) {
        Quad *q = &vm->quads[pc];
        
        if (strcmp(q->op, "FUNC_BEG") == 0) {
            while (pc < vm->quad_count && strcmp(vm->quads[pc].op, "FUNC_END") != 0) {
                pc++;
            }
        } else if (strcmp(q->op, "IMM") == 0) {
            vm_set_var(vm, q->result, atoi(q->arg1));
        } else if (strcmp(q->op, "ADD") == 0) {
            int v1 = vm_get_var_value(vm, q->arg1);
            int v2 = vm_get_var_value(vm, q->arg2);
            vm_set_var(vm, q->result, v1 + v2);
        } else if (strcmp(q->op, "SUB") == 0) {
            int v1 = vm_get_var_value(vm, q->arg1);
            int v2 = vm_get_var_value(vm, q->arg2);
            vm_set_var(vm, q->result, v1 - v2);
        } else if (strcmp(q->op, "MUL") == 0) {
            int v1 = vm_get_var_value(vm, q->arg1);
            int v2 = vm_get_var_value(vm, q->arg2);
            vm_set_var(vm, q->result, v1 * v2);
        } else if (strcmp(q->op, "DIV") == 0) {
            int v1 = vm_get_var_value(vm, q->arg1);
            int v2 = vm_get_var_value(vm, q->arg2);
            vm_set_var(vm, q->result, v2 != 0 ? v1 / v2 : 0);
        } else if (strcmp(q->op, "CMP_EQ") == 0) {
            int v1 = vm_get_var_value(vm, q->arg1);
            int v2 = vm_get_var_value(vm, q->arg2);
            vm_set_var(vm, q->result, v1 == v2 ? 1 : 0);
        } else if (strcmp(q->op, "STORE") == 0) {
            int val = vm_get_var_value(vm, q->arg1);
            vm_set_var(vm, q->result, val);
        } else if (strcmp(q->op, "JMP") == 0) {
            pc = atoi(q->result) - 1;
        } else if (strcmp(q->op, "JMP_F") == 0) {
            int cond = vm_get_var_value(vm, q->arg1);
            if (cond == 0) {
                pc = atoi(q->result) - 1;
            }
        } else if (strcmp(q->op, "PRINT_VAL") == 0) {
            int val = vm_get_var_value(vm, q->arg1);
            char temp[100];
            sprintf(temp, "%d ", val);
            strcat(vm->print_buf, temp);
        } else if (strcmp(q->op, "PRINT_NL") == 0) {
            printf("[程式輸出] >> %s\n", vm->print_buf);
            vm->print_buf[0] = '\0';
        } else if (strcmp(q->op, "PARAM") == 0) {
            param_stack[param_sp++] = vm_get_var_value(vm, q->arg1);
        } else if (strcmp(q->op, "CALL") == 0) {
            int p_count = atoi(q->arg2);
            char *f_name = q->arg1;
            
            int target_pc = -1;
            for (int i = 0; i < func_count; i++) {
                if (strcmp(func_map[i].name, f_name) == 0) {
                    target_pc = func_map[i].pc;
                    break;
                }
            }
            
            if (target_pc == -1) {
                printf("\n[VM 執行時期錯誤] 找不到函數 '%s'\n", f_name);
                exit(1);
            }
            
            vm->sp++;
            if (vm->sp >= MAX_STACK_DEPTH) {
                printf("\n[VM 執行時期錯誤] 堆疊溢出\n");
                exit(1);
            }
            
            vm->stack[vm->sp].var_count = 0;
            vm->stack[vm->sp].ret_pc = pc + 1;
            strcpy(vm->stack[vm->sp].ret_var, q->result);
            vm->stack[vm->sp].arg_count = p_count;
            vm->stack[vm->sp].formal_idx = 0;
            
            for (int i = 0; i < p_count; i++) {
                vm->stack[vm->sp].incoming_args[i] = param_stack[param_sp - p_count + i];
            }
            param_sp -= p_count;
            
            pc = target_pc;
            continue;
        } else if (strcmp(q->op, "FORMAL") == 0) {
            Frame *frame = &vm->stack[vm->sp];
            vm_set_var(vm, q->arg1, frame->incoming_args[frame->formal_idx++]);
        } else if (strcmp(q->op, "RET_VAL") == 0) {
            int ret_val = vm_get_var_value(vm, q->arg1);
            int ret_pc = vm->stack[vm->sp].ret_pc;
            char ret_var[MAX_TOKEN_LEN];
            strcpy(ret_var, vm->stack[vm->sp].ret_var);
            
            vm->sp--;
            vm_set_var(vm, ret_var, ret_val);
            pc = ret_pc;
            continue;
        } else if (strcmp(q->op, "FUNC_END") == 0) {
            if (vm->sp > 0) {
                int ret_pc = vm->stack[vm->sp].ret_pc;
                char ret_var[MAX_TOKEN_LEN];
                strcpy(ret_var, vm->stack[vm->sp].ret_var);
                
                vm->sp--;
                vm_set_var(vm, ret_var, 0);
                pc = ret_pc;
                continue;
            }
        }
        
        pc++;
    }
    
    printf("=== VM 執行完畢 ===\n\n記憶體狀態 (全域變數):\n");
    Frame *global = &vm->stack[0];
    for (int i = 0; i < global->var_count; i++) {
        if (global->vars[i].name[0] != 't') {
            printf("[%s] = %d\n", global->vars[i].name, global->vars[i].value);
        }
    }
}

VM* vm_create(Quad *quads, int quad_count, char (*string_pool)[MAX_TOKEN_LEN], int string_count) {
    VM *vm = (VM *)malloc(sizeof(VM));
    if (!vm) {
        printf("記憶體配置失敗\n");
        exit(1);
    }
    
    vm->quads = quads;
    vm->quad_count = quad_count;
    vm->string_pool = string_pool;
    vm->string_count = string_count;
    vm->sp = 0;
    vm->stack[0].var_count = 0;
    vm->print_buf[0] = '\0';
    
    return vm;
}

void vm_destroy(VM *vm) {
    if (vm) {
        free(vm);
    }
}

int main(int argc, char *argv[]) {
    if (argc < 2) {
        printf("用法: %s <source_file>\n", argv[0]);
        return 1;
    }
    
    FILE *f = fopen(argv[1], "r");
    if (!f) {
        printf("無法開啟檔案: %s\n", argv[1]);
        return 1;
    }
    
    fseek(f, 0, SEEK_END);
    long size = ftell(f);
    fseek(f, 0, SEEK_SET);
    
    char *source = (char *)malloc(size + 1);
    if (!source) {
        printf("記憶體配置失敗\n");
        fclose(f);
        return 1;
    }
    
    fread(source, 1, size, f);
    source[size] = '\0';
    fclose(f);
    
    printf("編譯器生成的中間碼 (PC: Quadruples):\n");
    printf("--------------------------------------------\n");
    
    Lexer lexer;
    lexer_init(&lexer, source);
    
    Parser *parser = parser_create(&lexer);
    parser_parse_program(parser);
    
    VM *vm = vm_create(parser->quads, parser->quad_count, parser->string_pool, parser->string_count);
    vm_run(vm);
    
    vm_destroy(vm);
    parser_destroy(parser);
    free(source);
    
    return 0;
}