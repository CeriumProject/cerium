use crate::error::CompilerResult;

// TODO: add default implementations based on compile_mut
// TODO: add context
pub trait Compilable {
    fn compile(&self) -> CompilerResult<(Vec<chasm_ir::Instruction>, chasm_ir::Operand)>;
    fn compile_mut(&self) -> CompilerResult<(Vec<chasm_ir::Instruction>, chasm_ir::Operand)>;
    fn compile_unit(&self) -> CompilerResult<Vec<chasm_ir::Instruction>>;
    fn compile_into(&self, operand: chasm_ir::Operand) -> CompilerResult<Vec<chasm_ir::Instruction>>;
}