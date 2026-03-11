use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

/// 定義我們支援的 LLVM IR 指令集 (抽象語法樹)
#[derive(Debug)]
enum IRInstruction {
    Add { dest: String, op1: String, op2: String },
    Ret { op: String },
}

#[derive(Debug)]
struct Function {
    name: String,
    instructions: Vec<IRInstruction>,
}

fn main() {
    // 1. 處理命令列參數
    let args: Vec<String> = env::args().collect();
    
    // 檢查是否提供了正確的參數數量
    if args.len() != 2 {
        eprintln!("用法: {} <輸入的 .ll 檔案>", args[0]);
        eprintln!("範例: {} file.ll", args[0]);
        process::exit(1);
    }

    let input_file = &args[1];
    let input_path = Path::new(input_file);

    // 自動決定輸出檔名：將副檔名替換為 .s
    // 例如: "file.ll" -> "file.s", "path/to/mycode.ll" -> "path/to/mycode.s"
    let output_path = input_path.with_extension("s");
    let output_file = output_path.to_str().expect("無法解析輸出路徑");

    // 2. 讀取 LLVM IR 檔案
    let ll_code = fs::read_to_string(input_file).unwrap_or_else(|err| {
        eprintln!("❌ 錯誤: 無法讀取檔案 '{}'\n原因: {}", input_file, err);
        process::exit(1);
    });
    
    println!("讀取檔案: {}", input_file);

    // 3. 解析 IR
    let function = parse_llvm_ir(&ll_code);
    println!("成功解析 IR，正在生成機器碼...");

    // 4. 程式碼生成 (Instruction Selection & Register Allocation)
    let asm = generate_arm64_assembly(&function);

    // 5. 輸出 .s 檔
    fs::write(&output_path, &asm).unwrap_or_else(|err| {
        eprintln!("❌ 錯誤: 無法寫入檔案 '{}'\n原因: {}", output_file, err);
        process::exit(1);
    });

    println!("🎉 成功生成組合語言至: {}", output_file);
}

/// 極簡版 LLVM IR 解析器
fn parse_llvm_ir(code: &str) -> Function {
    let mut func_name = String::new();
    let mut instructions = Vec::new();

    for line in code.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with(';') { continue; }

        if line.starts_with("define") {
            let start = line.find('@').unwrap() + 1;
            let end = line.find('(').unwrap();
            func_name = line[start..end].to_string();
        } else if line.contains("= add") {
            let parts: Vec<&str> = line.split('=').collect();
            let dest = parts[0].trim().to_string();
            
            let rhs: Vec<&str> = parts[1].split(',').collect();
            let op1 = rhs[0].split_whitespace().last().unwrap().to_string();
            let op2 = rhs[1].trim().to_string();

            instructions.push(IRInstruction::Add { dest, op1, op2 });
        } else if line.starts_with("ret") {
            let op = line.split_whitespace().last().unwrap().to_string();
            instructions.push(IRInstruction::Ret { op });
        }
    }

    Function { name: func_name, instructions }
}

/// 將 IR 轉換為 M3 (ARM64) 組合語言
fn generate_arm64_assembly(func: &Function) -> String {
    let mut asm = String::new();
    let macos_func_name = format!("_{}", func.name);

    asm.push_str(".text\n");
    asm.push_str(&format!(".global {}\n", macos_func_name));
    asm.push_str(".align 2\n");
    asm.push_str(&format!("{}:\n", macos_func_name));

    let mut reg_map: HashMap<String, String> = HashMap::new();
    let mut next_free_reg = 8; 

    for inst in &func.instructions {
        match inst {
            IRInstruction::Add { dest, op1, op2 } => {
                let r1 = load_operand_to_reg(op1, &mut next_free_reg, &mut asm, &reg_map);
                let r2 = load_operand_to_reg(op2, &mut next_free_reg, &mut asm, &reg_map);
                
                let dest_reg = format!("w{}", next_free_reg);
                reg_map.insert(dest.clone(), dest_reg.clone());
                next_free_reg += 1;

                asm.push_str(&format!("    add {}, {}, {}\n", dest_reg, r1, r2));
            }
            IRInstruction::Ret { op } => {
                if is_constant(op) {
                    asm.push_str(&format!("    mov w0, #{}\n", op));
                } else {
                    let reg = reg_map.get(op).expect("找不到變數暫存器");
                    asm.push_str(&format!("    mov w0, {}\n", reg));
                }
                asm.push_str("    ret\n");
            }
        }
    }

    asm
}

fn load_operand_to_reg(op: &str, next_free_reg: &mut u32, asm: &mut String, reg_map: &HashMap<String, String>) -> String {
    if is_constant(op) {
        let reg = format!("w{}", next_free_reg);
        *next_free_reg += 1;
        asm.push_str(&format!("    mov {}, #{}\n", reg, op));
        reg
    } else {
        reg_map.get(op).expect("未定義的虛擬暫存器").clone()
    }
}

fn is_constant(op: &str) -> bool {
    op.parse::<i32>().is_ok()
}