use crate::expr::{Accept, Binary, Expr, Grouping, Literal, Unary, Visitor};
use crate::scanner::token::*;

struct AstPrinter {}

impl AstPrinter {
    fn print(self, expr: Expr) -> String {
        expr.accept(&self)
    }
}

impl Visitor<String> for AstPrinter {
    fn visit_binary_expr(&self, expr: &Binary) -> String {
        let exprs = vec![&expr.left, &expr.right];
        parenthesize(self, &expr.operator.lexeme, exprs)
    }

    fn visit_grouping_expr(&self, expr: &Grouping) -> String {
        let exprs = vec![&expr.expression];
        parenthesize(self, "group", exprs)
    }

    fn visit_literal_expr(&self, expr: &Literal) -> String {
        match &expr.value {
            LiteralType::Non => "nil".to_owned(),
            LiteralType::Num(n) => n.to_string(),
            LiteralType::Str(s) => s.to_owned(),
        }
    }

    fn visit_unary_expr(&self, expr: &Unary) -> String {
        let exprs = vec![&expr.right];
        parenthesize(self, &expr.operator.lexeme, exprs)
    }
}

fn parenthesize(visitor: &impl Visitor<String>, name: &str, exprs: Vec<&Box<Expr>>) -> String {
    let mut parenthesized_expr = String::new();
    parenthesized_expr.push('(');
    parenthesized_expr.push_str(&name);
    for expr in exprs {
        parenthesized_expr.push(' ');
        parenthesized_expr.push_str(&expr.accept(visitor))
    }
    parenthesized_expr.push(')');
    parenthesized_expr
}

/// Debugger for AST
pub fn print_ast() {
    let num1 = Box::new(Literal::new(LiteralType::Num(123.0)));
    let unary_right = Box::new(Expr::Literal(num1));
    let left = Expr::Unary(Box::new(Unary::new(
        Token::new(TokenType::Minus, "-".to_owned(), LiteralType::Non, 1),
        unary_right,
    )));
    let operator = Token::new(TokenType::Star, "*".to_owned(), LiteralType::Non, 1);
    let num2 = Box::new(Literal::new(LiteralType::Num(45.67)));
    let grouping = Box::new(Grouping::new(Box::new(Expr::Literal(num2))));
    let right = Box::new(Expr::Grouping(grouping));
    let bin_expr = Binary::new(Box::new(left), operator, right);
    let expr = Expr::Binary(Box::new(bin_expr));
    let ast_printer = AstPrinter {};
    println!("{}", ast_printer.print(expr))
}
