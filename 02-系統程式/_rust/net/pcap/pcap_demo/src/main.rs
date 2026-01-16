use pcap::Capture;
use etherparse::{SlicedPacket, TransportSlice, InternetSlice};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 手動指定剛才測試成功的 en1
    let device_name = "en1";
    println!("正在監聽裝置: {}", device_name);

    let mut cap = Capture::from_device(device_name)?
        .promisc(false)
        .snaplen(65535)
        .timeout(100)
        .open()?;

    println!("開始抓取並解析封包... (按 Ctrl+C 停止)\n");

    loop {
        match cap.next_packet() {
            Ok(packet) => {
                // 因為 Linktype 是 1 (Ethernet)，直接用 from_ethernet 即可
                if let Ok(value) = SlicedPacket::from_ethernet(&packet.data) {
                    print_packet_info(&value);
                }
            },
            Err(pcap::Error::TimeoutExpired) => continue,
            Err(e) => {
                eprintln!("發生錯誤: {:?}", e);
                break;
            }
        }
    }
    Ok(())
}

fn print_packet_info(packet: &SlicedPacket) {
    // 解析 IP 層 (IPv4)
    if let Some(InternetSlice::Ipv4(ipv4_slice, ..)) = &packet.net {
        let header = ipv4_slice.header();
        let src_ip = header.source_addr();
        let dst_ip = header.destination_addr();
        
        // 解析傳輸層 (TCP/UDP)
        match &packet.transport {
            Some(TransportSlice::Tcp(tcp)) => {
                println!("[TCP] {}:{} -> {}:{}", 
                    src_ip, tcp.source_port(), 
                    dst_ip, tcp.destination_port()
                );
            },
            Some(TransportSlice::Udp(udp)) => {
                println!("[UDP] {}:{} -> {}:{}", 
                    src_ip, udp.source_port(), 
                    dst_ip, udp.destination_port()
                );
            },
            _ => {
                // 其他協定如 ICMP (Ping)
                println!("[IPv4] {} -> {} (Protocol: {:?})", 
                    src_ip, dst_ip, header.protocol()
                );
            }
        }
    } else if let Some(InternetSlice::Ipv6(ipv6_slice, ..)) = &packet.net {
        // 如果有 IPv6 流量也印一下
        let header = ipv6_slice.header();
        println!("[IPv6] {} -> {}", header.source_addr(), header.destination_addr());
    }
}