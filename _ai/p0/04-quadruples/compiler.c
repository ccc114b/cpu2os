#include <stdio.h>
#include <ctype.h>
#include <string.h>

const char *p;
int temp_count = 0;

// 輔助函數：生成暫存器名稱 (t1, t2, ...)
void new_temp_name(char *s) {
    sprintf(s, "t%d", ++temp_count);
}

// 統一輸出四元組格式：Op Arg1 Arg2 Result
void emit(const char* op, const char* arg1, const char* arg2, const char* result) {
    printf("%-6s %-6s %-6s %-6s\n", op, arg1, arg2, result);
}

// 預宣告
void expression(char *result);
void term(char *result);
void factor(char *result);

// 處理數字或變數
void get_atom(char *result) {
    if (isdigit(*p)) {
        char val[10] = {0};
        int i = 0;
        while (isdigit(*p)) val[i++] = *p++;
        new_temp_name(result);
        emit("IMM", val, "-", result); // IMM 代表立即值 (Immediate)
    } else if (isalpha(*p)) {
        char var[2] = {*p++, '\0'};
        new_temp_name(result);
        emit("LOAD", var, "-", result);
    }
}

void factor(char *result) {
    if (*p == '(') {
        p++; expression(result); p++;
    } else {
        get_atom(result);
    }
}

void term(char *result) {
    char left[10], right[10], target[10];
    factor(left);
    while (*p == '*' || *p == '/') {
        char op_str[2] = {*p++, '\0'};
        factor(right);
        new_temp_name(target);
        emit(op_str[0] == '*' ? "MUL" : "DIV", left, right, target);
        strcpy(left, target);
    }
    strcpy(result, left);
}

void expression(char *result) {
    char left[10], right[10], target[10];
    term(left);
    while (*p == '+' || *p == '-') {
        char op_str[2] = {*p++, '\0'};
        term(right);
        new_temp_name(target);
        emit(op_str[0] == '+' ? "ADD" : "SUB", left, right, target);
        strcpy(left, target);
    }
    strcpy(result, left);
}

void assignment() {
    if (isalpha(*p) && *(p+1) == '=') {
        char var_name[2] = {*p, '\0'};
        p += 2;
        char res[10];
        expression(res);
        emit("STORE", res, "-", var_name);
    }
}

int main() {
    char input[100];
    printf("輸入運算式 (如 x=3+y*2): ");
    scanf("%s", input);
    p = input;

    printf("\n%-8s %-8s %-8s %-8s\n", "Op", "Arg1", "Arg2", "Result");
    printf("------------------------------------\n");
    assignment();
    
    return 0;
}