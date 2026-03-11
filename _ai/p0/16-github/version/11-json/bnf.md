(* ========================================================= *)
(* 1. 程式與全域結構 (Program Structure)                     *)
(* ========================================================= *)
program = { function_def | statement } ;

function_def = "func" identifier "(" [ parameter_list ] ")" "{" { statement } "}" ;
parameter_list = identifier { "," identifier } ;

(* ========================================================= *)
(* 2. 陳述句 (Statements)                                    *)
(* ========================================================= *)
statement = if_statement
          | while_statement        (* 【新增】while 迴圈 *)
          | for_statement          (* 【新增】for 迴圈 *)
          | assignment_statement
          | return_statement
          | print_statement
          | break_statement        (* 【新增】跳出迴圈 *)
          | continue_statement ;   (* 【新增】繼續下一次迴圈 *)

if_statement = "if" "(" expression ")" "{" { statement } "}" ;

(* 【新增】while 迴圈：與 if 結構非常類似 *)
while_statement = "while" "(" expression ")" "{" { statement } "}" ;

(* 【新增】for 迴圈：支援類似 C 語言的結構 for(i=0; i<10; i=i+1) *)
(* 括號內分為三個區塊：初始化 (可選) ; 條件判斷 (可選) ; 步進操作 (可選) *)
for_statement = "for" "(" [ assignment_expr ] ";" [ expression ] ";" [ assignment_expr ] ")" "{" { statement } "}" ;

(* 【修改】為了讓 for 迴圈可以重複使用賦值邏輯，將賦值拆成表達式與陳述句 *)
assignment_expr = lvalue "=" expression ;
assignment_statement = assignment_expr ";" ;

return_statement = "return" expression ";" ;

print_statement = "print" "(" [ expression { "," expression } ] ")" ";" ;

(* 【新增】控制跳躍語句 *)
break_statement = "break" ";" ;
continue_statement = "continue" ";" ;


(* ========================================================= *)
(* 3. 運算式 (Expressions)                                   *)
(* ========================================================= *)
expression = arith_expr [ ( "==" | "<" | ">" ) arith_expr ] ;

arith_expr = term { ( "+" | "-" ) term } ;

term = factor { ( "*" | "/" ) factor } ;

factor = primary { postfix_op } ;

primary = number
        | string_literal
        | identifier
        | array_literal
        | dict_literal
        | "(" expression ")" ;

(* ========================================================= *)
(* 4. 複雜資料結構 (Complex Data Structures)                 *)
(* ========================================================= *)
array_literal = "[" [ expression { "," expression } ] "]" ;

dict_literal = "{" [ key_value_pair { "," key_value_pair } ] "}" ;
key_value_pair = ( string_literal | identifier ) ":" expression ;


(* ========================================================= *)
(* 5. 存取與操作 (Access & Mutations)                        *)
(* ========================================================= *)
postfix_op = "[" expression "]"                  
           | "." identifier                      
           | "(" [ argument_list ] ")" ;         

argument_list = expression { "," expression } ;

lvalue = identifier { "[" expression "]" | "." identifier } ;

(* ========================================================= *)
(* 6. 詞彙基本單位 (Tokens)                                  *)
(* ========================================================= *)
identifier = letter { letter | digit | "_" } ;
number = digit { digit } ;
string_literal = '"' { all_characters_except_quotes } '"' ;