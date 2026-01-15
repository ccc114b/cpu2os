use nix::mount::{mount, MsFlags};
use nix::sched::{clone, CloneFlags};
use nix::unistd::{chdir, execvp, pivot_root, sethostname};
use std::ffi::CString;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        eprintln!("用法: sudo cargo run <command>");
        return;
    }

    match args[1].as_str() {
        "run" => run(&args[2..]),
        "child" => child(&args[2..]),
        _ => println!("未知命令"),
    }
}

fn run(args: &[String]) {
    println!("主行程: 正在建立隔離空間...");

    // 定義子行程所需的隔離標籤
    // NEWUTS: 獨立主機名, NEWPID: 獨立進程樹, NEWNS: 獨立掛載點
    let flags = CloneFlags::CLONE_NEWUTS 
              | CloneFlags::CLONE_NEWPID 
              | CloneFlags::CLONE_NEWNS;

    // 重新呼叫自己，並進入 child 函數
    let mut stack = [0u8; 1024 * 1024]; // 1MB 棧空間
    let mut new_args = vec!["child".to_string()];
    new_args.extend_from_slice(args);

    let callback = Box::new(|| {
        child(args);
        0
    });

    clone(callback, &mut stack, flags, None).expect("Clone 失敗");
}

fn child(args: &[String]) {
    println!("子行程: 進入隔離環境 PID: {}", std::process::id());

    // 1. 設定主機名 (UTS Namespace)
    sethostname("my-container").expect("設定主機名失敗");

    // 2. 隔離檔案系統 (需要先準備一個 rootfs 目錄，例如從 alpine 官網下載的)
    let rootfs = Path::new("./rootfs");
    setup_rootfs(rootfs);

    // 3. 掛載 /proc 以便 ps 指令能運作
    mount(
        Some("proc"),
        "/proc",
        Some("proc"),
        MsFlags::empty(),
        None::<&str>,
    ).expect("掛載 proc 失敗");

    // 4. 執行目標指令
    let cmd = CString::new(args[0].clone()).unwrap();
    let c_args: Vec<CString> = args.iter()
        .map(|s| CString::new(s.clone()).unwrap())
        .collect();

    println!("執行指令: {:?}", args);
    execvp(&cmd, &c_args).expect("執行失敗");
}

fn setup_rootfs(new_root: &Path) {
    // pivot_root 需要 new_root 本身是一個掛載點
    // 這裡簡單使用 bind mount 將目錄掛載到自己身上來達成
    mount(
        Some(new_root),
        new_root,
        None::<&str>,
        MsFlags::MS_BIND | MsFlags::MS_REC,
        None::<&str>,
    ).unwrap();

    let old_root = new_root.join(".old_root");
    fs::create_dir_all(&old_root).unwrap();

    pivot_root(new_root, &old_root).expect("pivot_root 失敗");
    chdir("/").unwrap();
    
    // 卸載舊的根目錄（可選）
    // nix::mount::umount2("/.old_root", nix::mount::MntFlags::MNT_DETACH).ok();
}