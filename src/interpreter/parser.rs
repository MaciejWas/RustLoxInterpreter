use crate::interpreter::{
    errors::{ErrBuilder, ErrType::ParsingErr, LoxResult},
    readers::{Reader, TokenReader},
    scanner::ScannerOutput,
    tokens::Punct::*,
    tokens::*,
    LoxError,
};

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
        while self
            .token_reader
            .peek()
            .map_or(false, |t: &Token| !t.equals(&Eof))
        {
            stmts.push(self.statement()?);
            self.consume_punct(&Semicolon, "Reading statements")?;
        }

        Ok(stmts)
    }

    fn scoped_program(&self) -> LoxResult<Program> {
        let info = "Parsing scoped statements";
        self.consume_punct(&LeftBrace, info)?;

        let mut statements: Program = Vec::new();

        while !self
            .token_reader
            .peek_or(self.expected_next_token_err(info))?
            .equals(&RightBrace)
        {
            statements.push(self.statement()?);
            self.consume_punct(&Semicolon, info)?;
        }
        self.consume_punct(&RightBrace, "after reading scoped program")?;

        Ok(statements)
    }

    fn statement(&self) -> LoxResult<Statement> {
        let first_token = self.token_reader.peek_or(
            self.parsing_err()
                .expected_found_nothing("first token of a statement")
                .while_("Parsing statement")
                .build(),
        )?;
        match first_token {
            Token::KwdToken(Kwd::Print, _) => self.print_stmt(),
            Token::KwdToken(Kwd::If, _) => self.if_stmt(),
            Token::KwdToken(Kwd::Var, _) => self.var_stmt(),
            _ => Ok(Statement::ExprStmt(self.expression()?)),
        }
    }

    fn var_stmt(&self) -> LoxResult<Statement> {
        let info = "parsing assignment statement";
        self.consume_kwd(&Kwd::Var, info)?;
        let var_name: &Token = self.token_reader.advance_or(self.expected_next_token_err(info))?;

        if let Token::IdentifierToken(identifier, _) = var_name {
            self.consume_punct(&Equal, info)?;
            let expr = self.expression()?;
            let lval = LVal { identifier: identifier.clone() };
            let rval = RVal { expr };
            return Ok(Statement::LetStmt(lval, rval));
        }

        Err(self.parsing_err().expected_but_found("identifier", var_name).build())
    }

    fn print_stmt(&self) -> LoxResult<Statement> {
        self.consume_kwd(&Kwd::Print, "Parsing `print statement`")?;
        Ok(Statement::PrintStmt(self.expression()?))
    }

    fn if_stmt(&self) -> LoxResult<Statement> {
        self.consume_kwd(&Kwd::If, "Parsing `if statement`")?;

        let condition = self.parenthesized_expr()?;
        let inside_if = self.scoped_program()?;
        Ok(Statement::IfStmt(condition, inside_if))
    }

    fn expression(&self) -> LoxResult<Expr> {
        let is_parenthesized = self
            .token_reader
            .peek_or(self.expected_next_token_err("Checking if expression is inside parenthesis"))?
            .equals(&LeftParen);

        if is_parenthesized {
            self.token_reader.advance();
            let parenthesized_expr = self.expression();
            self.consume_punct(&RightParen, "Looking for closing parenthesis")?;

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
        let first_token = self
            .token_reader
            .peek_or(self.expected_next_token_err("Parsing first token of a unary expression"))?;

        if first_token.equals(&LeftParen) {
            return self.parenthesized_unary();
        }

        if first_token.can_be_unary_op() {
            self.token_reader
                .advance()
                .unwrap_or_else(|| panic!("This should not happen since we fucking peeked"));
            return self.unary_op();
        }

        self.unary_noop()
    }

    fn unary_op(&self) -> LoxResult<Unary> {
        let info = "Parsing unary expression with unary operator.";
        let first_token = self
            .token_reader
            .advance_or(self.expected_next_token_err(info))?;
        let second_token = self
            .token_reader
            .advance_or(self.expected_next_token_err(info))?;

        match second_token {
            Token::ValueToken(_, _) => Ok(Unary::Final(
                Some(first_token.clone()),
                second_token.clone(),
            )),
            Token::IdentifierToken(_, _) => Ok(Unary::Final(
                Some(first_token.clone()),
                second_token.clone(),
            )),
            Token::PunctToken(LeftParen, _) => Ok(Unary::Recursive(
                Some(first_token.clone()),
                Box::new(self.parenthesized_expr()?),
            )),
            _ => Err(self
                .parsing_err()
                .is_not(second_token, "a valid lox value")
                .build()),
        }
    }

    fn unary_noop(&self) -> LoxResult<Unary> {
        let first_token = self.token_reader.advance().unwrap_or_else(|| {
            panic!("unary_noop should be called after making sure that next token exists")
        });
        match first_token {
            Token::ValueToken(_, _) => Ok(Unary::Final(None, first_token.clone())),
            Token::IdentifierToken(_, _) => Ok(Unary::Final(None, first_token.clone())),
            Token::PunctToken(LeftParen, _) => {
                Ok(Unary::Recursive(None, Box::new(self.parenthesized_expr()?)))
            }
            _ => Err(self
                .parsing_err()
                .expected_but_found("unary expression", first_token)
                .build()),
        }
    }

    fn parenthesized_unary(&self) -> LoxResult<Unary> {
        self.consume_punct(&LeftParen, "Parsing parenthesized unary expression")?;
        let unary_inside_parenth = self.unary();
        self.consume_punct(&RightParen, "Parsing parenthesized unary expression")?;
        return unary_inside_parenth;
    }

    fn parenthesized_expr(&self) -> LoxResult<Expr> {
        let info = "Processing parenthesized expression";

        self.consume_punct(&LeftParen, info)?;
        let expr_inside_parenth = self.expression()?;
        self.consume_punct(&RightParen, info)?;

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

    fn parsing_err(&self) -> ErrBuilder {
        let relevant_token = self
            .token_reader
            .previous()
            .or(self.token_reader.peek())
            .unwrap_or_else(|| {
                panic!("Failed to find the first token while generating error message.")
            });
        ErrBuilder::at(position_of(relevant_token)).of_type(ParsingErr)
    }

    fn expected_next_token_err(&self, info: &str) -> LoxError {
        self.parsing_err()
            .expected_found_nothing("token")
            .while_(info)
            .build()
    }

    fn consume_punct(&self, expected: &Punct, info: &str) -> LoxResult<()> {
        self.token_reader
            .advance_or(
                self.parsing_err()
                    .expected_found_nothing(expected)
                    .while_(info)
                    .build(),
            )?
            .satisfies_or(
                |t: &Token| t.equals(expected),
                |t: &Token| {
                    self.parsing_err()
                        .expected_but_found(expected, t)
                        .while_(info)
                        .build()
                },
            )?;
        Ok(())
    }

    fn consume_kwd(&self, expected: &Kwd, info: &str) -> LoxResult<()> {
        self.token_reader
            .advance_or(
                self.parsing_err()
                    .expected_found_nothing(expected)
                    .while_(info)
                    .build(),
            )?
            .satisfies_or(
                |t: &Token| t.equals(expected),
                |t: &Token| {
                    self.parsing_err()
                        .expected_but_found(expected, t)
                        .while_(info)
                        .build()
                },
            )?;
        Ok(())
    }
}
