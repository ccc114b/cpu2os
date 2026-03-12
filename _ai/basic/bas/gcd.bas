10  PRINT "請輸入兩個數字 A 和 B："
20  INPUT A
30  INPUT B
40  LET R = A - INT(A / B) * B
50  IF R = 0 THEN 90
60  LET A = B
70  LET B = R
80  GOTO 40
90  PRINT "最大公因數是：", B
100 END