use std::collections::HashMap;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenType {
    // Parenthesis
    LEFT_PAREN,
    RIGHT_PAREN,
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
    COLON,

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
    NUMBER,

    // Boolean values
    TRUE,
    FALSE,
    NONE,

    // Conditional Statements
    IF,
    ELSE,
    THEN,

    // Functions
    FUNCTION,

    // Structs
    STRUCT,

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

    // Terminating Symbols
    NEW_LINE,

    EOF,
}

pub fn get_keywords() -> HashMap<String, TokenType> {
    HashMap::from([
        (String::from("and"), TokenType::AND),
        (String::from("or"), TokenType::OR),
        (String::from("if"), TokenType::IF),
        (String::from("else"), TokenType::ELSE),
        (String::from("then"), TokenType::THEN),
        (String::from("function"), TokenType::FUNCTION),
        (String::from("struct"), TokenType::STRUCT),
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
