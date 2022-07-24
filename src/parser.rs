use crate::expr::{Assign, Binary, Call, Expr, Get, Grouping, Literal, Logical, Unary, Variable};
use crate::lang_error::{self, LangError};
use crate::object::literal_type::LiteralType;
use crate::scanner::token::{Token, TokenType};
use crate::stmt::{Block, Class, Expression, Function, If, Print, Return, Stmt, Var, While};

#[derive(Default, Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

type Statements = Vec<Stmt>;

const MAX_NUM_OF_ARGS: usize = 255;

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
        } else if self.match_token_type(&vec![TokenType::Fun]) {
            self.function("function")
        } else if self.match_token_type(&vec![TokenType::Class]) {
            self.class_declaration()
        } else {
            self.statement()
        };
        if let Err(e) = result {
            self.synchronize();
            return Err(e);
        }
        result
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

    fn class_declaration(&mut self) -> Result<Stmt, LangError> {
        let name = self
            .consume(TokenType::Identifier, "Expect class name.")?
            .clone();
        self.consume(TokenType::LeftBrace, "Expect '{' before class body.")?;

        let mut methods: Vec<Function> = Vec::new();
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            match self.function("method")? {
                Stmt::Function(method) => methods.push(method),
                _ => panic!("Supposed to be a method"),
            }
        }

        self.consume(TokenType::RightBrace, "Expect '}' after class body.")?;
        Ok(Stmt::Class(Class::new(name, methods)))
    }

    fn function(&mut self, kind: &str) -> Result<Stmt, LangError> {
        let message = format!("Expect {} name.", kind);
        let name = self.consume(TokenType::Identifier, &message)?.clone();
        let message = format!("Expect '(' after {} name.", kind);
        self.consume(TokenType::LeftParen, &message)?;
        let mut parameters = Vec::new();
        if !self.check(&TokenType::RightParen) {
            loop {
                if parameters.len() >= MAX_NUM_OF_ARGS {
                    let error_message =
                        format!("Can't have more than {} parameters.", MAX_NUM_OF_ARGS);
                    lang_error::parser_error(self.peek(), error_message);
                }
                let parameter = self.consume(TokenType::Identifier, "Expect parameter name.")?;
                parameters.push(parameter.clone());
                if !self.match_token_type(&vec![TokenType::Comma]) {
                    break;
                }
            }
        }
        self.consume(TokenType::RightParen, "Expect ')' after parameters.")?;

        let message = format!("Expect '{{' before {} body.", kind);
        self.consume(TokenType::LeftBrace, &message)?;
        let body = self.block()?;
        Ok(Stmt::Function(Function::new(name, parameters, body)))
    }

    fn statement(&mut self) -> Result<Stmt, LangError> {
        if self.match_token_type(&vec![TokenType::If]) {
            return self.if_statement();
        }
        if self.match_token_type(&vec![TokenType::For]) {
            return self.for_statement();
        }
        if self.match_token_type(&vec![TokenType::Print]) {
            return self.print_statement();
        }
        if self.match_token_type(&vec![TokenType::Return]) {
            return self.return_statement();
        }
        if self.match_token_type(&vec![TokenType::While]) {
            return self.while_statement();
        }
        if self.match_token_type(&vec![TokenType::LeftBrace]) {
            let statements = self.block()?;
            let block = Stmt::Block(Block::new(statements));
            return Ok(block);
        }
        self.expression_statement()
    }

    fn if_statement(&mut self) -> Result<Stmt, LangError> {
        self.consume(TokenType::LeftParen, "Expect '(' after value.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after value.")?;
        let then_branch = Box::new(self.statement()?);
        let else_branch = if self.match_token_type(&vec![TokenType::Else]) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };

        Ok(Stmt::If(If::new(condition, then_branch, else_branch)))
    }

    fn for_statement(&mut self) -> Result<Stmt, LangError> {
        self.consume(TokenType::LeftParen, "Expect '(' after value.")?;
        let initializer = if self.match_token_type(&vec![TokenType::Semicolon]) {
            None
        } else if self.match_token_type(&vec![TokenType::Var]) {
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };
        let condition = if self.check(&TokenType::Semicolon) {
            Expr::Literal(Literal::new(LiteralType::True))
        } else {
            self.expression()?
        };
        self.consume(TokenType::Semicolon, "Expect ';' after loop condition.")?;
        let increment = if self.check(&TokenType::RightParen) {
            None
        } else {
            Some(self.expression()?)
        };
        self.consume(TokenType::RightParen, "Expect ')' after for clauses.")?;
        let body = self.statement()?;
        let for_loop_with_increment = if let Some(expr) = increment {
            let statements = vec![body, Stmt::Expression(Expression::new(expr))];
            let block = Block::new(statements);
            Stmt::Block(block)
        } else {
            body
        };
        let while_loop = While::new(condition, Box::new(for_loop_with_increment));
        let for_loop_with_condition = Stmt::While(while_loop);
        let for_loop_with_initializer = if let Some(statement) = initializer {
            let statements = vec![statement, for_loop_with_condition];
            let block = Block::new(statements);
            Stmt::Block(block)
        } else {
            for_loop_with_condition
        };

        Ok(for_loop_with_initializer)
    }

    fn print_statement(&mut self) -> Result<Stmt, LangError> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Print(Print::new(value)))
    }

    fn return_statement(&mut self) -> Result<Stmt, LangError> {
        let keyword = self.previous().clone();
        let value = if self.check(&TokenType::Semicolon) {
            let nil = Expr::Literal(Literal::new(LiteralType::Nil));
            Ok(nil)
        } else {
            self.expression()
        }?;

        self.consume(TokenType::Semicolon, "Expect ';' after return value.")?;
        Ok(Stmt::Return(Return::new(keyword, value)))
    }

    fn while_statement(&mut self) -> Result<Stmt, LangError> {
        self.consume(TokenType::LeftParen, "Expect '(' after value.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after value.")?;
        let body = Box::new(self.statement()?);

        Ok(Stmt::While(While::new(condition, body)))
    }

    fn block(&mut self) -> Result<Statements, LangError> {
        let mut statements = Vec::new();

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            let declaration = self.declaration()?;
            statements.push(declaration);
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;
        Ok(statements)
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
        let expr = self.or()?;

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

    fn or(&mut self) -> Result<Expr, LangError> {
        let token_types = vec![TokenType::Or];
        self.generate_logical_expr(token_types, Parser::and)
    }

    fn and(&mut self) -> Result<Expr, LangError> {
        let token_types = vec![TokenType::And];
        self.generate_logical_expr(token_types, Parser::equality)
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

        self.call()
    }

    fn call(&mut self) -> Result<Expr, LangError> {
        let primary = self.primary()?;

        let mut expr = primary;
        loop {
            if self.match_token_type(&vec![TokenType::LeftParen]) {
                expr = self.finish_call(expr.clone())?;
            } else if self.match_token_type(&vec![TokenType::Dot]) {
                let name =
                    self.consume(TokenType::Identifier, "Expect property name after '.'.")?;
                let get_expression = Get::new(Box::new(expr.clone()), name.clone());
                expr = Expr::Get(get_expression);
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, LangError> {
        let mut arguments = Vec::new();
        if !self.check(&TokenType::RightParen) {
            loop {
                let argument = self.expression()?;
                arguments.push(argument);
                if !self.match_token_type(&vec![TokenType::Comma]) {
                    break;
                }
            }
        }
        if arguments.len() >= MAX_NUM_OF_ARGS {
            let error_message = format!("Can't have more than {} arguments", MAX_NUM_OF_ARGS);
            lang_error::parser_error(self.peek(), error_message);
        }
        let paren = self.consume(TokenType::RightParen, "Expect ')' after arguments.")?;
        let call = Call::new(Box::new(callee), paren.clone(), arguments);
        Ok(Expr::Call(call))
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

    fn generate_logical_expr(
        &mut self,
        token_types: Vec<TokenType>,
        precedence: fn(&mut Self) -> Result<Expr, LangError>,
    ) -> Result<Expr, LangError> {
        let mut expr = precedence(self)?;

        while self.match_token_type(&token_types) {
            let operator = self.previous().clone();
            let right = precedence(self)?;
            let logical = Logical::new(Box::new(expr), operator, Box::new(right));
            let new_expr = Expr::Logical(logical);
            expr = new_expr;
        }
        Ok(expr)
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
