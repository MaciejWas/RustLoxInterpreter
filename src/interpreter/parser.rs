use crate::interpreter::errors::ErrBuilder;
use crate::interpreter::errors::ErrType::ParsingErr;
use crate::interpreter::errors::LoxResult;
use crate::interpreter::readers::{Reader, TokenReader};
use crate::interpreter::scanner::ScannerOutput;
use crate::interpreter::tokens::Equals;
use crate::interpreter::tokens::Kwd;
use crate::interpreter::tokens::Punct::*;
use crate::interpreter::tokens::Token;
use crate::interpreter::LoxError;

pub mod pretty_printing;
pub mod structure;
pub mod visitor;

use structure::*;

pub struct Parser {
    token_reader: TokenReader,
}

/// Parser á la recursive descent.
/// Methods either correspond directly to grammar rules (`program`, `statement`, ..., `unary`) or
/// are helper methods
impl Parser {
    pub fn new(scanner_output: ScannerOutput) -> Self {
        Parser {
            token_reader: TokenReader::from_vec(scanner_output.tokens),
        }
    }

    pub fn parse(&self) -> LoxResult<Program> {
        self.program()
    }

    fn program(&self) -> LoxResult<Program> {
        let mut stmts = Vec::new();
        println!("LOLOL");
        while self
            .token_reader
            .peek()
            .map_or(false, |t: &Token| !t.equals(&Eof))
        {
            stmts.push(self.statement()?);

            let after_statement = self
                .token_reader
                .advance_or(self.parsing_err().expected_found_nothing(";").build())?;

            if !after_statement.equals(&Semicolon) {
                return Err(self
                    .parsing_err()
                    .expected_but_found(";", after_statement)
                    .build());
            }
        }

        Ok(stmts)
    }

    fn statement(&self) -> LoxResult<Statement> {
        let pos = self
            .token_reader
            .previous()
            .map(|t: &Token| t.pos())
            .unwrap_or(0);

        let first_token = self.token_reader.peek_or(
            ErrBuilder::at(pos)
                .with_type(ParsingErr)
                .expected_found_nothing("next token")
                .build(),
        )?;

        if first_token.equals(&Kwd::Print) {
            self.token_reader.advance();
            return Ok(Statement::PrintStmt(self.expression()?));
        }

        Ok(Statement::ExprStmt(self.expression()?))
    }

    fn expression(&self) -> LoxResult<Expr> {
        let is_parenthesized = self
            .token_reader
            .peek_or(self.expected_next_token())?
            .equals(&LeftParen);

        if is_parenthesized {
            self.token_reader.advance();
            let parenthesized_expr = self.expression();
            self.token_reader
                .advance_or(self.expected_next_token())?
                .satisfies_or(
                    |t: &Token| t.equals(&RightParen),
                    |t: &Token| self.parsing_err().expected_but_found(RightParen, t).build(),
                )?;

            return parenthesized_expr;
        }

        let eq: Eqlty = self.equality()?;
        Ok(Expr::Eqlty(eq))
    }

    fn equality(&self) -> LoxResult<Eqlty> {
        self.abstract_recursive_descent(Self::comparison, |t: &Token| {
            t.equals(&EqualEqual) || t.equals(&BangEqual)
        })
    }

    fn comparison(&self) -> LoxResult<Comp> {
        self.abstract_recursive_descent(Self::term, |t: &Token| {
            t.equals(&LessEqual) || t.equals(&GreaterEqual) || t.equals(&Less) || t.equals(&Greater)
        })
    }

    fn term(&self) -> LoxResult<Term> {
        self.abstract_recursive_descent(Self::factor, |t: &Token| {
            t.equals(&Plus) || t.equals(&Minus)
        })
    }

    fn factor(&self) -> LoxResult<Factor> {
        self.abstract_recursive_descent(Self::unary, |t: &Token| {
            t.equals(&Star) || t.equals(&Slash)
        })
    }

    fn unary(&self) -> LoxResult<Unary> {
        let first_token = self.token_reader.advance_or(self.expected_next_token())?;

        if first_token.equals(&LeftParen) {
            return self.parenthesized_unary();
        }

        if first_token.can_be_unary_op() {
            let second_token = self.token_reader.advance_or(self.expected_next_token())?;

            return self.unary_op(first_token, second_token);
        }

        self.unary_noop(first_token)
    }

    fn unary_op(&self, first_token: &Token, second_token: &Token) -> LoxResult<Unary> {
        if let Token::ValueToken(_, _) = second_token {
            return Ok(Unary::Final(
                Some(first_token.clone()),
                second_token.clone(),
            ));
        }

        if let Token::IdentifierToken(_, _) = second_token {
            return Ok(Unary::Final(
                Some(first_token.clone()),
                second_token.clone(),
            ));
        }

        if second_token.equals(&LeftParen) {
            let expr_inside_parenth = Box::new(self.parenthesized_expr()?);
            return Ok(Unary::Recursive(
                Some(first_token.clone()),
                expr_inside_parenth,
            ));
        }

        return Err(self
            .parsing_err()
            .is_not(second_token, "a valid lox value")
            .build());
    }

    fn unary_noop(&self, first_token: &Token) -> LoxResult<Unary> {
        match first_token {
            Token::ValueToken(_, _) => Ok(Unary::Final(None, first_token.clone())),
            Token::IdentifierToken(_, _) => Ok(Unary::Final(None, first_token.clone())),
            Token::PunctToken(punct, pos) => {
                if punct == &LeftParen {
                    let expr_inside_parenth = Box::new(self.parenthesized_expr()?);
                    return Ok(Unary::Recursive(
                        Some(first_token.clone()),
                        expr_inside_parenth,
                    ));
                }
                Err(self
                    .parsing_err()
                    .with_pos(*pos)
                    .expected_but_found("unary expression", first_token)
                    .build())
            }
            _ => Err(self
                .parsing_err()
                .with_pos(first_token.pos())
                .expected_but_found("unary expression", first_token)
                .build()),
        }
    }

    fn parenthesized_unary(&self) -> LoxResult<Unary> {
        let unary_inside_parenth = self.unary();
        self.token_reader
            .advance_or(self.parsing_err().expected_found_nothing(')').build())?
            .satisfies_or(
                |t: &Token| t.equals(&RightParen),
                |t: &Token| self.parsing_err().expected_but_found(RightParen, t).build(),
            )?;

        return unary_inside_parenth;
    }

    fn parenthesized_expr(&self) -> LoxResult<Expr> {
        let expr_inside_parenth = self.expression()?;
        self.token_reader
            .advance_or(self.parsing_err().expected_found_nothing(')').build())?
            .satisfies_or(
                |t: &Token| t.equals(&RightParen),
                |t: &Token| self.parsing_err().expected_but_found(')', t).build(),
            )?;

        Ok(expr_inside_parenth)
    }

    /// This function builds struct representing Rule from its sub rules, assuming `Rule = SubRule [ Token SubRule ]*` where `token_predicate(Token) = true`.
    /// Arguments:
    ///     next_rule: function for finding `SubRule`
    ///     token_predicate: decides if token matches `Rule`
    fn abstract_recursive_descent<SubRule, Rule>(
        &self,
        next_rule: fn(&Self) -> LoxResult<SubRule>,
        token_predicate: fn(&Token) -> bool,
    ) -> LoxResult<Rule>
    where
        Rule: FromSubRules<SubRule>,
    {
        let mut sub_rules = Vec::new();
        let first_sub_rule = next_rule(&self)?;

        while let Some(token) = self.token_reader.advance_if(token_predicate) {
            let next_sub_rule = next_rule(&self)?;
            sub_rules.push((token.clone(), next_sub_rule));
        }

        Ok(Rule::from_sub(first_sub_rule, sub_rules))
    }

    fn current_pos(&self) -> usize {
        self.token_reader
            .previous()
            .map(|t: &Token| t.pos())
            .unwrap_or(0)
    }

    fn parsing_err(&self) -> ErrBuilder {
        ErrBuilder::at(self.current_pos()).with_type(ParsingErr)
    }

    fn expected_next_token(&self) -> LoxError {
        self.parsing_err().expected_found_nothing("token").build()
    }
}
