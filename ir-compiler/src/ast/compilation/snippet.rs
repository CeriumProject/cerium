#[macro_export]
macro_rules! snippet {
    () => (
        Vec::new()
    );
    ($x:expr $(,)?) => (
        $crate::ast::compilation::snippet::SnippetElement::into_vec($x)
    );
    ($e:expr $(, $es:expr)+ $(,)?) => {
        {
            let mut result = crate::ast::compilation::snippet::SnippetElement::into_vec($e);
            result.extend(snippet![$($es),+]);
            result
        }
    };
}

use chasm_ir::Instruction;
pub use snippet;

pub(crate) trait SnippetElement {
    fn into_vec(self) -> Vec<Instruction>;
}

impl SnippetElement for Instruction {
    fn into_vec(self) -> Vec<Instruction> {
        vec![self]
    }
}

impl SnippetElement for Vec<Instruction> {
    fn into_vec(self) -> Vec<Instruction> {
        self
    }
}
