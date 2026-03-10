#include <stdio.h>
#include <ctype.h>

const char *p;
int temp_count = 0;

int new_temp() { return ++temp_count; }

// 預宣告
int expression();
int term();
int factor();

// 處理數字或變數 (Identifier)
int get_atom() {
    if (isdigit(*p)) {
        int val = 0;
        while (isdigit(*p)) val = val * 10 + (*p++ - '0');
        int t = new_temp();
        printf("t%d = %d\n", t, val);
        return t;
    } else if (isalpha(*p)) {
        // 假設這是一個變數，直接回傳一個代表該變數的代號（或直接印出）
        char var_name = *p++;
        int t = new_temp();
        printf("t%d = load %c\n", t, var_name); // 模擬從記憶體載入變數
        return t;
    }
    return 0;
}

int factor() {
    if (*p == '(') {
        p++; int t = expression(); p++; return t;
    }
    return get_atom();
}

int term() {
    int left = factor();
    while (*p == '*' || *p == '/') {
        char op = *p++;
        int right = factor();
        int target = new_temp();
        printf("t%d = t%d %c t%d\n", target, left, op, right);
        left = target;
    }
    return left;
}

int expression() {
    int left = term();
    while (*p == '+' || *p == '-') {
        char op = *p++;
        int right = term();
        int target = new_temp();
        printf("t%d = t%d %c t%d\n", target, left, op, right);
        left = target;
    }
    return left;
}

// 新增：處理賦值陳述句 assignment = id "=" expression
void assignment() {
    if (isalpha(*p) && *(p+1) == '=') {
        char var_name = *p;
        p += 2; // 跳過 'x='
        int result_t = expression();
        printf("store t%d into %c\n", result_t, var_name);
    } else {
        expression(); // 如果不是賦值，就當作普通運算
    }
}

int main() {
    char input[100];
    printf("請輸入賦值語句 (例如 x=3+5*(2-y)): ");
    scanf("%s", input);
    p = input;

    printf("\n--- 產生的中間碼 (Intermediate Code) ---\n");
    assignment();
    
    return 0;
}