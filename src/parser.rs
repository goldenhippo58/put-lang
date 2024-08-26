use crate::ast::*;
use crate::token::{Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> ProgramNode {
        let mut program = ProgramNode::new();

        while !self.is_at_end() {
            if let Some(stmt) = self.parse_statement() {
                program.statements.push(stmt);
            } else {
                break;
            }
        }

        program
    }

    fn parse_statement(&mut self) -> Option<Box<dyn StatementNode>> {
        if self.match_token(TokenType::If) {
            self.parse_if_statement()
        } else if self.match_token(TokenType::While) {
            self.parse_while_statement()
        } else if self.match_token(TokenType::Var) {
            self.parse_variable_declaration()
        } else {
            self.parse_expression()
        }
    }

    fn parse_if_statement(&mut self) -> Option<Box<dyn StatementNode>> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.")?;
        let condition = self.parse_expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after if condition.")?;

        let then_branch = self.parse_statement()?;
        let mut else_branch = None;

        if self.match_token(TokenType::Else) {
            else_branch = Some(self.parse_statement()?);
        }

        Some(Box::new(IfNode::new(condition, then_branch, else_branch)))
    }

    fn parse_while_statement(&mut self) -> Option<Box<dyn StatementNode>> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'while'.")?;
        let condition = self.parse_expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after while condition.")?;

        let body = self.parse_statement()?;

        Some(Box::new(WhileNode::new(condition, body)))
    }

    fn parse_variable_declaration(&mut self) -> Option<Box<dyn StatementNode>> {
        let name_token = self.consume(TokenType::Identifier, "Expect variable name.")?;
        let var_name = name_token.lexeme.clone();

        // Assume data type, modify as necessary to detect actual type
        let data_type = DataType::Integer; // Default to Integer for example purposes

        let mut initializer = None;
        if self.match_token(TokenType::Assign) {
            initializer = Some(self.parse_expression()?);
        }

        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        )?;

        // Use the initializer in constructing the AssignmentNode if it exists
        if let Some(init) = initializer {
            Some(Box::new(AssignmentNode::new(
                Box::new(VariableNode::new(var_name, data_type)),
                init,
            )))
        } else {
            // If there's no initializer, just create a VariableNode
            Some(Box::new(VariableNode::new(var_name, data_type)))
        }
    }

    fn parse_expression(&mut self) -> Option<Box<dyn StatementNode>> {
        self.parse_addition()
    }

    fn parse_addition(&mut self) -> Option<Box<dyn StatementNode>> {
        let mut expr = self.parse_multiplication()?;

        while self.match_any(&[TokenType::Plus, TokenType::Minus]) {
            let operator = match self.previous().token_type {
                TokenType::Plus => BinaryOperator::Add,
                TokenType::Minus => BinaryOperator::Subtract,
                _ => unreachable!(),
            };
            let right = self.parse_multiplication()?;
            expr = Box::new(BinaryOperationNode::new(expr, operator, right));
        }

        Some(expr)
    }

    fn parse_multiplication(&mut self) -> Option<Box<dyn StatementNode>> {
        let mut expr = self.parse_primary()?;

        while self.match_any(&[TokenType::Star, TokenType::Slash]) {
            let operator = match self.previous().token_type {
                TokenType::Star => BinaryOperator::Multiply,
                TokenType::Slash => BinaryOperator::Divide,
                _ => unreachable!(),
            };
            let right = self.parse_primary()?;
            expr = Box::new(BinaryOperationNode::new(expr, operator, right));
        }

        Some(expr)
    }

    fn parse_primary(&mut self) -> Option<Box<dyn StatementNode>> {
        if self.match_token(TokenType::Number) {
            let token = self.previous();
            // Determine if the number is an integer or float for correct data type
            let data_type = if token.lexeme.contains('.') {
                DataType::Float
            } else {
                DataType::Integer
            };
            Some(Box::new(NumberNode::new(token.lexeme.clone(), data_type)))
        } else if self.match_token(TokenType::Identifier) {
            let token = self.previous();
            // Default to Integer, adjust based on context or additional checks
            let data_type = DataType::Integer;
            Some(Box::new(VariableNode::new(token.lexeme.clone(), data_type)))
        } else if self.match_token(TokenType::LeftParen) {
            let expr = self.parse_expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            Some(Box::new(ParenthesisNode::new(expr)))
        } else {
            eprintln!("Unexpected token: {:?}", self.peek());
            None
        }
    }

    fn match_token(&mut self, token_type: TokenType) -> bool {
        if self.check(token_type) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn match_any(&mut self, token_types: &[TokenType]) -> bool {
        for &token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().token_type == token_type
        }
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
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Option<&Token> {
        if self.check(token_type) {
            Some(self.advance())
        } else {
            eprintln!("Parse error: {} at line {}", message, self.peek().line);
            None
        }
    }
}
