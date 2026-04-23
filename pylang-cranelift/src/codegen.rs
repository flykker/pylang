use pylang_ir::Function as IrFunction;

#[allow(dead_code)]
pub fn lower_ir_to_clif(_func: &IrFunction) -> Result<String, String> {
    Ok(".text\n".to_string())
}

#[allow(dead_code)]
pub fn compile_to_bytes(_func: &IrFunction) -> Result<Vec<u8>, String> {
    Ok(vec![0x90, 0xc3])
}