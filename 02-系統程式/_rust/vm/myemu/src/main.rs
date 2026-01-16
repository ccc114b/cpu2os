use dynasmrt::{dynasm, DynasmApi};
use goblin::elf::Elf;
use std::fs;
use std::mem;

#[repr(C)]
struct Cpu {
    regs: [u64; 32],
    pc: u64,
    exit_flag: u64,
}

impl Cpu {
    fn new(entry_point: u64) -> Self {
        let mut cpu = Self {
            regs: [0; 32],
            pc: entry_point,
            exit_flag: 0,
        };
        cpu.regs[2] = 0x7ffffff0; // SP (棧指標)
        cpu
    }
}

struct Jitter;

impl Jitter {
    fn compile(instruction: u32) -> dynasmrt::ExecutableBuffer {
        let mut ops = dynasmrt::aarch64::Assembler::new().unwrap();

        let opcode = instruction & 0x7f;
        let rd = ((instruction >> 7) & 0x1f) as usize;
        let funct3 = (instruction >> 12) & 0x7;
        let rs1 = ((instruction >> 15) & 0x1f) as usize;
        let rs2 = ((instruction >> 20) & 0x1f) as usize;
        
        // 取得 12 位元的帶正負號立即數
        let imm_i = ((instruction as i32) >> 20) as i32; 
        // 取得 20 位元的 U-type 立即數 (用於 LUI)
        let imm_u = (instruction & 0xfffff000) as i32;

        let rd_offset = (rd * 8) as u32;
        let rs1_offset = (rs1 * 8) as u32;
        let rs2_offset = (rs2 * 8) as u32;

        match opcode {
            0x13 if funct3 == 0 => { // ADDI
                if imm_i >= 0 {
                    let val = imm_i as u32;
                    dynasm!(ops
                        ; .arch aarch64
                        ; ldr x9, [x0, rs1_offset]
                        ; add x9, x9, val      // 移除括號
                        ; str x9, [x0, rd_offset]
                    );
                } else {
                    let val = imm_i.abs() as u32;
                    dynasm!(ops
                        ; .arch aarch64
                        ; ldr x9, [x0, rs1_offset]
                        ; sub x9, x9, val      // 移除括號
                        ; str x9, [x0, rd_offset]
                    );
                }
            }
            0x33 if funct3 == 0 => { // ADD (暫存器 + 暫存器)
                dynasm!(ops
                    ; .arch aarch64
                    ; ldr x9, [x0, rs1_offset]
                    ; ldr x10, [x0, rs2_offset]
                    ; add x9, x9, x10
                    ; str x9, [x0, rd_offset]
                );
            }
            0x37 => { // LUI
                let val = imm_u as u64;
                let low = (val & 0xFFFF) as u32;
                let high = ((val >> 16) & 0xFFFF) as u32;
                dynasm!(ops
                    ; .arch aarch64
                    ; movz x9, low             // 移除括號
                    ; movk x9, high, lsl 16    // 移除括號
                    ; str x9, [x0, rd_offset]
                );
            }
            0x73 => { // ECALL
                dynasm!(ops
                    ; .arch aarch64
                    ; mov x9, 1
                    ; str x9, [x0, 264] 
                );
            }
            _ => {
                // 不認識的指令，不做事
            }
        }

        // 統一加上 ret
        dynasm!(ops
            ; .arch aarch64
            ; ret
        );

        ops.finalize().unwrap()
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: cargo run <riscv64_elf_file>");
        return;
    }

    let path = &args[1];
    let buffer = fs::read(path).expect("Failed to read file");
    let elf = Elf::parse(&buffer).expect("Failed to parse ELF");

    let mut mem_space = vec![0u8; 1024 * 1024]; 
    let entry_point = elf.entry;
    
    for ph in elf.program_headers {
        if ph.p_type == goblin::elf::program_header::PT_LOAD {
            let start = ph.p_vaddr as usize;
            let filesz = ph.p_filesz as usize;
            let offset = ph.p_offset as usize;
            if start + filesz <= mem_space.len() {
                mem_space[start..start + filesz].copy_from_slice(&buffer[offset..offset + filesz]);
            }
        }
    }

    let mut cpu = Cpu::new(entry_point);
    println!("myemu: Starting at PC 0x{:x}", cpu.pc);

    loop {
        cpu.regs[0] = 0; // x0 始終為 0

        let pc = cpu.pc as usize;
        if pc + 4 > mem_space.len() { break; }

        let instr_bytes = &mem_space[pc..pc + 4];
        let instruction = u32::from_le_bytes(instr_bytes.try_into().unwrap());

        if instruction == 0 { break; }

        // JIT 編譯
        let code = Jitter::compile(instruction);
        
        // 修改函數簽名為接受一個參數: *mut Cpu
        let f: extern "C" fn(*mut Cpu) = unsafe { 
            mem::transmute(code.ptr(dynasmrt::AssemblyOffset(0))) 
        };

        // 執行並傳入 cpu 指標，這會進入 ARM64 的 x0 暫存器
        f(&mut cpu);

        if cpu.exit_flag != 0 {
            println!("myemu: Exit requested.");
            break;
        }

        // 基本流控制：簡單遞增 PC
        cpu.pc += 4;

        // 如果 a0 有變化，印出來看看
        let opcode = instruction & 0x7f;
        if opcode == 0x13 || opcode == 0x33 || opcode == 0x37 {
            println!("  PC: 0x{:x} | a0: {}", pc, cpu.regs[10]);
        }

        if cpu.pc > entry_point + 1000 { break; }
    }
    
    println!("Final a0: {}", cpu.regs[10]);
}