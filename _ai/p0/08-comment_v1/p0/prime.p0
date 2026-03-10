func mod(n, d) {
    if (n < d) {
        return n;
    }
    return mod(n - d, d);
}

func is_prime_recursive(n, i) {
    if (i * i > n) {
        return 1;
    }
    
    m = mod(n, i);
    if (m == 0) {
        return 0;
    }
    
    return is_prime_recursive(n, i + 1);
}

func check_prime(n) {
    if (n < 2) {
        return 0;
    }
    if (n == 2) {
        return 1;
    }
    return is_prime_recursive(n, 2);
}

result1 = check_prime(13);
result2 = check_prime(15);