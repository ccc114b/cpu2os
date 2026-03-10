(py310) cccimac@cccimacdeiMac 14-syscall % ./pytest.sh
編譯器生成的中間碼 (PC: Quadruples):
--------------------------------------------
000: FUNC_BEG     factorial  -          -         
001: FORMAL       n          -          -         
002: IMM          0          -          t1        
003: CMP_EQ       n          t1         t2        
004: JMP_F        t2         -          ?         
005: IMM          1          -          t3        
006: RET_VAL      t3         -          -         
007: IMM          1          -          t4        
008: SUB          n          t4         t5        
009: PARAM        t5         -          -         
010: CALL         factorial  1          t6        
011: MUL          n          t6         t7        
012: RET_VAL      t7         -          -         
013: FUNC_END     factorial  -          -         
014: IMM          5          -          t8        
015: PARAM        t8         -          -         
016: CALL         factorial  1          t9        
017: STORE        t9         -          result    

=== VM 執行開始 ===
=== VM 執行完畢 ===

記憶體狀態 (全域變數):
[result] = 120
編譯器生成的中間碼 (PC: Quadruples):
--------------------------------------------
000: FUNC_BEG     get_first_score -          -         
001: FORMAL       user_obj   -          -         
002: LOAD_STR     0          -          t1        
003: GET_ITEM     user_obj   t1         t2        
004: IMM          0          -          t3        
005: GET_ITEM     t2         t3         t4        
006: RET_VAL      t4         -          -         
007: FUNC_END     get_first_score -          -         
008: LOAD_STR     1          -          t5        
009: PRINT_VAL    t5         -          -         
010: PRINT_NL     -          -          -         
011: NEW_DICT     -          -          t6        
012: LOAD_STR     2          -          t7        
013: LOAD_STR     3          -          t8        
014: SET_ITEM     t6         t7         t8        
015: LOAD_STR     4          -          t9        
016: IMM          25         -          t10       
017: SET_ITEM     t6         t9         t10       
018: LOAD_STR     5          -          t11       
019: NEW_ARR      -          -          t12       
020: IMM          100        -          t13       
021: APPEND_ITEM  t12        -          t13       
022: IMM          95         -          t14       
023: APPEND_ITEM  t12        -          t14       
024: IMM          80         -          t15       
025: APPEND_ITEM  t12        -          t15       
026: SET_ITEM     t6         t11        t12       
027: LOAD_STR     6          -          t16       
028: IMM          1          -          t17       
029: SET_ITEM     t6         t16        t17       
030: STORE        t6         -          user      
031: LOAD_STR     7          -          t18       
032: PRINT_VAL    t18        -          -         
033: LOAD_STR     8          -          t19       
034: GET_ITEM     user       t19        t20       
035: PRINT_VAL    t20        -          -         
036: PRINT_NL     -          -          -         
037: LOAD_STR     9          -          t21       
038: PRINT_VAL    t21        -          -         
039: PARAM        user       -          -         
040: CALL         get_first_score 1          t22       
041: PRINT_VAL    t22        -          -         
042: PRINT_NL     -          -          -         
043: LOAD_STR     10         -          t23       
044: PRINT_VAL    t23        -          -         
045: PRINT_NL     -          -          -         
046: LOAD_STR     11         -          t24       
047: IMM          2          -          t25       
048: IMM          99         -          t26       
049: GET_ITEM     user       t24        t27       
050: SET_ITEM     t27        t25        t26       
051: LOAD_STR     12         -          t28       
052: LOAD_STR     13         -          t29       
053: SET_ITEM     user       t28        t29       
054: LOAD_STR     14         -          t30       
055: PRINT_VAL    t30        -          -         
056: LOAD_STR     15         -          t31       
057: GET_ITEM     user       t31        t32       
058: PRINT_VAL    t32        -          -         
059: PRINT_NL     -          -          -         
060: LOAD_STR     16         -          t33       
061: PRINT_VAL    t33        -          -         
062: PRINT_VAL    user       -          -         
063: PRINT_NL     -          -          -         

=== VM 執行開始 ===
[程式輸出] >> === 建立複雜的 JSON 資料結構 ===
[程式輸出] >> 使用者姓名: Alice
[程式輸出] >> 第一筆成績是: 100
[程式輸出] >> === 開始修改資料 ===
[程式輸出] >> 修改後的成績陣列: [100, 95, 99]
[程式輸出] >> 修改後的使用者物件: {'name': 'Alice', 'age': 25, 'scores': [100, 95, 99], 'is_active': 1, 'email': 'alice@example.com'}
=== VM 執行完畢 ===

記憶體狀態 (全域變數):
[user] = {'name': 'Alice', 'age': 25, 'scores': [100, 95, 99], 'is_active': 1, 'email': 'alice@example.com'}
編譯器生成的中間碼 (PC: Quadruples):
--------------------------------------------
000: LOAD_STR     0          -          t1        
001: PRINT_VAL    t1         -          -         
002: PRINT_NL     -          -          -         
003: NEW_ARR      -          -          t2        
004: IMM          0          -          t3        
005: APPEND_ITEM  t2         -          t3        
006: IMM          0          -          t4        
007: APPEND_ITEM  t2         -          t4        
008: IMM          0          -          t5        
009: APPEND_ITEM  t2         -          t5        
010: IMM          0          -          t6        
011: APPEND_ITEM  t2         -          t6        
012: IMM          0          -          t7        
013: APPEND_ITEM  t2         -          t7        
014: STORE        t2         -          arr       
015: IMM          0          -          t8        
016: STORE        t8         -          i         
017: IMM          5          -          t9        
018: CMP_LT       i          t9         t10       
019: JMP_F        t10        -          ?         
020: JMP          -          -          ?         
021: IMM          1          -          t11       
022: ADD          i          t11        t12       
023: STORE        t12        -          i         
024: JMP          -          -          17        
025: IMM          2          -          t13       
026: CMP_EQ       i          t13        t14       
027: JMP_F        t14        -          ?         
028: JMP          -          -          21        
029: IMM          10         -          t15       
030: MUL          i          t15        t16       
031: SET_ITEM     arr        i          t16       
032: JMP          -          -          21        
033: LOAD_STR     1          -          t17       
034: PRINT_VAL    t17        -          -         
035: PRINT_VAL    arr        -          -         
036: PRINT_NL     -          -          -         
037: LOAD_STR     2          -          t18       
038: PRINT_VAL    t18        -          -         
039: PRINT_NL     -          -          -         
040: IMM          0          -          t19       
041: STORE        t19        -          count     
042: IMM          1          -          t20       
043: JMP_F        t20        -          ?         
044: IMM          1          -          t21       
045: ADD          count      t21        t22       
046: STORE        t22        -          count     
047: IMM          3          -          t23       
048: CMP_GT       count      t23        t24       
049: JMP_F        t24        -          ?         
050: JMP          -          -          ?         
051: LOAD_STR     3          -          t25       
052: PRINT_VAL    t25        -          -         
053: PRINT_VAL    count      -          -         
054: LOAD_STR     4          -          t26       
055: PRINT_VAL    t26        -          -         
056: PRINT_NL     -          -          -         
057: JMP          -          -          42        
058: LOAD_STR     5          -          t27       
059: PRINT_VAL    t27        -          -         
060: PRINT_NL     -          -          -         

=== VM 執行開始 ===
[程式輸出] >> === 1. 測試 For 迴圈與 Continue ===
[程式輸出] >> 過濾後的陣列: [0, 10, 0, 30, 40]
[程式輸出] >> === 2. 測試 While 迴圈與 Break ===
[程式輸出] >> While 執行第 1 次
[程式輸出] >> While 執行第 2 次
[程式輸出] >> While 執行第 3 次
[程式輸出] >> 跳出迴圈了！
=== VM 執行完畢 ===

記憶體狀態 (全域變數):
[arr] = [0, 10, 0, 30, 40]
[i] = 5
[count] = 4
編譯器生成的中間碼 (PC: Quadruples):
--------------------------------------------
000: FUNC_BEG     fact       -          -         
001: FORMAL       n          -          -         
002: IMM          2          -          t1        
003: CMP_LT       n          t1         t2        

❌ [語法錯誤] 第 2 行, 第 15 字元: 預期 ')' (目前讀到: '{')
      if (n < 2 {
                ^
編譯器生成的中間碼 (PC: Quadruples):
--------------------------------------------
000: FUNC_BEG     test_array -          -         
001: LOAD_STR     0          -          t1        
002: PRINT_VAL    t1         -          -         
003: PRINT_NL     -          -          -         
004: IMM          3          -          t2        
005: PARAM        t2         -          -         
006: IMM          0          -          t3        
007: PARAM        t3         -          -         
008: CALL         array      2          t4        
009: STORE        t4         -          arr       
010: LOAD_STR     1          -          t5        
011: PRINT_VAL    t5         -          -         
012: PRINT_VAL    arr        -          -         
013: PRINT_NL     -          -          -         
014: LOAD_STR     2          -          t6        
015: PRINT_VAL    t6         -          -         
016: PARAM        arr        -          -         
017: CALL         len        1          t7        
018: PRINT_VAL    t7         -          -         
019: PRINT_NL     -          -          -         
020: PARAM        arr        -          -         
021: IMM          99         -          t8        
022: PARAM        t8         -          -         
023: CALL         push       2          t9        
024: PARAM        arr        -          -         
025: IMM          100        -          t10       
026: PARAM        t10        -          -         
027: CALL         push       2          t11       
028: LOAD_STR     3          -          t12       
029: PRINT_VAL    t12        -          -         
030: PRINT_VAL    arr        -          -         
031: PRINT_NL     -          -          -         
032: LOAD_STR     4          -          t13       
033: PRINT_VAL    t13        -          -         
034: PARAM        arr        -          -         
035: CALL         len        1          t14       
036: PRINT_VAL    t14        -          -         
037: PRINT_NL     -          -          -         
038: PARAM        arr        -          -         
039: CALL         pop        1          t15       
040: STORE        t15        -          last_val  
041: LOAD_STR     5          -          t16       
042: PRINT_VAL    t16        -          -         
043: PRINT_VAL    last_val   -          -         
044: PRINT_NL     -          -          -         
045: LOAD_STR     6          -          t17       
046: PRINT_VAL    t17        -          -         
047: PRINT_VAL    arr        -          -         
048: PRINT_NL     -          -          -         
049: LOAD_STR     7          -          t18       
050: PRINT_VAL    t18        -          -         
051: PRINT_NL     -          -          -         
052: FUNC_END     test_array -          -         
053: FUNC_BEG     test_dict  -          -         
054: LOAD_STR     8          -          t19       
055: PRINT_VAL    t19        -          -         
056: PRINT_NL     -          -          -         
057: NEW_DICT     -          -          t20       
058: LOAD_STR     9          -          t21       
059: LOAD_STR     10         -          t22       
060: SET_ITEM     t20        t21        t22       
061: LOAD_STR     11         -          t23       
062: IMM          25         -          t24       
063: SET_ITEM     t20        t23        t24       
064: LOAD_STR     12         -          t25       
065: LOAD_STR     13         -          t26       
066: SET_ITEM     t20        t25        t26       
067: STORE        t20        -          d         
068: LOAD_STR     14         -          t27       
069: PRINT_VAL    t27        -          -         
070: PRINT_VAL    d          -          -         
071: PRINT_NL     -          -          -         
072: LOAD_STR     15         -          t28       
073: PRINT_VAL    t28        -          -         
074: PARAM        d          -          -         
075: CALL         len        1          t29       
076: PRINT_VAL    t29        -          -         
077: PRINT_NL     -          -          -         
078: PARAM        d          -          -         
079: CALL         keys       1          t30       
080: STORE        t30        -          k_list    
081: LOAD_STR     16         -          t31       
082: PRINT_VAL    t31        -          -         
083: PRINT_VAL    k_list     -          -         
084: PRINT_NL     -          -          -         
085: PARAM        d          -          -         
086: LOAD_STR     17         -          t32       
087: PARAM        t32        -          -         
088: CALL         has_key    2          t33       
089: STORE        t33        -          has_name  
090: PARAM        d          -          -         
091: LOAD_STR     18         -          t34       
092: PARAM        t34        -          -         
093: CALL         has_key    2          t35       
094: STORE        t35        -          has_job   
095: LOAD_STR     19         -          t36       
096: PRINT_VAL    t36        -          -         
097: PRINT_VAL    has_name   -          -         
098: PRINT_NL     -          -          -         
099: LOAD_STR     20         -          t37       
100: PRINT_VAL    t37        -          -         
101: PRINT_VAL    has_job    -          -         
102: PRINT_NL     -          -          -         
103: PARAM        d          -          -         
104: LOAD_STR     21         -          t38       
105: PARAM        t38        -          -         
106: CALL         remove     2          t39       
107: LOAD_STR     22         -          t40       
108: PRINT_VAL    t40        -          -         
109: PRINT_VAL    d          -          -         
110: PRINT_NL     -          -          -         
111: LOAD_STR     23         -          t41       
112: PRINT_VAL    t41        -          -         
113: PRINT_NL     -          -          -         
114: FUNC_END     test_dict  -          -         
115: FUNC_BEG     test_type_and_cast -          -         
116: LOAD_STR     24         -          t42       
117: PRINT_VAL    t42        -          -         
118: PRINT_NL     -          -          -         
119: IMM          42         -          t43       
120: STORE        t43        -          n         
121: LOAD_STR     25         -          t44       
122: STORE        t44        -          s         
123: NEW_ARR      -          -          t45       
124: IMM          1          -          t46       
125: APPEND_ITEM  t45        -          t46       
126: IMM          2          -          t47       
127: APPEND_ITEM  t45        -          t47       
128: STORE        t45        -          a         
129: NEW_DICT     -          -          t48       
130: LOAD_STR     26         -          t49       
131: IMM          1          -          t50       
132: SET_ITEM     t48        t49        t50       
133: STORE        t48        -          d         
134: LOAD_STR     27         -          t51       
135: PRINT_VAL    t51        -          -         
136: PARAM        n          -          -         
137: CALL         typeof     1          t52       
138: PRINT_VAL    t52        -          -         
139: PRINT_NL     -          -          -         
140: LOAD_STR     28         -          t53       
141: PRINT_VAL    t53        -          -         
142: PARAM        s          -          -         
143: CALL         typeof     1          t54       
144: PRINT_VAL    t54        -          -         
145: PRINT_NL     -          -          -         
146: LOAD_STR     29         -          t55       
147: PRINT_VAL    t55        -          -         
148: PARAM        a          -          -         
149: CALL         typeof     1          t56       
150: PRINT_VAL    t56        -          -         
151: PRINT_NL     -          -          -         
152: LOAD_STR     30         -          t57       
153: PRINT_VAL    t57        -          -         
154: PARAM        d          -          -         
155: CALL         typeof     1          t58       
156: PRINT_VAL    t58        -          -         
157: PRINT_NL     -          -          -         
158: PARAM        s          -          -         
159: CALL         int        1          t59       
160: IMM          50         -          t60       
161: ADD          t59        t60        t61       
162: STORE        t61        -          parsed    
163: LOAD_STR     31         -          t62       
164: PRINT_VAL    t62        -          -         
165: PRINT_VAL    parsed     -          -         
166: PRINT_NL     -          -          -         
167: PARAM        n          -          -         
168: CALL         str        1          t63       
169: LOAD_STR     32         -          t64       
170: ADD          t63        t64        t65       
171: STORE        t65        -          str_val   
172: LOAD_STR     33         -          t66       
173: PRINT_VAL    t66        -          -         
174: PRINT_VAL    str_val    -          -         
175: PRINT_NL     -          -          -         
176: LOAD_STR     34         -          t67       
177: PRINT_VAL    t67        -          -         
178: PRINT_NL     -          -          -         
179: FUNC_END     test_type_and_cast -          -         
180: FUNC_BEG     test_char_conversion -          -         
181: LOAD_STR     35         -          t68       
182: PRINT_VAL    t68        -          -         
183: PRINT_NL     -          -          -         
184: LOAD_STR     36         -          t69       
185: STORE        t69        -          char_A    
186: PARAM        char_A     -          -         
187: CALL         ord        1          t70       
188: STORE        t70        -          code      
189: LOAD_STR     37         -          t71       
190: PRINT_VAL    t71        -          -         
191: PRINT_VAL    code       -          -         
192: PRINT_NL     -          -          -         
193: IMM          1          -          t72       
194: ADD          code       t72        t73       
195: PARAM        t73        -          -         
196: CALL         chr        1          t74       
197: STORE        t74        -          char_B    
198: LOAD_STR     38         -          t75       
199: PRINT_VAL    t75        -          -         
200: PRINT_VAL    char_B     -          -         
201: PRINT_NL     -          -          -         
202: LOAD_STR     39         -          t76       
203: PRINT_VAL    t76        -          -         
204: PRINT_NL     -          -          -         
205: FUNC_END     test_char_conversion -          -         
206: FUNC_BEG     test_system -          -         
207: LOAD_STR     40         -          t77       
208: PRINT_VAL    t77        -          -         
209: PRINT_NL     -          -          -         
210: CALL         time       0          t78       
211: STORE        t78        -          t1        
212: LOAD_STR     41         -          t79       
213: PRINT_VAL    t79        -          -         
214: PRINT_VAL    t1         -          -         
215: PRINT_NL     -          -          -         
216: CALL         random     0          t80       
217: STORE        t80        -          r         
218: LOAD_STR     42         -          t81       
219: PRINT_VAL    t81        -          -         
220: PRINT_VAL    r          -          -         
221: PRINT_NL     -          -          -         
222: LOAD_STR     43         -          t82       
223: PRINT_VAL    t82        -          -         
224: PRINT_NL     -          -          -         
225: FUNC_END     test_system -          -         
226: FUNC_BEG     main       -          -         
227: LOAD_STR     44         -          t83       
228: PRINT_VAL    t83        -          -         
229: PRINT_NL     -          -          -         
230: LOAD_STR     45         -          t84       
231: PRINT_VAL    t84        -          -         
232: PRINT_NL     -          -          -         
233: CALL         test_array 0          t85       
234: CALL         test_dict  0          t86       
235: CALL         test_type_and_cast 0          t87       
236: CALL         test_char_conversion 0          t88       
237: CALL         test_system 0          t89       
238: LOAD_STR     46         -          t90       
239: PRINT_VAL    t90        -          -         
240: PRINT_NL     -          -          -         
241: LOAD_STR     47         -          t91       
242: PARAM        t91        -          -         
243: CALL         input      1          t92       
244: STORE        t92        -          user_in   
245: LOAD_STR     48         -          t93       
246: PRINT_VAL    t93        -          -         
247: PRINT_VAL    user_in    -          -         
248: PRINT_NL     -          -          -         
249: LOAD_STR     49         -          t94       
250: PRINT_VAL    t94        -          -         
251: PRINT_NL     -          -          -         
252: IMM          0          -          t95       
253: PARAM        t95        -          -         
254: CALL         exit       1          t96       
255: LOAD_STR     50         -          t97       
256: PRINT_VAL    t97        -          -         
257: PRINT_NL     -          -          -         
258: FUNC_END     main       -          -         
259: CALL         main       0          t98       

=== VM 執行開始 ===
[程式輸出] >> >>> 開始執行系統函數測試 <<<
[程式輸出] >> 
[程式輸出] >> --- 1. 測試陣列相關 (array, len, push, pop) ---
[程式輸出] >> 初始化 array(3, 0): [0, 0, 0]
[程式輸出] >> 陣列長度 len(arr): 3
[程式輸出] >> push 兩次後: [0, 0, 0, 99, 100]
[程式輸出] >> 新陣列長度: 5
[程式輸出] >> pop() 彈出的值: 100
[程式輸出] >> pop 後的陣列: [0, 0, 0, 99]
[程式輸出] >> 
[程式輸出] >> --- 2. 測試字典相關 (keys, has_key, remove) ---
[程式輸出] >> 初始字典: {'name': 'Alice', 'age': 25, 'city': 'Taipei'}
[程式輸出] >> 字典長度 len(d): 3
[程式輸出] >> 所有的鍵 keys(d): ['name', 'age', 'city']
[程式輸出] >> 是否包含 'name'? 1
[程式輸出] >> 是否包含 'job'? 0
[程式輸出] >> remove(d, 'age') 之後的字典: {'name': 'Alice', 'city': 'Taipei'}
[程式輸出] >> 
[程式輸出] >> --- 3. 測試型別與轉換 (typeof, int, str) ---
[程式輸出] >> typeof(42)  : int
[程式輸出] >> typeof('100'): string
[程式輸出] >> typeof([1]) : array
[程式輸出] >> typeof({k}) : dict
[程式輸出] >> int('100') + 50 = 150
[程式輸出] >> str(42) 串接結果: 42 是一個字串
[程式輸出] >> 
[程式輸出] >> --- 4. 測試字元與 ASCII 轉換 (ord, chr) ---
[程式輸出] >> ord('A') 的 ASCII 碼 = 65
[程式輸出] >> chr(66) 還原的字元 = B
[程式輸出] >> 
[程式輸出] >> --- 5. 測試系統狀態 (time, random) ---
[程式輸出] >> 當前時間戳 time(): 1772853045.85821
[程式輸出] >> 產生的亂數 random(): 0.7532588601957289
[程式輸出] >> 
[程式輸出] >> --- 6. 測試 I/O 與強制終止 (input, exit) ---
請輸入任意文字 (或直接按 Enter 繼續): abc
[程式輸出] >> 你剛才輸入的是: abc
[程式輸出] >> 準備呼叫 exit(0) 結束虛擬機...