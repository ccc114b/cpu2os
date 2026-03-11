RUST_BACKTRACE=1 ./target/release/ll2asm $1.ll
clang -o $1 $1.s
./$1
echo $?

