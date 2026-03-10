## ccc: 請寫出該程式的 完整 EBNF 語法

根據您提供的 `compiler.c` 原始碼，**p0** 語言的語法邏輯可以總結為以下完整的 **EBNF (Extended Backus-Naur Form)**。這套語法描述了標記（Tokens）如何組合成表達式、陳述句以及函數結構。

---

## p0 語言 EBNF 語法定義

### 1. 程式結構 (Program Structure)

```ebnf
program       = { function_def | statement } ;
function_def  = "func" identifier "(" [ parameter_list ] ")" "{" { statement } "}" ;
parameter_list = identifier { "," identifier } ;

```

* **program**: 程式由零個或多個函數定義或全域陳述句組成。
* **function_def**: 以 `func` 開頭，包含函數名、參數列及花括號內的陳述句區塊。

---

### 2. 陳述句 (Statements)

```ebnf
statement     = if_statement
              | assignment_statement
              | return_statement ;

if_statement  = "if" "(" expression ")" "{" { statement } "}" ;
assignment_statement = identifier "=" expression [ ";" ] ;
return_statement     = "return" expression [ ";" ] ;

```

* **if_statement**: 支援條件判斷，當表達式為真（非 0）時執行區塊。
* **assignment_statement**: 將表達式的結果存入指定的識別碼（變數）中。
* **return_statement**: 計算表達式並結束目前函數執行，回傳結果。

---

### 3. 表達式與運算 (Expressions & Arithmetic)

這部分體現了運算優先級：**比較 < 加減 < 乘除 < 基本單元**。

```ebnf
expression    = arith_expr [ ( "==" | "<" | ">" ) arith_expr ] ;
arith_expr    = term { ( "+" | "-" ) term } ;
term          = factor { ( "*" | "/" ) factor } ;
factor        = number
              | identifier [ "(" [ argument_list ] ")" ]
              | "(" expression ")" ;
argument_list = expression { "," expression } ;

```

* **expression**: 最外層處理比較運算子 `==`, `<`, `>`。
* **arith_expr** & **term**: 分別處理加減法與乘除法的優先級順序。
* **factor**: 最基本的運算單元，可以是純數字、變數、括號表達式，或是**函數呼叫**。

---

### 4. 詞法元素 (Lexical Elements)

```ebnf
identifier    = letter { letter | digit | "_" } ;
number        = digit { digit } ;
letter        = "A" | ... | "Z" | "a" | ... | "z" ;
digit         = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" ;
comment       = "//" { all_characters } newline
              | "/*" { all_characters } "*/" ;

```

* **comment**: 支援單行 `//` 與多行 `/* ... */` 註解，Lexer 會在解析標記前將其過濾。
* **identifier**: 變數或函數名稱，必須以字母或底線開頭。

---

### 語法特點說明

1. **分號的可選性**: 在您的實作中，`assignment` 與 `return` 的分號是可選的（Parser 會檢查是否存在但不會強制報錯）。
2. **遞迴下降**: 此 EBNF 反映了 `compiler.c` 中 `expression` -> `arith_expr` -> `term` -> `factor` 的遞迴呼叫路徑。
3. **函數是一等公民**: 雖然 p0 簡約，但語法上完整支援了具備參數傳遞與回傳值的函數機制。

這套 EBNF 是您 **p0** 語言的靈魂藍圖。需要我幫您根據這套語法撰寫更複雜的 p0 演算法範例嗎？