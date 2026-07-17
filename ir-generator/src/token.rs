#[rustfmt::skip]
#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    // wrappers
    Ident(String), Number(String), String(String), Character(char),
    // constants
    True, False, Nullptr,
    // parentheses etc.
    LParen, RParen, LBracket, RBracket, LBrace, RBrace,
    // special characters
    Semicolon, Colon, Comma, Arrow, Scope, Dot,
    // top-level definitions
    Fn, Const, Struct,
    // declarations
    Let, In,
    // loops
    For, Downto, Loop, // Break,
    // conditions
    If, Else,
    // generic operators
    Assign, Plus, Minus, Asterisk, Slash,
    // comparison operators
    // Equals, NotEquals, GreaterThan, GreaterThanEquals, LessThan, LessThanEquals,
    // boolean operators
    // And, Or, Xor,
    // bitwise operators
    Pipe, RShift, LShift,
    // multipurpose operators
    Ampersand, Bang, Circumflex, LessThan, GreaterThan,
    // types
    F16, I16, U16, Bool, Char, Any, Undefined,
    // conversion
    As, Alias,
    // inspection
    Sizeof,
}
