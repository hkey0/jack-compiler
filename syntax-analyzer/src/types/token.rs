use super::constants::{VALID_KEYWORDS, VALID_SYMBOLS};
use super::utils::is_number;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    Keyword,
    Symbol,
    IntegerConstant,
    StringConstant,
    Identifier,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

impl Token {
    pub fn new(buffer: String) -> Self {
        if VALID_SYMBOLS.contains(&buffer.as_str()) {
            return Self {
                token_type: TokenType::Symbol,
                value: buffer,
            };
        }

        if VALID_KEYWORDS.contains(&buffer.as_str()) {
            return Self {
                token_type: TokenType::Keyword,
                value: buffer,
            };
        }

        if is_number(buffer.as_str().get(0..1).unwrap_or("")) {
            return Self {
                token_type: TokenType::IntegerConstant,
                value: buffer,
            };
        }

        if buffer.contains('"') {
            return Self {
                token_type: TokenType::StringConstant,
                value: buffer.replace('"', ""),
            };
        }

        Self {
            token_type: TokenType::Identifier,
            value: buffer,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_valid_symbols() {
        let token = Token::new("{".to_string());

        assert_eq!(token.value, "{".to_string());
        assert_eq!(token.token_type, TokenType::Symbol);
    }

    #[test]
    fn can_parse_valid_keywords() {
        let token = Token::new("class".to_string());

        assert_eq!(token.value, "class".to_string());
        assert_eq!(token.token_type, TokenType::Keyword);
    }

    #[test]
    fn can_parse_integer_constants() {
        let token = Token::new("0123".to_string());

        assert_eq!(token.value, "0123".to_string());
        assert_eq!(token.token_type, TokenType::IntegerConstant);
    }

    #[test]
    fn can_parse_valid_string_constants() {
        let token = Token::new("\"test\"".to_string());

        assert_eq!(token.value, "test".to_string());
        assert_eq!(token.token_type, TokenType::StringConstant);
    }
}
