// p0 範例：判斷質數 (Prime Number)
// 邏輯：如果一個數 n 能被 2 到 n-1 之間的任何數整除，就不是質數。

// 手寫取餘數函數 (n % d)
func mod(n, d) {
    if (n < d) {
        return n;
    }
    return mod(n - d, d);
}

// 遞迴檢查是否能被整除
// n: 要檢查的數, i: 目前嘗試的除數
func is_prime_recursive(n, i) {
    // 1. 如果除數平方大於 n，代表沒找到因數，它是質數
    if (i * i > n) {
        return 1;
    }
    
    // 2. 如果 n 能被 i 整除，它不是質數
    m = mod(n, i);
    if (m == 0) {
        return 0;
    }
    
    // 3. 嘗試下一個除數
    return is_prime_recursive(n, i + 1);
}

// 主進入點
func check_prime(n) {
    if (n < 2) {
        return 0;
    }
    if (n == 2) {
        return 1;
    }
    return is_prime_recursive(n, 2);
}

// 測試：13 是質數 (result1 應為 1)
result1 = check_prime(13);

// 測試：15 不是質數 (result2 應為 0)
result2 = check_prime(15);