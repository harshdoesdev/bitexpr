use thiserror::Error;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Identifier(String),
    Operator(Operator),
    OpenParenthesis,
    CloseParenthesis,
    StringLiteral(String),
    Function(String),
    Comma,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Identifier(s) => write!(f, "{}", s),
            Token::Operator(op) => write!(f, "{:?}", op),
            Token::OpenParenthesis => write!(f, "("),
            Token::CloseParenthesis => write!(f, ")"),
            Token::StringLiteral(s) => write!(f, "\"{}\"", s),
            Token::Function(s) => write!(f, "{}", s),
            Token::Comma => write!(f, ","),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Power,
    Equal,
    NotEqual,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
    And,
    Or,
}

#[derive(Debug, PartialEq, Error)]
pub enum TokenizerError {
    #[error("Unexpected character '{0}'")]
    UnexpectedChar(char),
    #[error("Unexpected end of string after escape character")]
    UnexpectedEndOfString,
}

// todo: rewrite using a state machine
pub fn tokenize(expression: &str) -> Result<Vec<Token>, TokenizerError> {
    let mut tokens = Vec::new();
    let mut chars = expression.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            ' ' => {
                // Ignore spaces
                chars.next();
            }
            '(' => {
                tokens.push(Token::OpenParenthesis);
                chars.next();
            }
            ')' => {
                tokens.push(Token::CloseParenthesis);
                chars.next();
            }
            ',' => {
                tokens.push(Token::Comma);
                chars.next();
            }
            '+' => {
                tokens.push(Token::Operator(Operator::Plus));
                chars.next();
            }
            '-' => {
                tokens.push(Token::Operator(Operator::Minus));
                chars.next();
            }
            '*' => {
                tokens.push(Token::Operator(Operator::Multiply));
                chars.next();
            }
            '/' => {
                tokens.push(Token::Operator(Operator::Divide));
                chars.next();
            }
            '%' => {
                tokens.push(Token::Operator(Operator::Modulo));
                chars.next();
            }
            '^' => {
                tokens.push(Token::Operator(Operator::Power));
                chars.next();
            }
            '=' => {
                chars.next();
                if let Some(&'=') = chars.peek() {
                    tokens.push(Token::Operator(Operator::Equal));
                    chars.next();
                } else {
                    return Err(TokenizerError::UnexpectedChar('='));
                }
            }
            '!' => {
                chars.next();
                if let Some(&'=') = chars.peek() {
                    tokens.push(Token::Operator(Operator::NotEqual));
                    chars.next();
                } else {
                    return Err(TokenizerError::UnexpectedChar('!'));
                }
            }
            '<' => {
                chars.next();
                if let Some(&'=') = chars.peek() {
                    tokens.push(Token::Operator(Operator::LessEqual));
                    chars.next();
                } else {
                    tokens.push(Token::Operator(Operator::Less));
                }
            }
            '>' => {
                chars.next();
                if let Some(&'=') = chars.peek() {
                    tokens.push(Token::Operator(Operator::GreaterEqual));
                    chars.next();
                } else {
                    tokens.push(Token::Operator(Operator::Greater));
                }
            }
            '&' => {
                chars.next();
                if let Some(&'&') = chars.peek() {
                    tokens.push(Token::Operator(Operator::And));
                    chars.next();
                } else {
                    return Err(TokenizerError::UnexpectedChar('&'));
                }
            }
            '|' => {
                chars.next();
                if let Some(&'|') = chars.peek() {
                    tokens.push(Token::Operator(Operator::Or));
                    chars.next();
                } else {
                    return Err(TokenizerError::UnexpectedChar('|'));
                }
            }
            '"' => {
                chars.next();
                let mut string_literal = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch == '\\' {
                        chars.next();
                        if let Some(escaped_char) = chars.next() {
                            match escaped_char {
                                'n' => string_literal.push('\n'),
                                't' => string_literal.push('\t'),
                                '"' => string_literal.push('"'),
                                _ => string_literal.push(escaped_char),
                            }
                        } else {
                            return Err(TokenizerError::UnexpectedEndOfString);
                        }
                    } else if ch == '"' {
                        chars.next();
                        break;
                    } else {
                        string_literal.push(ch);
                        chars.next();
                    }
                }
                tokens.push(Token::StringLiteral(string_literal));
            }
            _ => {
                let mut identifier = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_alphanumeric() || ch == '_' || ch == '.' {
                        identifier.push(ch);
                        chars.next();
                    } else if ch == '\\' {
                        chars.next();
                        if let Some(escaped_char) = chars.next() {
                            identifier.push(escaped_char);
                        } else {
                            return Err(TokenizerError::UnexpectedEndOfString);
                        }
                    } else {
                        break;
                    }
                }
                if let Some(&'(') = chars.peek() {
                    tokens.push(Token::Function(identifier));
                } else {
                    tokens.push(Token::Identifier(identifier));
                }
            }
        }
    }

    Ok(tokens)
}
