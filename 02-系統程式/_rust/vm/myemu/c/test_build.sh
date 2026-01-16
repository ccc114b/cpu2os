# riscv64-unknown-elf-gcc -nostdlib -static -o test_bin test.c
riscv64-unknown-elf-gcc -march=rv64i -mabi=lp64 -nostdlib -static -o test_bin test.c