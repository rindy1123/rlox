use crate::lang_error;
use crate::scanner::token::{LiteralType, Token, TokenType};
use substring::Substring;

pub struct Scanner {
    source: String,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: u32,
}

const AND: &str = "and";
const CLASS: &str = "class";
const ELSE: &str = "else";
const FALSE: &str = "false";
const FOR: &str = "for";
const FUN: &str = "fun";
const IF: &str = "if";
const NIL: &str = "nil";
const OR: &str = "or";
const PRINT: &str = "print";
const RETURN: &str = "return";
const SUPER: &str = "super";
const THIS: &str = "this";
const TRUE: &str = "true";
const VAR: &str = "var";
const WHILE: &str = "while";

impl Default for Scanner {
    fn default() -> Self {
        Scanner {
            source: String::new(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            ..Default::default()
        }
    }

    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        let eof_token = Token::new(
            TokenType::EOF,
            String::from(""),
            LiteralType::Non,
            self.line,
        );
        self.tokens.push(eof_token);
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token_without_value(TokenType::LeftParen),
            ')' => self.add_token_without_value(TokenType::RightParen),
            '{' => self.add_token_without_value(TokenType::LeftBrace),
            '}' => self.add_token_without_value(TokenType::RightBrace),
            ',' => self.add_token_without_value(TokenType::Comma),
            '.' => self.add_token_without_value(TokenType::Dot),
            '-' => self.add_token_without_value(TokenType::Minus),
            '+' => self.add_token_without_value(TokenType::Plus),
            ';' => self.add_token_without_value(TokenType::Semicolon),
            '*' => self.add_token_without_value(TokenType::Star),
            ' ' | '\r' | '\t' => (),
            '"' => self.string(),
            '\n' => {
                self.line += 1;
            }
            '!' => {
                let token_type = if self.match_char('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token_without_value(token_type)
            }
            '=' => {
                let token_type = if self.match_char('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token_without_value(token_type)
            }
            '<' => {
                let token_type = if self.match_char('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token_without_value(token_type)
            }
            '>' => {
                let token_type = if self.match_char('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token_without_value(token_type)
            }
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token_without_value(TokenType::Slash);
                }
            }
            _ => {
                if self.is_digit(c) {
                    self.number();
                } else if self.is_alpha(c) {
                    self.identifier();
                } else {
                    lang_error::error(self.line, String::from("Unexpected character"));
                }
            }
        };
    }

    fn match_char(&mut self, c: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.nth_char(self.current) != c {
            return false;
        }
        self.current += 1;
        true
    }

    fn advance(&mut self) -> char {
        let ret = self.nth_char(self.current);
        self.current += 1;
        ret
    }

    fn nth_char(&self, n: usize) -> char {
        self.source.chars().nth(n).unwrap()
    }

    fn add_token_without_value(&mut self, token_type: TokenType) {
        self.add_token(token_type, LiteralType::Non);
    }

    fn add_token(&mut self, token_type: TokenType, literal: LiteralType) {
        let lexeme = self.source.substring(self.start, self.current).to_string();
        let token = Token::new(token_type, lexeme, literal, self.line);
        self.tokens.push(token);
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.nth_char(self.current)
    }

    fn next_peek(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.nth_char(self.current + 1)
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            lang_error::error(self.line, String::from("Unterminated string."));
            return;
        }

        self.advance();
        let value = self
            .source
            .substring(self.start + 1, self.current - 1)
            .to_string();
        self.add_token(TokenType::LString, LiteralType::Str(value));
    }

    fn is_digit(&self, c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.next_peek()) {
            self.advance();
        }

        while self.is_digit(self.peek()) {
            self.advance();
        }
        let literal = match self
            .source
            .substring(self.start, self.current)
            .parse::<f64>()
        {
            Ok(num) => num,
            Err(_) => {
                lang_error::error(self.line, String::from("Not a number."));
                return;
            }
        };
        self.add_token(TokenType::Number, LiteralType::Num(literal))
    }

    fn is_alpha(&self, c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }

    fn identifier(&mut self) {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }
        let text = self.source.substring(self.start, self.current);
        let token_type = match text {
            AND => TokenType::And,
            CLASS => TokenType::Class,
            ELSE => TokenType::Else,
            FALSE => TokenType::False,
            FOR => TokenType::For,
            FUN => TokenType::Fun,
            IF => TokenType::If,
            NIL => TokenType::Nil,
            OR => TokenType::Or,
            PRINT => TokenType::Print,
            RETURN => TokenType::Return,
            SUPER => TokenType::Super,
            THIS => TokenType::This,
            TRUE => TokenType::True,
            VAR => TokenType::Var,
            WHILE => TokenType::While,
            _ => TokenType::Identifier,
        };
        self.add_token_without_value(token_type);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn create_scanner() -> Scanner {
        Scanner::new(String::from("()"))
    }

    #[test]
    fn test_scan_tokens() {
        let mut scanner1 = create_scanner();
        let expected_token1 =
            Token::new(TokenType::LeftParen, String::from("("), LiteralType::Non, 1);
        let expected_token2 = Token::new(
            TokenType::RightParen,
            String::from(")"),
            LiteralType::Non,
            1,
        );
        let expected_token3 = Token::new(TokenType::EOF, String::from(""), LiteralType::Non, 1);
        scanner1.scan_tokens();
        assert_eq!(scanner1.tokens[0], expected_token1);
        assert_eq!(scanner1.tokens[1], expected_token2);
        assert_eq!(scanner1.tokens[2], expected_token3);
        assert_eq!(scanner1.tokens.len(), 3);
    }

    #[test]
    fn test_scan_token() {
        let mut scanner1 = create_scanner();
        let expected_token =
            Token::new(TokenType::LeftParen, String::from("("), LiteralType::Non, 1);
        scanner1.scan_token();
        assert_eq!(scanner1.tokens[0], expected_token);
    }

    #[test]
    fn test_is_at_end() {
        let mut scanner1 = create_scanner();
        scanner1.source = String::from("123");
        scanner1.current = 3;
        assert!(scanner1.is_at_end())
    }

    #[test]
    fn test_match_char() {
        let mut scanner1 = create_scanner();
        scanner1.source = String::from("abc");
        // current indicates the char 'c'
        scanner1.current = 2;
        // when target_char doesn't match the current char
        let target_char1 = 'x';
        assert!(!scanner1.match_char(target_char1));
        // when target_char match the current char
        let target_char2 = 'c';
        assert!(scanner1.match_char(target_char2));
        // after scanner scanned throughout the source
        assert!(!scanner1.match_char(target_char2));
    }

    #[test]
    fn test_advance() {
        let mut scanner1 = create_scanner();
        assert_eq!(scanner1.advance(), '(');
    }

    #[test]
    fn test_nth_char() {
        let mut scanner1 = create_scanner();
        scanner1.source = String::from("012");
        assert_eq!(scanner1.nth_char(1), '1');
    }

    #[test]
    fn test_add_token() {
        let mut scanner1 = create_scanner();
        scanner1.source = String::from("012");
        scanner1.current = 2;
        scanner1.add_token(TokenType::LeftParen, LiteralType::Non);
        let expected_token = Token::new(
            TokenType::LeftParen,
            String::from("01"),
            LiteralType::Non,
            1,
        );
        assert_eq!(scanner1.tokens[0], expected_token);
    }

    #[test]
    fn test_peek() {
        let mut scanner1 = create_scanner();
        scanner1.source = String::from("012");
        scanner1.current = 2;
        assert_eq!(scanner1.peek(), '2');
        scanner1.current = 3;
        assert_eq!(scanner1.peek(), '\0');
    }

    #[test]
    fn test_next_peek() {
        let mut scanner1 = create_scanner();
        scanner1.source = String::from("012");
        scanner1.current = 1;
        assert_eq!(scanner1.next_peek(), '2');
        scanner1.current = 2;
        assert_eq!(scanner1.next_peek(), '\0');
    }

    #[test]
    fn test_string_succeed() {
        let mut scanner1 = create_scanner();
        scanner1.source = String::from("\"abcdefg\nhijkl\"");
        // Scanning starts from the letter 'a'
        scanner1.current = 1;
        scanner1.string();
        let value = String::from("abcdefg\nhijkl");
        let expected_token = Token::new(
            TokenType::LString,
            String::from("\"abcdefg\nhijkl\""),
            LiteralType::Str(value),
            2,
        );
        assert_eq!(scanner1.tokens[0], expected_token);
    }

    #[test]
    fn test_string_fail() {
        let mut scanner1 = create_scanner();
        scanner1.source = String::from("\"abcdef");
        // Scanning starts from the letter 'a'
        scanner1.current = 1;
        scanner1.string();
        assert_eq!(scanner1.tokens.len(), 0);
    }

    #[test]
    fn test_is_digit() {
        let scanner1 = create_scanner();
        assert!(scanner1.is_digit('8'));
        assert!(!scanner1.is_digit('a'));
    }

    #[test]
    fn test_number_integer() {
        let mut scanner1 = create_scanner();
        scanner1.source = String::from("123");
        scanner1.current = 1;
        scanner1.number();
        let value = 123f64;
        let expected_token = Token::new(
            TokenType::Number,
            String::from("123"),
            LiteralType::Num(value),
            1,
        );
        assert_eq!(scanner1.tokens[0], expected_token);
    }

    #[test]
    fn test_number_float() {
        let mut scanner1 = create_scanner();
        scanner1.source = String::from("123.456");
        scanner1.current = 1;
        scanner1.number();
        let value = 123.456f64;
        let expected_token = Token::new(
            TokenType::Number,
            String::from("123.456"),
            LiteralType::Num(value),
            1,
        );
        assert_eq!(scanner1.tokens[0], expected_token);
    }

    #[test]
    fn test_is_alpha() {
        let scanner1 = create_scanner();
        assert!(scanner1.is_alpha('g'));
        assert!(!scanner1.is_alpha('0'));
    }

    #[test]
    fn test_identifier() {
        let mut scanner1 = create_scanner();
        scanner1.source = String::from("and");
        scanner1.current = 1;
        scanner1.identifier();
        let expected_token = Token::new(TokenType::And, String::from("and"), LiteralType::Non, 1);
        assert_eq!(scanner1.tokens[0], expected_token);
    }
}
