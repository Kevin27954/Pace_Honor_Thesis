#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum TokenType {
    // Parenthesis
    LEFT_PAREN,
    RIGHT_PAREN,
    // There shouldn't be braces for functions.
    LEFT_BRACE,
    RIGHT_BRACE,

    // Symbols
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SLASH,
    STAR,
    SEMICOLON,

    // Equality Operator
    // !
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Comparison Operators
    AND,
    OR,

    // Literals
    IDENTIFIERS,
    STRING,
    NUMBERS,

    // Boolean values
    TRUE,
    FALSE,

    // Conditional Statements
    IF,
    ELSE,

    // Functions
    FUNCTION,

    // Loops
    FOR,
    WHILE,
    DO,

    // Ending Keyword for functions, loops, etc
    END,

    // Variable declaration
    LET,

    // Return statement
    RETURN,

    NEW_LINE,
    WHITE_SPACE,
    EOF,
}
