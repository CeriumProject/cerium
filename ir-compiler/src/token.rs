#[rustfmt::skip]
#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    // wrappers
    Ident(String), Number(String),
    // constants
    // Nullptr,
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
    // multipurpose operators
    Ampersand,
    // types
    F16, I16, U16, // Any, Bool,
    // conversion
    As, Alias,
}
