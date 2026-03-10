#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// 提供給 Stronge 的字串相加 (+)
char* concat(const char* s1, const char* s2) {
    size_t len1 = strlen(s1);
    size_t len2 = strlen(s2);
    char* result = (char*)malloc(len1 + len2 + 1);
    strcpy(result, s1);
    strcat(result, s2);
    return result;
}

// 提供給 Stronge 的 print() 函數
long long print(const char* s) {
    printf("%s\n", s);
    return 0; // return int (i64)
}

// 提供給 Stronge 的 int 轉 str (動態配置記憶體)
char* to_str(long long val) {
    // 64 位元整數最多約 20 個字元，分配 32 byte 絕對夠用
    char* str = (char*)malloc(32);
    snprintf(str, 32, "%lld", val);
    return str;
}
