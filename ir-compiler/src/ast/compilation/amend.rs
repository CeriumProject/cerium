#[macro_export]
macro_rules! amend {
    ($code:expr, $follow_up:expr) => {
        crate::ast::compilation::amend::_amend($code, $follow_up)
    };
}

pub(crate) fn _amend(mut outer: Vec<Instruction>, inner: Vec<Instruction>) -> Vec<Instruction> {
    if let Some(
        Instruction::Alloc(_, _, body)
        | Instruction::Param(_, _, body)
        | Instruction::Result(_, _, body),
    ) = outer.last_mut()
    {
        body.extend(inner);
    } else {
        outer.extend(inner);
    }
    outer
}

pub use amend;
use chasm_ir::Instruction;
