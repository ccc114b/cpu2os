use std::io::{self, Write};

fn main() {
    loop {
        // 1. 顯示提示字元
        print!("myshell > ");
        // 必須手動 flush，因為 print! 不會自動換行，輸出可能會被留在緩衝區
        io::stdout().flush().unwrap();

        // 2. 讀取使用者輸入
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // 去掉結尾的換行符號
        let input = input.trim();

        // 如果輸入為空，繼續下一次循環
        if input.is_empty() {
            continue;
        }

        println!("你輸入了: {}", input);
    }
}