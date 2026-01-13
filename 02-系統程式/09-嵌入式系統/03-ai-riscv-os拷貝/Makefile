# RISC-V 64-bit RTOS Makefile

# 工具鏈設定
CROSS_COMPILE = riscv64-unknown-elf-
CC = $(CROSS_COMPILE)gcc
AS = $(CROSS_COMPILE)as
LD = $(CROSS_COMPILE)ld
OBJCOPY = $(CROSS_COMPILE)objcopy
OBJDUMP = $(CROSS_COMPILE)objdump

# 編譯選項
ARCH = rv64imac_zicsr
ABI = lp64
CFLAGS = -march=$(ARCH) -mabi=$(ABI) -O2 -g -Wall
CFLAGS += -fno-builtin -nostdlib -nostartfiles
CFLAGS += -ffreestanding -fno-common
CFLAGS += -mcmodel=medany

LDFLAGS = -T linker.ld

# 目標檔案
TARGET = rtos
OBJS = startup.o main.o

# 預設目標
all: $(TARGET).elf $(TARGET).bin $(TARGET).dump

# 編譯 C 檔案
%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@

# 編譯組合語言檔案
%.o: %.S
	$(CC) $(CFLAGS) -c $< -o $@

# 連結（使用 gcc 而非 ld，這樣可以正確處理所有選項）
$(TARGET).elf: $(OBJS)
	$(CC) $(CFLAGS) $(LDFLAGS) $(OBJS) -o $@

# 產生 binary
$(TARGET).bin: $(TARGET).elf
	$(OBJCOPY) -O binary $< $@

# 產生反組譯檔案
$(TARGET).dump: $(TARGET).elf
	$(OBJDUMP) -D $< > $@

# 清理
clean:
	rm -f $(OBJS) $(TARGET).elf $(TARGET).bin $(TARGET).dump

# 使用 QEMU 執行
run: $(TARGET).elf
	qemu-system-riscv64 -machine virt -nographic -bios none -kernel $(TARGET).elf

# 使用 GDB 除錯
debug: $(TARGET).elf
	qemu-system-riscv64 -machine virt -nographic -bios none -kernel $(TARGET).elf -s -S

.PHONY: all clean run debug