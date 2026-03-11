```
(py310) cccimac@cccimacdeiMac 08-comment % ./compiler p0/prime_comment.p0
編譯器生成的中間碼 (PC: Quadruples):
--------------------------------------------
000: FUNC_BEG   mod        -          -         
001: FORMAL     n          -          -         
002: FORMAL     d          -          -         
003: CMP_LT     n          d          t1        
004: JMP_F      t1         -          ?         
005: RET_VAL    n          -          -         
006: SUB        n          d          t2        
007: PARAM      t2         -          -         
008: PARAM      d          -          -         
009: CALL       mod        2          t3        
010: RET_VAL    t3         -          -         
011: FUNC_END   mod        -          -         
012: FUNC_BEG   is_prime_recursive -          -         
013: FORMAL     n          -          -         
014: FORMAL     i          -          -         
015: MUL        i          i          t4        
016: CMP_GT     t4         n          t5        
017: JMP_F      t5         -          ?         
018: IMM        1          -          t6        
019: RET_VAL    t6         -          -         
020: PARAM      n          -          -         
021: PARAM      i          -          -         
022: CALL       mod        2          t7        
023: STORE      t7         -          m         
024: IMM        0          -          t8        
025: CMP_EQ     m          t8         t9        
026: JMP_F      t9         -          ?         
027: IMM        0          -          t10       
028: RET_VAL    t10        -          -         
029: PARAM      n          -          -         
030: IMM        1          -          t11       
031: ADD        i          t11        t12       
032: PARAM      t12        -          -         
033: CALL       is_prime_recursive 2          t13       
034: RET_VAL    t13        -          -         
035: FUNC_END   is_prime_recursive -          -         
036: FUNC_BEG   check_prime -          -         
037: FORMAL     n          -          -         
038: IMM        2          -          t14       
039: CMP_LT     n          t14        t15       
040: JMP_F      t15        -          ?         
041: IMM        0          -          t16       
042: RET_VAL    t16        -          -         
043: IMM        2          -          t17       
044: CMP_EQ     n          t17        t18       
045: JMP_F      t18        -          ?         
046: IMM        1          -          t19       
047: RET_VAL    t19        -          -         
048: PARAM      n          -          -         
049: IMM        2          -          t20       
050: PARAM      t20        -          -         
051: CALL       is_prime_recursive 2          t21       
052: RET_VAL    t21        -          -         
053: FUNC_END   check_prime -          -         
054: IMM        13         -          t22       
055: PARAM      t22        -          -         
056: CALL       check_prime 1          t23       
057: STORE      t23        -          result1   
058: IMM        15         -          t24       
059: PARAM      t24        -          -         
060: CALL       check_prime 1          t25       
061: STORE      t25        -          result2   

=== VM 執行開始 ===
=== VM 執行完畢 ===

全域變數結果:
>> result1 = 1
>> result2 = 0
```
