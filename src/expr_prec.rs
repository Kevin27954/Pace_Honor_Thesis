use crate::scanner::TokenType;

pub struct Precdence {
    pub none: u8,
    pub assignment: u8,
    pub or: u8,
    pub and: u8,
    pub equality: u8,
    pub comparison: u8,
    pub term: u8,
    pub factor: u8,
    pub unary: u8,
    pub call: u8,
    pub instance: u8,
}

pub const PRECEDENCE: Precdence = Precdence {
    none: 0,
    assignment: 1,
    or: 2,
    and: 3,
    equality: 4,
    comparison: 5,
    term: 6,
    factor: 7,
    unary: 8,
    call: 9,
    instance: 9,
};

pub fn get_precedence(token_type: TokenType) -> u8 {
    match token_type {
        TokenType::Minus | TokenType::Plus => PRECEDENCE.term,
        TokenType::Slash | TokenType::Star => PRECEDENCE.factor,
        TokenType::BangEqual | TokenType::EqualEqual => PRECEDENCE.equality,
        TokenType::Greater | TokenType::GreaterEqual | TokenType::Less | TokenType::LessEqual => {
            PRECEDENCE.comparison
        }
        TokenType::And => PRECEDENCE.and,
        TokenType::Or => PRECEDENCE.or,
        TokenType::LeftParen | TokenType::Dot => PRECEDENCE.call,
        TokenType::LeftBrace => PRECEDENCE.instance,
        _ => PRECEDENCE.none,
    }
}

#[derive(Debug)]
pub enum ParseFn {
    Unary,
    Grouping,
    Number,
    Binary,

    And,
    Or,

    Variable,
    Literal,
    String,

    Call,
    Dot,
    Instance,
}

pub struct ParseRule {
    pub prefix_rule: Option<ParseFn>,
    pub infix_rule: Option<ParseFn>,
    pub precedence: u8,
}

pub fn get_parse_rule(token_type: TokenType) -> ParseRule {
    match token_type {
        // Parenthesis
        TokenType::LeftParen => ParseRule {
            prefix_rule: Some(ParseFn::Grouping),
            infix_rule: Some(ParseFn::Call),
            precedence: get_precedence(token_type),
        },
        TokenType::RightParen => ParseRule {
            prefix_rule: None,
            infix_rule: None,
            precedence: get_precedence(token_type),
        },
        TokenType::LeftBrace => ParseRule {
            prefix_rule: None,
            infix_rule: Some(ParseFn::Instance),
            precedence: get_precedence(token_type),
        },
        TokenType::RightBrace => ParseRule {
            prefix_rule: None,
            infix_rule: None,
            precedence: get_precedence(token_type),
        },

        // Symbols
        TokenType::Comma => ParseRule {
            prefix_rule: None,
            infix_rule: None,
            precedence: get_precedence(token_type),
        },
        TokenType::Dot => ParseRule {
            prefix_rule: None,
            infix_rule: Some(ParseFn::Dot),
            precedence: get_precedence(token_type),
        },
        TokenType::Minus => ParseRule {
            prefix_rule: Some(ParseFn::Unary),
            infix_rule: Some(ParseFn::Binary),
            precedence: get_precedence(token_type),
        },
        TokenType::Plus => ParseRule {
            prefix_rule: None,
            infix_rule: Some(ParseFn::Binary),
            precedence: get_precedence(token_type),
        },
        TokenType::Slash => ParseRule {
            prefix_rule: None,
            infix_rule: Some(ParseFn::Binary),
            precedence: get_precedence(token_type),
        },
        TokenType::Star => ParseRule {
            prefix_rule: None,
            infix_rule: Some(ParseFn::Binary),
            precedence: get_precedence(token_type),
        },
        TokenType::Semicolon => ParseRule {
            prefix_rule: None,
            infix_rule: None,
            precedence: get_precedence(token_type),
        },
        TokenType::Colon => ParseRule {
            prefix_rule: None,
            infix_rule: None,
            precedence: get_precedence(token_type),
        },

        // Equality Operator
        TokenType::Bang => ParseRule {
            prefix_rule: Some(ParseFn::Unary),
            infix_rule: None,
            precedence: get_precedence(token_type),
        },
        TokenType::BangEqual => ParseRule {
            prefix_rule: None,
            infix_rule: Some(ParseFn::Binary),
            precedence: get_precedence(token_type),
        },
        TokenType::Equal => ParseRule {
            prefix_rule: None,
            infix_rule: None,
            precedence: get_precedence(token_type),
        },
        TokenType::EqualEqual => ParseRule {
            prefix_rule: None,
            infix_rule: Some(ParseFn::Binary),
            precedence: get_precedence(token_type),
        },

        // Equality Operators
        TokenType::Greater => ParseRule {
            prefix_rule: None,
            infix_rule: Some(ParseFn::Binary),
            precedence: get_precedence(token_type),
        },
        TokenType::GreaterEqual => ParseRule {
            prefix_rule: None,
            infix_rule: Some(ParseFn::Binary),
            precedence: get_precedence(token_type),
        },
        TokenType::Less => ParseRule {
            prefix_rule: None,
            infix_rule: Some(ParseFn::Binary),
            precedence: get_precedence(token_type),
        },
        TokenType::LessEqual => ParseRule {
            prefix_rule: None,
            infix_rule: Some(ParseFn::Binary),
            precedence: get_precedence(token_type),
        },

        // Comparison
        TokenType::And => ParseRule {
            prefix_rule: None,
            infix_rule: Some(ParseFn::And),
            precedence: get_precedence(token_type),
        },
        TokenType::Or => ParseRule {
            prefix_rule: None,
            infix_rule: Some(ParseFn::Or),
            precedence: get_precedence(token_type),
        },

        // Litearls
        TokenType::Identifier => ParseRule {
            prefix_rule: Some(ParseFn::Variable),
            infix_rule: None,
            precedence: get_precedence(token_type),
        },
        TokenType::String => ParseRule {
            prefix_rule: Some(ParseFn::String),
            infix_rule: None,
            precedence: get_precedence(token_type),
        },
        TokenType::Number => ParseRule {
            prefix_rule: Some(ParseFn::Number),
            infix_rule: None,
            precedence: get_precedence(token_type),
        },

        // Boolean values
        TokenType::True => ParseRule {
            prefix_rule: Some(ParseFn::Literal),
            infix_rule: None,
            precedence: get_precedence(token_type),
        },
        TokenType::False => ParseRule {
            prefix_rule: Some(ParseFn::Literal),
            infix_rule: None,
            precedence: get_precedence(token_type),
        },
        TokenType::None => ParseRule {
            prefix_rule: Some(ParseFn::Literal),
            infix_rule: None,
            precedence: get_precedence(token_type),
        },

        // Conditional Statment
        TokenType::If => ParseRule {
            prefix_rule: None,
            infix_rule: None,
            precedence: get_precedence(token_type),
        },
        TokenType::Else => ParseRule {
            prefix_rule: None,
            infix_rule: None,
            precedence: get_precedence(token_type),
        },
        TokenType::Then => ParseRule {
            prefix_rule: None,
            infix_rule: None,
            precedence: get_precedence(token_type),
        },

        // Function
        TokenType::Function => ParseRule {
            prefix_rule: None,
            infix_rule: None,
            precedence: get_precedence(token_type),
        },

        // Struct
        TokenType::Struct => ParseRule {
            prefix_rule: None,
            infix_rule: None,
            precedence: get_precedence(token_type),
        },

        // Loops
        TokenType::For => ParseRule {
            prefix_rule: None,
            infix_rule: None,
            precedence: get_precedence(token_type),
        },
        TokenType::While => ParseRule {
            prefix_rule: None,
            infix_rule: None,
            precedence: get_precedence(token_type),
        },

        // Keywords
        TokenType::Do => ParseRule {
            prefix_rule: None,
            infix_rule: None,
            precedence: get_precedence(token_type),
        },
        TokenType::End => ParseRule {
            prefix_rule: None,
            infix_rule: None,
            precedence: get_precedence(token_type),
        },
        TokenType::Let => ParseRule {
            prefix_rule: None,
            infix_rule: None,
            precedence: get_precedence(token_type),
        },
        TokenType::Return => ParseRule {
            prefix_rule: None,
            infix_rule: None,
            precedence: get_precedence(token_type),
        },

        TokenType::Comment => ParseRule {
            prefix_rule: None,
            infix_rule: None,
            precedence: get_precedence(token_type),
        },
        TokenType::NewLine => ParseRule {
            prefix_rule: None,
            infix_rule: None,
            precedence: get_precedence(token_type),
        },
        TokenType::Error => ParseRule {
            prefix_rule: None,
            infix_rule: None,
            precedence: get_precedence(token_type),
        },
        TokenType::EOF => ParseRule {
            prefix_rule: None,
            infix_rule: None,
            precedence: get_precedence(token_type),
        },
    }
}
