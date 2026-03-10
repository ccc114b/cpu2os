根據您提供的 `compiler.py` 原始碼，該程式語言的 **EBNF (Extended Backus-Naur Form)** 語法定義如下：

---

## 程式語言 EBNF 語法定義

### 1. 頂層結構 (Top-level)

* **program** ::= ( **function_def** | **statement** )*
* **function_def** ::= "func" **id** "(" [ **id** ("," **id**)* ] ")" "{" **statement*** "}"

### 2. 陳述句 (Statements)

* **statement** ::=
* "if" "(" **expression** ")" "{" **statement*** "}" [ "else" "{" **statement*** "}" ]
* | "while" "(" **expression** ")" "{" **statement*** "}"
* | "for" "(" [ **expr_or_assign** ] ";" [ **expression** ] ";" [ **expr_or_assign** ] ")" "{" **statement*** "}"
* | "break" ";"
* | "continue" ";"
* | "return" **expression** ";"
* | "print" "(" [ **expression** ("," **expression**)* ] ")" ";"
* | **expr_or_assign** ";"



### 3. 表達式與賦值 (Expressions & Assignment)

* **expr_or_assign** ::= **id** { "[" **expression** "]" | "." **id** | "(" [ **expression** ("," **expression**)* ] ")" } [ "=" **expression** ]
* **expression** ::= **arith_expr** [ ("==" | "<" | ">") **arith_expr** ]
* **arith_expr** ::= **term** ( ("+" | "-") **term** )*
* **term** ::= **factor** ( ("*" | "/") **factor** )*
* **factor** ::= **primary** { "[" **expression** "]" | "." **id** | "(" [ **expression** ("," **expression**)* ] ")" }

### 4. 基礎元與字面量 (Primary & Literals)

* **primary** ::=
* **num**
* | **string**
* | **id**
* | "[" [ **expression** ("," **expression**)* ] "]"
* | "{" [ (**id** | **string**) ":" **expression** ("," (**id** | **string**) ":" **expression**)* ] "}"
* | "(" **expression** ")"



---

**您可以嘗試執行什麼？**
如果您手邊有符合此語法的程式碼檔案（例如 `test.src`），您可以使用以下指令來執行編譯與模擬：
`python compiler.py test.src`