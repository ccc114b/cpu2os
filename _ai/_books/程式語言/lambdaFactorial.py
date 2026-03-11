# Y Combinator 的定義
Y = lambda f: (lambda x: f(lambda v: x(x)(v)))(lambda x: f(lambda v: x(x)(v)))

# 階層邏輯
factorial_gen = lambda f: lambda n: 1 if n == 0 else n * f(n - 1)

# 執行
factorial = Y(factorial_gen)
print(factorial(5))