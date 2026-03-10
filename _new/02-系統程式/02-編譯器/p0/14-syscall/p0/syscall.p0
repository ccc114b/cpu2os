// ==========================================
// 系統函數綜合測試程式 (syscall.p0)
// ==========================================

func test_array() {
    print("--- 1. 測試陣列相關 (array, len, push, pop) ---");
    
    // 初始化長度為 3，預設值為 0 的陣列
    arr = array(3, 0);
    print("初始化 array(3, 0):", arr);
    print("陣列長度 len(arr):", len(arr));

    // 推入新元素
    push(arr, 99);
    push(arr, 100);
    print("push 兩次後:", arr);
    print("新陣列長度:", len(arr));

    // 彈出尾端元素
    last_val = pop(arr);
    print("pop() 彈出的值:", last_val);
    print("pop 後的陣列:", arr);
    print("");
}

func test_dict() {
    print("--- 2. 測試字典相關 (keys, has_key, remove) ---");
    
    d = {"name": "Alice", "age": 25, "city": "Taipei"};
    print("初始字典:", d);
    print("字典長度 len(d):", len(d));

    // 取出所有的鍵
    k_list = keys(d);
    print("所有的鍵 keys(d):", k_list);

    // 檢查鍵是否存在 (回傳 1 表示存在，0 表示不存在)
    has_name = has_key(d, "name");
    has_job = has_key(d, "job");
    print("是否包含 'name'?", has_name);
    print("是否包含 'job'?", has_job);

    // 移除指定的鍵
    remove(d, "age");
    print("remove(d, 'age') 之後的字典:", d);
    print("");
}

func test_type_and_cast() {
    print("--- 3. 測試型別與轉換 (typeof, int, str) ---");
    
    n = 42;
    s = "100";
    a =[1, 2];
    d = {"k": 1};

    // 測試 typeof
    print("typeof(42)  :", typeof(n));
    print("typeof('100'):", typeof(s));
    print("typeof([1]) :", typeof(a));
    print("typeof({k}) :", typeof(d));

    // 測試字串轉整數
    parsed = int(s) + 50;
    print("int('100') + 50 =", parsed);

    // 測試整數轉字串
    str_val = str(n) + " 是一個字串";
    print("str(42) 串接結果:", str_val);
    print("");
}

func test_char_conversion() {
    print("--- 4. 測試字元與 ASCII 轉換 (ord, chr) ---");
    
    char_A = "A";
    code = ord(char_A);
    print("ord('A') 的 ASCII 碼 =", code);

    // 66 對應的是 'B'
    char_B = chr(code + 1);
    print("chr(66) 還原的字元 =", char_B);
    print("");
}

func test_system() {
    print("--- 5. 測試系統狀態 (time, random) ---");
    
    // 獲取時間戳
    t1 = time();
    print("當前時間戳 time():", t1);

    // 產生 0.0 ~ 1.0 亂數
    r = random();
    print("產生的亂數 random():", r);
    print("");
}

func main() {
    print(">>> 開始執行系統函數測試 <<<");
    print("");
    
    test_array();
    test_dict();
    test_type_and_cast();
    test_char_conversion();
    test_system();

    print("--- 6. 測試 I/O 與強制終止 (input, exit) ---");
    
    // 程式會在這裡暫停，等待使用者輸入
    user_in = input("請輸入任意文字 (或直接按 Enter 繼續): ");
    print("你剛才輸入的是:", user_in);

    print("準備呼叫 exit(0) 結束虛擬機...");
    exit(0);
    
    print("這行永遠不應該被印出來！因為 VM 已經終止了！");
}

// 程式進入點
main();