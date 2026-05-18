#[rustfmt::skip]
#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    // wrappers
    Ident(String), Number(String),
    // parentheses etc.
    LParen, RParen, /* LBracket, RBracket, */ LBrace, RBrace,
    // special characters
    Semicolon, Colon, Comma, Arrow,
    // top-level definitions
    Fn, // Const, Struct,
    // declarations
    Let, In,
    // loops
    For, Downto, Loop, // Break,
    // generic operators
    Assign, Plus, Minus, Asterisk, Slash,
    // comparison operators
    // Equals, NotEquals, GreaterThan, GreaterThanEquals, LessThan, LessThanEquals,
    // boolean operators
    // Bang, And, Or, Xor,
    // Types
    F16, I16, U16, // Bool,
    // conversion
    As, Alias,
}