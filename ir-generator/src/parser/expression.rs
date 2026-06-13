use crate::ast::compiler_macro::CompilerMacro;
use crate::ast::dereference::Dereference;
use crate::ast::generic_operation::GenericOperator;
use crate::ast::reference::Reference;
use crate::ast::{
    ArrayIndexation, Assignment, ConstantValue, Declaration, ForDownTo, GenericOperation,
    Invocation, Loop, Scope, TypeAlias, Variable,
};
use crate::ast::{Expression, TypeCast};
use crate::error::{CompilerError, CompilerResult, UnexpectedEof, UnexpectedTokenError};
use crate::parser::{Parser, join_ranges};
use crate::ranged::Ranged;
use crate::token::Token;
use crate::{expect_token, next_matches};

impl Parser<'_> {
    pub(super) fn parse_expression(&mut self) -> CompilerResult<Ranged<Expression>> {
        let dest = self.parse_addition_subtraction()?;

        if next_matches!(self.lexer, Token::Assign) {
            let source = self.parse_expression()?;
            let range = join_ranges(&dest, &source);
            let expression = Expression::Assignment(Box::new(Assignment { dest, source }));
            Ok((range, expression))
        } else {
            Ok(dest)
        }
    }

    fn parse_addition_subtraction(&mut self) -> CompilerResult<Ranged<Expression>> {
        let mut lhs = self.parse_multiplication_division()?;
        loop {
            if let Some(op_range) = next_matches!(self.lexer, (op_range, Token::Plus), op_range) {
                let rhs = self.parse_multiplication_division()?;
                let operator = (op_range, GenericOperator::Add);
                lhs = (
                    join_ranges(&lhs, &rhs),
                    GenericOperation { lhs, rhs, operator }.into(),
                );
            } else if let Some(op_range) =
                next_matches!(self.lexer, (op_range, Token::Minus), op_range)
            {
                let rhs = self.parse_multiplication_division()?;
                let operator = (op_range, GenericOperator::Sub);
                lhs = (
                    join_ranges(&lhs, &rhs),
                    GenericOperation { lhs, rhs, operator }.into(),
                );
            } else {
                break Ok(lhs);
            }
        }
    }

    fn parse_multiplication_division(&mut self) -> CompilerResult<Ranged<Expression>> {
        let mut lhs = self.parse_typing_operation()?;
        loop {
            if let Some(op_range) = next_matches!(self.lexer, (op_range, Token::Asterisk), op_range)
            {
                let rhs = self.parse_typing_operation()?;
                let operator = (op_range, GenericOperator::Mul);
                lhs = (
                    join_ranges(&lhs, &rhs),
                    GenericOperation { lhs, rhs, operator }.into(),
                );
            } else if let Some(op_range) =
                next_matches!(self.lexer, (op_range, Token::Slash), op_range)
            {
                let rhs = self.parse_typing_operation()?;
                let operator = (op_range, GenericOperator::Div);
                lhs = (
                    join_ranges(&lhs, &rhs),
                    GenericOperation { lhs, rhs, operator }.into(),
                );
            } else {
                break Ok(lhs);
            }
        }
    }

    fn parse_typing_operation(&mut self) -> CompilerResult<Ranged<Expression>> {
        let mut value = self.parse_prefix_operation()?;
        loop {
            if next_matches!(self.lexer, Token::As) {
                let r#type = self.parse_type()?;
                value = (
                    join_ranges(&value, &r#type),
                    Expression::TypeCast(Box::new(TypeCast { value, r#type })),
                );
            } else if next_matches!(self.lexer, Token::Alias) {
                let r#type = self.parse_type()?;
                value = (
                    join_ranges(&value, &r#type),
                    Expression::TypeAlias(Box::new(TypeAlias { value, r#type })),
                );
            } else {
                break Ok(value);
            }
        }
    }

    fn parse_prefix_operation(&mut self) -> CompilerResult<Ranged<Expression>> {
        match self
            .lexer
            .peek()
            .ok_or(CompilerError::UnexpectedEof(UnexpectedEof))?
            .clone()
        {
            Ok((prefix_range, Token::Ampersand)) => {
                let _ = self.lexer.next();
                let inner = self.parse_prefix_operation()?;
                let range = *prefix_range.start()..=*inner.0.end();
                Ok((range, Expression::Reference(Box::new(Reference { inner }))))
            }
            Ok((prefix_range, Token::Asterisk)) => {
                let _ = self.lexer.next();
                let inner = self.parse_prefix_operation()?;
                let range = *prefix_range.start()..=*inner.0.end();
                Ok((
                    range,
                    Expression::Dereference(Box::new(Dereference { inner })),
                ))
            }
            Ok((_, Token::Plus)) => {
                todo!()
            }
            Ok((_, Token::Minus)) => {
                todo!()
            }
            Ok(_) => self.parse_operand(),
            Err(err) => Err(err.clone()),
        }
    }

    fn parse_operand(&mut self) -> CompilerResult<Ranged<Expression>> {
        let mut result = match self
            .lexer
            .peek()
            .ok_or(CompilerError::UnexpectedEof(UnexpectedEof))?
        {
            Ok((_, Token::LBrace)) => self.parse_scope(),
            Ok((_, Token::LParen)) => self.parse_parens(),
            Ok((_, Token::Let)) => self.parse_let(),
            Ok((_, Token::For)) => self.parse_for(),
            Ok((_, Token::Loop)) => self.parse_loop(),
            Ok((_, Token::Ident(_))) => self.parse_variable(),
            Ok((_, Token::Number(_))) => self.parse_constant_value(),
            Ok((range, token)) => Err(CompilerError::UnexpectedTokenError(UnexpectedTokenError {
                range: range.clone(),
                token: token.clone(),
            })),
            Err(err) => Err(err.clone()),
        }?;

        loop {
            if next_matches!(self.lexer, Token::LParen) {
                let mut parameters = Vec::new();
                while !matches!(self.lexer.peek(), Some(Ok((_, Token::RParen)))) {
                    parameters.push(self.parse_expression()?);

                    if !next_matches!(self.lexer, Token::Comma) {
                        break;
                    }
                }
                let end = expect_token!(self.lexer, (range, Token::RParen), *range.end())?;
                result = (
                    *result.0.start()..=end,
                    Expression::Invocation(Box::new(Invocation {
                        function: result,
                        parameters,
                    })),
                );
            } else if next_matches!(self.lexer, Token::LBracket) {
                let index = self.parse_expression()?;
                let end = expect_token!(self.lexer, (range, Token::RBracket), *range.end())?;
                result = (
                    *result.0.start()..=end,
                    Expression::ArrayIndexation(Box::new(ArrayIndexation {
                        array: result,
                        index,
                    })),
                )
            } else {
                break Ok(result);
            }
        }
    }

    pub(super) fn parse_scope(&mut self) -> CompilerResult<Ranged<Expression>> {
        let start = expect_token!(self.lexer, (range, Token::LBrace), *range.start())?;
        let mut statements = Vec::new();
        let result = loop {
            if matches!(self.lexer.peek(), Some(Ok((_, Token::RBrace)))) {
                break None;
            }

            let candidate = self.parse_expression()?;

            if next_matches!(self.lexer, Token::Semicolon) {
                statements.push(candidate);
            } else {
                break Some(candidate);
            }
        };

        let end = expect_token!(self.lexer, (range, Token::RBrace), *range.end())?;

        Ok((
            start..=end,
            Expression::Scope(Box::new(Scope { statements, result })),
        ))
    }

    fn parse_parens(&mut self) -> CompilerResult<Ranged<Expression>> {
        expect_token!(self.lexer, Token::LParen)?;
        let result = self.parse_expression();
        expect_token!(self.lexer, Token::RParen)?;
        result
    }

    fn parse_let(&mut self) -> CompilerResult<Ranged<Expression>> {
        let start = expect_token!(self.lexer, (range, Token::Let), *range.start())?;
        let name = self.parse_qualifier()?;
        expect_token!(self.lexer, Token::Assign)?;
        let value = self.parse_expression()?;
        if next_matches!(self.lexer, Token::In) {
            let body = self.parse_expression()?;
            let range = start..=*body.0.end();
            Ok((
                range.clone(),
                Expression::Scope(Box::new(Scope {
                    statements: vec![(
                        range,
                        Expression::Declaration(Box::new(Declaration { name, value })),
                    )],
                    result: Some(body),
                })),
            ))
        } else {
            let range = start..=*value.0.end();
            Ok((
                range,
                Expression::Declaration(Box::new(Declaration { name, value })),
            ))
        }
    }

    fn parse_for(&mut self) -> CompilerResult<Ranged<Expression>> {
        let start = *expect_token!(self.lexer, Token::For)?.start();
        let counter = self.parse_qualifier()?;
        expect_token!(self.lexer, Token::Downto)?;
        let limit = self.parse_expression()?;
        let body = self.parse_scope()?;
        let range = start..=*body.0.end();
        Ok((
            range,
            Expression::ForDownTo(Box::new(ForDownTo {
                counter,
                limit,
                body,
            })),
        ))
    }

    fn parse_loop(&mut self) -> CompilerResult<Ranged<Expression>> {
        let loop_range = expect_token!(self.lexer, Token::Loop)?;
        let body = self.parse_scope()?;
        let range = *loop_range.start()..=*body.0.end();
        Ok((range, Expression::Loop(Box::new(Loop { body }))))
    }

    fn parse_variable(&mut self) -> CompilerResult<Ranged<Expression>> {
        let name = self.parse_qualifier()?;
        if next_matches!(self.lexer, Token::Bang) {
            expect_token!(self.lexer, Token::LParen)?;

            let mut expressions = Vec::new();
            while !matches!(self.lexer.peek(), Some(Ok((_, Token::RParen)))) {
                expressions.push(self.parse_expression()?);

                if !next_matches!(self.lexer, Token::Comma) {
                    break;
                }
            }
            let end = expect_token!(self.lexer, (range, Token::RParen), range)?;

            let range = join_ranges(&name, &(end, ()));
            Ok((
                range,
                Expression::CompilerMacro(Box::new(CompilerMacro { name, expressions })),
            ))
        } else {
            Ok((
                name.0.clone(),
                Expression::Variable(Box::new(Variable { name })),
            ))
        }
    }

    fn parse_constant_value(&mut self) -> CompilerResult<Ranged<Expression>> {
        match self
            .lexer
            .next()
            .ok_or(CompilerError::UnexpectedEof(UnexpectedEof))??
        {
            (range, Token::Number(number)) => Ok((
                range.clone(),
                Expression::Constant(Box::new(ConstantValue {
                    value: (range, number),
                })),
            )),
            (range, token) => Err(CompilerError::UnexpectedTokenError(UnexpectedTokenError {
                token,
                range,
            })),
        }
    }
}
