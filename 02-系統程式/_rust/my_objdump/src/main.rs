use goblin::{Object, mach::{Mach, MachO}};
use capstone::prelude::*;
use std::{env, fs};
use anyhow::{Result, Context};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("使用方法: {} <file_path>", args[0]);
        return Ok(());
    }

    let path = &args[1];
    let buffer = fs::read(path).context("無法讀取檔案")?;

    match Object::parse(&buffer).context("無法解析檔案格式")? {
        Object::Elf(elf) => {
            println!("格式: ELF (Linux/Unix)");
            process_elf(&elf, &buffer)?;
        }
        Object::PE(pe) => {
            println!("格式: PE (Windows)");
            process_pe(&pe, &buffer)?;
        }
        Object::Mach(mach) => {
            println!("格式: Mach-O (macOS)");
            process_mach(mach, &buffer)?;
        }
        _ => println!("不支援的檔案格式"),
    }

    Ok(())
}

// --- Windows PE 處理 ---
fn process_pe(pe: &goblin::pe::PE, buffer: &[u8]) -> Result<()> {
    println!("架構: {:#x}", pe.header.coff_header.machine);
    for section in &pe.sections {
        println!("{:<15} {:08x} {:08x}", section.name().unwrap_or("?"), section.virtual_address, section.virtual_size);
    }

    let cs = if pe.is_64 {
        Capstone::new().x86().mode(arch::x86::ArchMode::Mode64).syntax(arch::x86::ArchSyntax::Intel).build()
    } else {
        Capstone::new().x86().mode(arch::x86::ArchMode::Mode32).syntax(arch::x86::ArchSyntax::Intel).build()
    }.map_err(|e| anyhow::anyhow!("{}", e))?;

    if let Some(sec) = pe.sections.iter().find(|s| s.name().unwrap_or("") == ".text") {
        let start = sec.pointer_to_raw_data as usize;
        let end = start + sec.size_of_raw_data as usize;
        let code = &buffer[start..end];
        let insns = cs.disasm_all(code, pe.image_base as u64 + sec.virtual_address as u64).map_err(|e| anyhow::anyhow!("{}", e))?;
        println!("\nDisassembly (.text):");
        for i in insns.iter().take(15) {
            println!("  {:08x}:  {} {}", i.address(), i.mnemonic().unwrap_or(""), i.op_str().unwrap_or(""));
        }
    }
    Ok(())
}

// --- Linux ELF 處理 ---
fn process_elf(elf: &goblin::elf::Elf, buffer: &[u8]) -> Result<()> {
    println!("{:<20} {:<16}", "Section", "Addr");
    for sh in &elf.section_headers {
        println!("{:<20} {:016x}", elf.shdr_strtab.get_at(sh.sh_name).unwrap_or("?"), sh.sh_addr);
    }

    if elf.header.e_machine == goblin::elf::header::EM_X86_64 {
        let cs = Capstone::new().x86().mode(arch::x86::ArchMode::Mode64).syntax(arch::x86::ArchSyntax::Intel).build().map_err(|e| anyhow::anyhow!("{}", e))?;
        for shdr in &elf.section_headers {
            if shdr.is_executable() && shdr.sh_size > 0 {
                let offset = shdr.sh_offset as usize;
                let code = &buffer[offset..offset + shdr.sh_size as usize];
                let insns = cs.disasm_all(code, shdr.sh_addr).map_err(|e| anyhow::anyhow!("{}", e))?;
                println!("\nDisassembly ({}):", elf.shdr_strtab.get_at(shdr.sh_name).unwrap_or(""));
                for i in insns.iter().take(15) {
                    println!("  {:08x}:  {} {}", i.address(), i.mnemonic().unwrap_or(""), i.op_str().unwrap_or(""));
                }
            }
        }
    }
    Ok(())
}

fn process_mach(mach: Mach, buffer: &[u8]) -> Result<()> {
    match mach {
        Mach::Binary(macho) => {
            process_single_macho(&macho)?;
        }
        Mach::Fat(fat) => {
            println!("偵測到 Fat Binary (包含 {} 個架構)", fat.narches);

            // 定義我們要尋找的目標：在 M3 Mac 上，目標是 ARM64 (16777228)
            let target_cpu = if cfg!(target_arch = "aarch64") {
                goblin::mach::constants::cputype::CPU_TYPE_ARM64
            } else {
                goblin::mach::constants::cputype::CPU_TYPE_X86_64
            };

            let mut selected_arch = None;

            // 遍歷所有架構，尋找最合適的一個
            for arch_res in fat.iter_arches() {
                if let Ok(arch) = arch_res {
                    if arch.cputype == target_cpu {
                        selected_arch = Some(arch);
                        break; // 找到了完全匹配的，直接跳出
                    }
                    // 如果沒找到完全匹配的，暫時記住第一個，當作備選
                    if selected_arch.is_none() {
                        selected_arch = Some(arch);
                    }
                }
            }

            if let Some(arch) = selected_arch {
                println!("選擇架構: CPU Type {} (Offset: {})", arch.cputype, arch.offset);
                let macho = MachO::parse(buffer, arch.offset as usize).context("解析 Mach-O 失敗")?;
                process_single_macho(&macho)?;
            }
        }
    }
    Ok(())
}

fn process_single_macho(macho: &MachO) -> Result<()> {
    println!("架構詳細資訊 (CPU Type: {:?})", macho.header.cputype);
    println!("{:<25} {:<16} {:<16}", "Section", "Addr", "Size");

    for segment in &macho.segments {
        for section_res in segment {
            let (section, _) = section_res.map_err(|e| anyhow::anyhow!("{:?}", e))?;
            println!("{:<25} {:016x} {:016x}", section.name().unwrap_or("?"), section.addr, section.size);
        }
    }

    // 支援 Intel 與 Apple Silicon 的反組譯
    let cs_result = if macho.header.cputype == goblin::mach::constants::cputype::CPU_TYPE_X86_64 {
        Capstone::new().x86().mode(arch::x86::ArchMode::Mode64).syntax(arch::x86::ArchSyntax::Intel).build()
    } else if macho.header.cputype == goblin::mach::constants::cputype::CPU_TYPE_ARM64 {
        Capstone::new().arm64().mode(arch::arm64::ArchMode::Arm).build()
    } else {
        println!("\n[提示] 暫不支援此架構的反組譯");
        return Ok(());
    };

    let cs = cs_result.map_err(|e| anyhow::anyhow!("{}", e))?;

    for segment in &macho.segments {
        for section_res in segment {
            let (section, data) = section_res.map_err(|e| anyhow::anyhow!("{:?}", e))?;
            if section.name().unwrap_or("") == "__text" {
                println!("\nDisassembly (__text section):");
                let insns = cs.disasm_all(data, section.addr).map_err(|e| anyhow::anyhow!("{}", e))?;
                for i in insns.iter().take(15) {
                    println!("  {:08x}:  {:<10} {}", i.address(), i.mnemonic().unwrap_or(""), i.op_str().unwrap_or(""));
                }
            }
        }
    }
    Ok(())
}