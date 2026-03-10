#include <stdio.h>
#include <ctype.h>
#include <stdlib.h>

// 全域變數追蹤目前讀取到的字元位置
const char *p;

// 預覽目前的字元
char peek() { return *p; }

// 讀取並跳至下一個字元
char get() { return *p++; }

// 函數預宣告（對應 EBNF 規則）
double expression();
double term();
double factor();
double number();

// 1. expression = term , { ( "+" | "-" ) , term } ;
double expression() {
    double result = term();
    while (peek() == '+' || peek() == '-') {
        if (get() == '+') result += term();
        else result -= term();
    }
    return result;
}

// 2. term = factor , { ( "*" | "/" ) , factor } ;
double term() {
    double result = factor();
    while (peek() == '*' || peek() == '/') {
        if (get() == '*') result *= factor();
        else result /= factor();
    }
    return result;
}

// 3. factor = number | "(" , expression , ")" ;
double factor() {
    if (peek() == '(') {
        get(); // 跳過 '('
        double result = expression();
        get(); // 跳過 ')'
        return result;
    }
    return number();
}

// 4. number = digit , { digit } ;
double number() {
    double result = 0;
    while (isdigit(peek())) {
        result = result * 10 + (get() - '0');
    }
    return result;
}

int main() {
    char input[100];
    printf("請輸入數學運算式 (例如 3+5*(2-1)): ");
    scanf("%s", input);
    
    p = input;
    printf("計算結果: %.2f\n", expression());
    
    return 0;
}