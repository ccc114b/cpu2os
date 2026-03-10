#include <stdio.h>
#include <ctype.h>
#include <string.h>
#include <stdlib.h>

// --- 詞法分析 (Lexer) 部分 ---
typedef enum {
    TK_FUNC, TK_RETURN, TK_ID, TK_NUM, TK_LPAREN, TK_RPAREN, 
    TK_LBRACE, TK_RBRACE, TK_COMMA, TK_ASSIGN, TK_SEMICOLON,
    TK_PLUS, TK_MINUS, TK_MUL, TK_DIV, TK_EOF
} TokenType;

typedef struct {
    TokenType type;
    char text[32];
} Token;

Token cur_token;
const char *src;

void next_token() {
    while (isspace(*src)) src++;
    if (*src == '\0') { cur_token.type = TK_EOF; return; }

    if (isdigit(*src)) {
        int i = 0;
        while (isdigit(*src)) cur_token.text[i++] = *src++;
        cur_token.text[i] = '\0';
        cur_token.type = TK_NUM;
    } else if (isalpha(*src) || *src == '_') {
        int i = 0;
        while (isalnum(*src) || *src == '_') cur_token.text[i++] = *src++;
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

// --- 中間碼生成 部分 ---
int t_idx = 0;
void new_t(char *s) { sprintf(s, "t%d", ++t_idx); }
void emit(const char* op, const char* a1, const char* a2, const char* res) {
    printf("(%-10s, %-10s, %-10s, %-10s)\n", op, a1, a2, res);
}

// --- 遞迴下降解析 (Parser) ---
void expression(char *res);

void factor(char *res) {
    if (cur_token.type == TK_NUM) {
        new_t(res); emit("IMM", cur_token.text, "-", res);
        next_token();
    } else if (cur_token.type == TK_ID) {
        char name[32]; strcpy(name, cur_token.text);
        next_token();
        if (cur_token.type == TK_LPAREN) { // 函數呼叫
            next_token();
            int count = 0;
            while (cur_token.type != TK_RPAREN) {
                char arg[32]; expression(arg);
                emit("PARAM", arg, "-", "-");
                count++;
                if (cur_token.type == TK_COMMA) next_token();
            }
            next_token(); // skip )
            new_t(res);
            char c_str[10]; sprintf(c_str, "%d", count);
            emit("CALL", name, c_str, res);
        } else {
            strcpy(res, name); // 單純變數
        }
    } else if (cur_token.type == TK_LPAREN) {
        next_token(); expression(res); next_token();
    }
}

void term(char *res) {
    char l[32], r[32], t[32];
    factor(l);
    while (cur_token.type == TK_MUL || cur_token.type == TK_DIV) {
        char op[10]; strcpy(op, cur_token.type == TK_MUL ? "MUL" : "DIV");
        next_token(); factor(r);
        new_t(t); emit(op, l, r, t);
        strcpy(l, t);
    }
    strcpy(res, l);
}

void expression(char *res) {
    char l[32], r[32], t[32];
    term(l);
    while (cur_token.type == TK_PLUS || cur_token.type == TK_MINUS) {
        char op[10]; strcpy(op, cur_token.type == TK_PLUS ? "ADD" : "SUB");
        next_token(); term(r);
        new_t(t); emit(op, l, r, t);
        strcpy(l, t);
    }
    strcpy(res, l);
}

void statement() {
    if (cur_token.type == TK_ID) {
        char name[32]; strcpy(name, cur_token.text);
        next_token();
        if (cur_token.type == TK_ASSIGN) {
            next_token();
            char res[32]; expression(res);
            emit("STORE", res, "-", name);
            next_token(); // skip ;
        }
    } else if (cur_token.type == TK_RETURN) {
        next_token();
        char res[32]; expression(res);
        emit("RET_VAL", res, "-", "-");
        next_token();
    }
}

void parse_program() {
    while (cur_token.type != TK_EOF) {
        if (cur_token.type == TK_FUNC) {
            next_token(); // skip func
            char f_name[32]; strcpy(f_name, cur_token.text);
            emit("FUNC_BEG", f_name, "-", "-");
            next_token(); next_token(); // skip name and (
            while (cur_token.type == TK_ID) {
                emit("FORMAL", cur_token.text, "-", "-");
                next_token();
                if (cur_token.type == TK_COMMA) next_token();
            }
            next_token(); next_token(); // skip ) and {
            while (cur_token.type != TK_RBRACE) statement();
            emit("FUNC_END", f_name, "-", "-");
            next_token();
        } else {
            statement();
        }
    }
}

int main() {
    char code[] = "func add(a, b) { sum = a + b; return sum; } x = add(10, 20);";
    src = code;
    printf("Compiling: %s\n\n", code);
    printf("%-10s %-10s %-10s %-10s\n", "Op", "Arg1", "Arg2", "Result");
    printf("--------------------------------------------\n");
    next_token();
    parse_program();
    return 0;
}