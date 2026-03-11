# 03. BNF 語法

以下 BNF 依目前 `p0c.rs` 的實作整理（含 `for` 與 `while`）。

```bnf
<program>        ::= { <func_decl> | <statement> } <eof>

<func_decl>      ::= "func" <id> "(" [ <id_list> ] ")" "{" { <statement> } "}"
<id_list>        ::= <id> { "," <id> }

<statement>      ::= <if_stmt>
                   | <while_stmt>
                   | <for_stmt>
                   | <break_stmt>
                   | <continue_stmt>
                   | <return_stmt>
                   | <expr_or_assign> ";"

<if_stmt>        ::= "if" "(" <expr> ")" "{" { <statement> } "}"
                     [ "else" "{" { <statement> } "}" ]

<while_stmt>     ::= "while" "(" <expr> ")" "{" { <statement> } "}"

<for_stmt>       ::= "for" "(" [ <expr_or_assign> ] ";"
                               [ <expr> ] ";"
                               [ <expr_or_assign> ] ")"
                     "{" { <statement> } "}"

<break_stmt>     ::= "break" ";"
<continue_stmt>  ::= "continue" ";"
<return_stmt>    ::= "return" <expr> ";"

<expr_or_assign> ::= <postfix_expr>
                     [ "=" <expr>
                     | "[" <expr> "]" "=" <expr>
                     | "." <id> "=" <expr> ]

<expr>           ::= <arith_expr>
                     [ ("==" | "<" | ">") <arith_expr> ]

<arith_expr>     ::= <term> { ("+" | "-") <term> }
<term>           ::= <factor> { ("*" | "/") <factor> }

<factor>         ::= <num>
                   | <string>
                   | "(" <expr> ")"
                   | <postfix_expr>
                   | <array_lit>
                   | <dict_lit>

<postfix_expr>   ::= <id>
                     { "(" [ <arg_list> ] ")"
                     | "[" <expr> "]"
                     | "." <id> }

<arg_list>       ::= <expr> { "," <expr> }

<array_lit>      ::= "[" [ <expr> { "," <expr> } ] "]"
<dict_lit>       ::= "{" [ <dict_item> { "," <dict_item> } ] "}"
<dict_item>      ::= (<string> | <id>) ":" <expr>
```

## 備註

- `for` 的 `init / cond / update` 皆可省略。
- `break` / `continue` 僅允許在迴圈內。
- `return` 目前要求帶回傳值。
