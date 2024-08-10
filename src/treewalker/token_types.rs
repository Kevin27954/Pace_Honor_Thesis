#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum TokenType {
    // Parenthesis
    LEFT_PAREN,
    RIGHT_PAREN,
    // There shouldn't be braces for functions.
    LEFT_BRACE,
    RIGHT_BRACE,

    COMMA,
    DOT,
    MINUS,
    PLUS,
    SLASH,
    STAR,
    SEMICOLON,

    // !
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    STRING,
    NUMBERS,

    TRUE,
    FALSE,

    AND,
    OR,

    IF,
    ELSE,

    FUNCTION,
    FOR,
    WHILE,
    DO,

    // Ending Keyword for functions, loops, etc
    END,

    LET,
    RETURN,

    EOF,
}
