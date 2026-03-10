#include <stdio.h>
#include <ctype.h>

const char *p;
int temp_count = 0; // 用來生成 t1, t2, t3...

// 生成一個新的暫存器編號
int new_temp() {
    return ++temp_count;
}

// 預宣告
int expression();
int term();
int factor();

// 處理數字：直接印出賦值指令並回傳暫存器
int get_number() {
    int val = 0;
    while (isdigit(*p)) {
        val = val * 10 + (*p++ - '0');
    }
    int t = new_temp();
    printf("t%d = %d\n", t, val);
    return t;
}

// factor = number | "(" expression ")"
int factor() {
    if (*p == '(') {
        p++; // skip (
        int t = expression();
        p++; // skip )
        return t;
    }
    return get_number();
}

// term = factor { (*|/) factor }
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

// expression = term { (+|-) term }
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

int main() {
    char input[100];
    printf("請輸入數學運算式 (例如 3+5*(2-1)): ");
    scanf("%s", input);
    p = input;

    printf("\n--- 產生的三位址碼 (3AC) ---\n");
    int final_t = expression();
    printf("Result is in t%d\n", final_t);
    
    return 0;
}