RUST_BACKTRACE=1 ./target/release/tp0c $1.tp0
clang -Wno-override-module $1.ll sys/runtime.c -o $1
