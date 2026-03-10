```
(py310) cccimac@cccimacdeiMac 08-comment % ./compiler p0/fact.p0         
編譯器生成的中間碼 (PC: Quadruples):
--------------------------------------------
000: FUNC_BEG   factorial  -          -         
001: FORMAL     n          -          -         
002: IMM        0          -          t1        
003: CMP_EQ     n          t1         t2        
004: JMP_F      t2         -          ?         
005: IMM        1          -          t3        
006: RET_VAL    t3         -          -         
007: IMM        1          -          t4        
008: SUB        n          t4         t5        
009: PARAM      t5         -          -         
010: CALL       factorial  1          t6        
011: MUL        n          t6         t7        
012: RET_VAL    t7         -          -         
013: FUNC_END   factorial  -          -         
014: IMM        5          -          t8        
015: PARAM      t8         -          -         
016: CALL       factorial  1          t9        
017: STORE      t9         -          result    

=== VM 執行開始 ===
=== VM 執行完畢 ===

全域變數結果:
>> result = 120
```
