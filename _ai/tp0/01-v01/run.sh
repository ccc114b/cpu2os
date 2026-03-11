set -x
RUST_BACKTRACE=1 ./target/release/tp0c $1.tp0
clang -Wno-override-module -S $1.ll -o $1.s
clang -Wno-override-module $1.ll sys/runtime.c -o $1
./$1
