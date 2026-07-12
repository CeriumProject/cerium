use crate::ast::Expression;

impl Expression {
    fn iter_rec(&self) -> RecursiveExpressionIterator<'_> {
        RecursiveExpressionIterator {
            recursion: vec![self.iter()],
        }
    }

    fn iter(&self) -> Box<dyn Iterator<Item = &'_ Expression>> {
        todo!()
    }
}

struct RecursiveExpressionIterator<'a> {
    // view: ExpressionView<'a>,
    recursion: Vec<Box<dyn Iterator<Item = &'a Expression>>>,
}

/*
struct ExpressionView<'a> {
    outer: Option<Box<ExpressionView<'a>>>,
    iter: Box<dyn Iterator<Item=&'a Expression>>,
}*/

impl<'a> Iterator for RecursiveExpressionIterator<'a> {
    type Item = &'a Expression;

    fn next(&mut self) -> Option<&'a Expression> {
        match self.recursion.last_mut()?.next() {
            None => {
                self.recursion.pop();
                self.next()
            }
            Some(_e) => todo!(),
        }
    }
}
