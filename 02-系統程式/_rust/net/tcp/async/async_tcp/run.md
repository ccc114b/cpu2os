
使用 sync 版的 client 來測試，連接本 server

```
(py310) cccimac@cccimacdeiMac async_tcp % cargo run
    Updating crates.io index
     Locking 34 packages to latest Rust 1.92.0 compatible versions
   Compiling libc v0.2.180
   Compiling proc-macro2 v1.0.105
   Compiling quote v1.0.43
   Compiling unicode-ident v1.0.22
   Compiling parking_lot_core v0.9.12
   Compiling smallvec v1.15.1
   Compiling scopeguard v1.2.0
   Compiling cfg-if v1.0.4
   Compiling bytes v1.11.0
   Compiling lock_api v0.4.14
   Compiling pin-project-lite v0.2.16
   Compiling errno v0.3.14
   Compiling mio v1.1.1
   Compiling socket2 v0.6.1
   Compiling signal-hook-registry v1.4.8
   Compiling syn v2.0.114
   Compiling parking_lot v0.12.5
   Compiling tokio-macros v2.6.0
   Compiling tokio v1.49.0
   Compiling async_tcp v0.1.0 (/Users/cccimac/Desktop/ccc/cpu2os/02-系統程式/_rust/net/tcp/async/async_tcp)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 6.70s
     Running `target/debug/async_tcp`
Error: Os { code: 48, kind: AddrInUse, message: "Address already in use" }
(py310) cccimac@cccimacdeiMac async_tcp % cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/async_tcp`
非同步伺服器啟動於 8080

```

client

```
(py310) cccimac@cccimacdeiMac sync % ./client
已連線至伺服器
收到回傳: Hello, Rust TCP!
(py310) cccimac@cccimacdeiMac sync % ./client
已連線至伺服器
收到回傳: Hello, Rust TCP!
```