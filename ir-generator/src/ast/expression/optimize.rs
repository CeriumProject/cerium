use crate::ast::Expression;
use crate::ranged::Ranged;

pub trait OptimizeExpression {
    fn optimize(self) -> Expression;
}

pub trait OptimizeRangedExpression {
    fn optimize(self) -> Ranged<Expression>;
}

impl<T: OptimizeExpression> OptimizeRangedExpression for Ranged<T> {
    fn optimize(self) -> Ranged<Expression> {
        (self.0, self.1.optimize())
    }
}
