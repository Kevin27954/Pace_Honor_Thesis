use std::collections::HashMap;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    // Parenthesis
    LEFT_PAREN,
    RIGHT_PAREN,
    // There shouldn't be braces for functions.
    // LEFT_BRACE,
    // RIGHT_BRACE,

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
    IDENTIFIER,
    STRING,
    NUMBERS,

    // Boolean values
    TRUE,
    FALSE,
    NONE,

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

    // Comments
    COMMENT,

    EOF,
}

pub fn get_keywords() -> HashMap<String, TokenType> {
    HashMap::from([
        (String::from("and"), TokenType::AND),
        (String::from("or"), TokenType::OR),
        (String::from("if"), TokenType::IF),
        (String::from("else"), TokenType::ELSE),
        (String::from("function"), TokenType::FUNCTION),
        (String::from("for"), TokenType::FOR),
        (String::from("while"), TokenType::WHILE),
        (String::from("do"), TokenType::DO),
        (String::from("end"), TokenType::END),
        (String::from("let"), TokenType::LET),
        (String::from("return"), TokenType::RETURN),
        (String::from("true"), TokenType::TRUE),
        (String::from("false"), TokenType::FALSE),
        (String::from("none"), TokenType::NONE),
    ])
}
