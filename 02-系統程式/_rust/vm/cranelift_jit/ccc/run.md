```
(py310) cccimac@cccimacdeiMac cranelift_jit % cargo build
    Updating crates.io index
     Locking 57 packages to latest Rust 1.85.1 compatible versions
      Adding cranelift v0.106.2 (available: v0.121.2)
      Adding cranelift-jit v0.106.2 (available: v0.121.2)
      Adding cranelift-module v0.106.2 (available: v0.121.2)
      Adding cranelift-native v0.106.2 (available: v0.121.2)
   Compiling version_check v0.9.5
   Compiling zerocopy v0.8.33
   Compiling cfg-if v1.0.4
   Compiling cranelift-isle v0.106.2
   Compiling cranelift-codegen-shared v0.106.2
   Compiling once_cell v1.21.3
   Compiling equivalent v1.0.2
   Compiling target-lexicon v0.12.16
   Compiling hashbrown v0.16.1
   Compiling slice-group-by v0.3.1
   Compiling cranelift-codegen-meta v0.106.2
   Compiling log v0.4.29
   Compiling smallvec v1.15.1
   Compiling ahash v0.8.12
   Compiling cranelift-entity v0.106.2
   Compiling rustc-hash v1.1.0
   Compiling arbitrary v1.4.2
   Compiling indexmap v2.13.0
   Compiling cranelift-bforest v0.106.2
   Compiling hashbrown v0.14.5
   Compiling gimli v0.28.1
   Compiling cranelift-control v0.106.2
   Compiling bumpalo v3.19.1
   Compiling libc v0.2.180
   Compiling anyhow v1.0.100
   Compiling bitflags v1.3.2
   Compiling cranelift-codegen v0.106.2
   Compiling mach v0.3.2
   Compiling wasmtime-jit-icache-coherence v19.0.2
   Compiling hashbrown v0.13.2
   Compiling region v2.2.0
   Compiling regalloc2 v0.9.3
   Compiling cranelift-native v0.106.2
   Compiling cranelift-frontend v0.106.2
   Compiling cranelift-module v0.106.2
   Compiling cranelift-jit v0.106.2
   Compiling cranelift v0.106.2
   Compiling cranelift_jit v0.1.0 (/Users/cccimac/Desktop/ccc/cpu2os/02-系統程式/_rust/jit_cranelift/cranelift_jit)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 15.00s
(py310) cccimac@cccimacdeiMac cranelift_jit % cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/cranelift_jit`
JIT 生成的加法函數結果: 30
```