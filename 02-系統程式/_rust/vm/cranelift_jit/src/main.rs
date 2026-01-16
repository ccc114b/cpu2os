use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{Linkage, Module};
use std::mem;

fn main() {
    // ---- 1. 初始化 JIT 環境 ----
    // 取得當前電腦的硬體架構 (ISA: Instruction Set Architecture)
    let isa_builder = cranelift_native::builder().unwrap();
    let isa = isa_builder.finish(settings::Flags::new(settings::builder())).unwrap();

    // 建立 JIT 模組，這負責管理機器碼所在的記憶體
    let builder = JITBuilder::with_isa(isa, cranelift_module::default_libcall_names());
    let mut module = JITModule::new(builder);

    // ---- 2. 建立函數簽名 (Signature) ----
    // 定義一個函數，接收 (I32, I32) 並回傳 I32
    let mut sig = module.make_signature();
    sig.params.push(AbiParam::new(types::I32)); // 參數 a
    sig.params.push(AbiParam::new(types::I32)); // 參數 b
    sig.returns.push(AbiParam::new(types::I32)); // 回傳值

    // ---- 3. 撰寫函數內容 ----
    let mut ctx = codegen::Context::new();
    ctx.func.signature = sig;

    let mut func_ctx = FunctionBuilderContext::new();
    let mut builder = FunctionBuilder::new(&mut ctx.func, &mut func_ctx);

    // 建立函數的進入點區塊 (Entry Block)
    let block = builder.create_block();
    // 讓區塊接收函數的參數
    builder.append_block_params_for_function_params(block);
    // 切換到該區塊開始寫指令
    builder.switch_to_block(block);
    builder.seal_block(block);

    // 取得傳入的參數值
    let arg_a = builder.block_params(block)[0];
    let arg_b = builder.block_params(block)[1];

    // 生成加法指令： sum = a + b
    let sum = builder.ins().iadd(arg_a, arg_b);

    // 生成回傳指令
    builder.ins().return_(&[sum]);

    // 結束函數建置
    builder.finalize();

    // ---- 4. 編譯與執行 ----
    // 在模組中宣告並定義函數
    let id = module
        .declare_function("add_numbers", Linkage::Export, &ctx.func.signature)
        .unwrap();

    module.define_function(id, &mut ctx).unwrap();

    // 進行最後的連結與記憶體配置
    module.finalize_definitions().unwrap();

    // 取得機器碼在記憶體中的位址
    let code_ptr = module.get_finalized_function(id);

    // 將位址轉為 Rust 的函數型別 (unsafe)
    let add_fn: extern "C" fn(i32, i32) -> i32 = unsafe { mem::transmute(code_ptr) };

    // 測試執行！
    let res = add_fn(10, 20);
    println!("JIT 生成的加法函數結果: {}", res); // 輸出 30
}