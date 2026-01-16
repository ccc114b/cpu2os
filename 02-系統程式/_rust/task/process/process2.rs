use std::process::Command;

fn main() {
    let output = Command::new("ls") // Windows 下請改用 "dir" (搭配 cmd /C)
        .arg("-l")
        .arg("-a")
        .output()
        .expect("執行命令失敗");

    if output.status.success() {
        // 將位元組轉為字串
        let s = String::from_utf8_lossy(&output.stdout);
        println!("命令執行成功，輸出如下：\n{}", s);
    } else {
        let err = String::from_utf8_lossy(&output.stderr);
        println!("命令執行失敗，錯誤訊息：{}", err);
    }
}