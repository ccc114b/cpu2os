use pcap::{Device, Capture};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 取得所有網卡清單
    let devices = Device::list()?;
    
    println!("--- 偵測到以下網卡 ---");
    for (i, d) in devices.iter().enumerate() {
        let ips: Vec<_> = d.addresses.iter().map(|a| format!("{:?}", a.addr)).collect();
        println!("{}: {} (IPs: {})", i, d.name, ips.join(", "));
    }
    println!("----------------------\n");

    // 2. 找出第一個有 IPv4 地址且不是 loopback 的網卡，或是你手動指定
    // 如果你知道是哪一張（例如 en1），可以改為 d.name == "en1"
    let target_device = devices.into_iter()
        .find(|d| {
            // 條件：有分配地址，且不是迴圈地址
            !d.addresses.is_empty() && d.name != "lo0"
        })
        .ok_or("找不到活動中的實體網卡")?;

    println!("嘗試監聽: {}", target_device.name);

    let mut cap = Capture::from_device(target_device)?
        .promisc(false)
        .snaplen(65535)
        .timeout(100)
        .open()?;

    // 檢查資料鏈路層類型 (非常重要！)
    // macOS 上常見的有 Ethernet (1) 或 Null/Loopback (0)
    let link_type = cap.get_datalink();
    println!("網卡資料類型: {:?}", link_type);

    println!("開始監聽... (請去打開網頁或執行 ping)");

    let mut packet_count = 0;
    loop {
        match cap.next_packet() {
            Ok(packet) => {
                packet_count += 1;
                // 先印出最原始的資訊，確認真的有「抓到東西」
                println!("[#{}] 抓到封包！長度: {} bytes", packet_count, packet.header.len);
                
                // 這裡暫時不做複雜解析，先確認有沒有東西進來
            },
            Err(pcap::Error::TimeoutExpired) => continue,
            Err(e) => {
                println!("錯誤: {:?}", e);
                break;
            }
        }
    }

    Ok(())
}