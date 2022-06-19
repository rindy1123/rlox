use crate::expr::{Assign, Binary, Expr, Grouping, Literal, Unary, Variable};
use crate::lang_error::{self, LangError};
use crate::scanner::literal_type::LiteralType;
use crate::scanner::token::{Token, TokenType};
use crate::stmt::{Expression, Print, Stmt, Var};

#[derive(Default, Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            ..Default::default()
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, LangError> {
        let mut statements: Vec<Stmt> = Vec::new();
        while !self.is_at_end() {
            let statement = self.declaration()?;
            statements.push(statement);
        }
        Ok(statements)
    }

    fn declaration(&mut self) -> Result<Stmt, LangError> {
        let result = if self.match_token_type(&vec![TokenType::Var]) {
            self.var_declaration()
        } else {
            self.statement()
        };
        if let Err(e) = result {
            self.synchronize();
            Err(e)
        } else {
            result
        }
    }

    fn var_declaration(&mut self) -> Result<Stmt, LangError> {
        let name = self
            .consume(TokenType::Identifier, "Expect variable name.")?
            .clone();

        let initializer = if self.match_token_type(&vec![TokenType::Equal]) {
            self.expression()?
        } else {
            Expr::Literal(Literal::new(LiteralType::Nil))
        };

        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        )?;
        let var = Var::new(name, initializer);
        Ok(Stmt::Var(var))
    }

    fn statement(&mut self) -> Result<Stmt, LangError> {
        if self.match_token_type(&vec![TokenType::Print]) {
            return self.print_statement();
        }
        self.expression_statement()
    }

    fn print_statement(&mut self) -> Result<Stmt, LangError> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Print(Print::new(value)))
    }

    fn expression_statement(&mut self) -> Result<Stmt, LangError> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Expression(Expression::new(value)))
    }

    fn expression(&mut self) -> Result<Expr, LangError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, LangError> {
        let expr = self.equality()?;

        if self.match_token_type(&vec![TokenType::Equal]) {
            let equals = self.previous().clone();
            let value = self.assignment()?;

            if let Expr::Variable(var) = expr {
                let name = var.name;
                let assign = Expr::Assign(Assign::new(name, Box::new(value)));
                return Ok(assign);
            } else {
                lang_error::parser_error(&equals, "Expect expression.".to_string());
                return Err(LangError::ParseError);
            }
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, LangError> {
        let token_types = vec![TokenType::EqualEqual, TokenType::BangEqual];
        self.generate_binary_expr(token_types, Parser::comparison)
    }

    fn comparison(&mut self) -> Result<Expr, LangError> {
        let token_types = vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ];
        self.generate_binary_expr(token_types, Parser::term)
    }

    fn term(&mut self) -> Result<Expr, LangError> {
        let token_types = vec![TokenType::Minus, TokenType::Plus];
        self.generate_binary_expr(token_types, Parser::factor)
    }

    fn factor(&mut self) -> Result<Expr, LangError> {
        let token_types = vec![TokenType::Star, TokenType::Slash];
        self.generate_binary_expr(token_types, Parser::unary)
    }

    fn unary(&mut self) -> Result<Expr, LangError> {
        let token_types = vec![TokenType::Bang, TokenType::Minus];
        if self.match_token_type(&token_types) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            let unary = Unary::new(operator, Box::new(right));
            return Ok(Expr::Unary(unary));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, LangError> {
        if self.match_token_type(&vec![TokenType::False]) {
            let literal = Literal::new(LiteralType::False);
            return Ok(Expr::Literal(literal));
        }
        if self.match_token_type(&vec![TokenType::True]) {
            let literal = Literal::new(LiteralType::True);
            return Ok(Expr::Literal(literal));
        }
        if self.match_token_type(&vec![TokenType::Nil]) {
            let literal = Literal::new(LiteralType::Nil);
            return Ok(Expr::Literal(literal));
        }

        if self.match_token_type(&vec![TokenType::Number, TokenType::LString]) {
            let value = self.previous().clone().literal.unwrap();
            let literal = Literal::new(value);
            return Ok(Expr::Literal(literal));
        }

        if self.match_token_type(&vec![TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            let grouping = Grouping::new(Box::new(expr));
            return Ok(Expr::Grouping(grouping));
        }

        if self.match_token_type(&vec![TokenType::Identifier]) {
            let name = self.previous().clone();
            let var = Variable::new(name);
            return Ok(Expr::Variable(var));
        }

        lang_error::parser_error(self.peek(), "Expect expression.".to_string());
        Err(LangError::ParseError)
    }

    fn generate_binary_expr(
        &mut self,
        token_types: Vec<TokenType>,
        precedence: fn(&mut Self) -> Result<Expr, LangError>,
    ) -> Result<Expr, LangError> {
        let mut expr = precedence(self)?;

        while self.match_token_type(&token_types) {
            let operator = self.previous().clone();
            let right = precedence(self)?;
            let binary = Binary::new(Box::new(expr), operator, Box::new(right));
            let new_expr = Expr::Binary(binary);
            expr = new_expr;
        }
        Ok(expr)
    }

    fn match_token_type(&mut self, token_types: &Vec<TokenType>) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        return self.peek().token_type == *token_type;
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap()
    }

    fn previous(&self) -> &Token {
        self.tokens.get(self.current - 1).unwrap()
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<&Token, LangError> {
        if self.check(&token_type) {
            return Ok(self.advance());
        }

        lang_error::parser_error(self.peek(), message.to_string());
        Err(LangError::ParseError)
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }
            match self.peek().token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => self.advance(),
            };
        }
    }
}
