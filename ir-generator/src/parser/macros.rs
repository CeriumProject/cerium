#[macro_export]
macro_rules! expect_token {
    ($lexer:expr, $pattern:pat, $result:expr) => {
        match $lexer.next() {
            Some(Ok($pattern)) => Ok($result),
            Some(Ok((range, token))) => {
                #[cfg(debug_assertions)]
                println!("Error caused in line {}", line!());
                Err(crate::error::CompilerError::UnexpectedTokenError(
                    crate::error::UnexpectedTokenError { range, token },
                ))
            }
            Some(Err(e)) => Err(e),
            None => Err(crate::error::CompilerError::UnexpectedEof(
                crate::error::UnexpectedEof,
            )),
        }
    };
    ($lexer:expr, $token:pat) => {
        expect_token!($lexer, (a, $token), a)
    };
}

#[macro_export]
macro_rules! next_matches {
    ($lexer:expr, $pattern:pat, $result:expr) => {
        match $lexer.peek() {
            Some(Ok($pattern)) => match $lexer.next() {
                Some(Ok($pattern)) => Some($result),
                _ => unreachable!(),
            },
            _ => None,
        }
    };
    ($lexer:expr, $pattern:pat) => {
        $lexer
            .next_if(|token| matches!(token, Ok((_, $pattern))))
            .is_some()
    };
}

#[macro_export]
macro_rules! join_expression {
    ($expr:ident { $lhs:expr, $rhs:expr }) => {{
        let lhs = $lhs;
        let rhs = $rhs;
        let range = *lhs.range.start()..=*rhs.range.end();
        let expression = Expression::$expr(Box::new($expr { lhs, rhs }));
        (range, expression)
    }};
}
