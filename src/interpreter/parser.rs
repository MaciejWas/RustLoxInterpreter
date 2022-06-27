//! The parser. Basically a pure function from a `Vec<Token>` to a `Vec<Statement>`.

use crate::interpreter::errors::position::Position;
use crate::interpreter::{
    errors::{ErrBuilder, ErrType::ParsingErr, LoxResult},
    readers::{Reader, TokenReader},
    scanner::ScannerOutput,
    tokens::Punct::*,
    tokens::*,
    LoxError,
};

pub mod locator;
pub mod pretty_printing;
pub mod structure;
pub mod visitor;

use structure::*;

pub struct Parser {
    token_reader: TokenReader,
}

/// Parser á la recursive descent.
/// Methods either correspond directly to grammar rules (`program`, `statement`, ..., `unary`) or
/// are helper methods.
///
/// Methods which correspond to grammar rules can have `decider` word in their name - it means
/// that they do not progress the internal state but rather look ahead and determine which kind of grammar rule
/// comes next, and then call specific method which does progress the iternal state.
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
        let can_progress = || {
            self.token_reader
                .peek()
                .map(|t: &Token| !t.equals(&Eof))
                .unwrap_or(false)
        };
        while can_progress() {
            let next_stmt = self.statement_decider()?;
            stmts.push(next_stmt);
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
            statements.push(self.statement_decider()?);
            self.consume_punct(&Semicolon, info)?;
        }
        self.consume_punct(&RightBrace, "after reading scoped program")?;

        Ok(statements)
    }

    fn statement_decider(&self) -> LoxResult<Statement> {
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
            Token::KwdToken(Kwd::While, _) => self.while_stmt(),
            Token::KwdToken(Kwd::Fun, _) => self.fn_def_stmt(),
            _ => self.expr_stmt(),
        }
    }

    fn fn_def_stmt(&self) -> LoxResult<Statement> {
        let process_descr = "Parsing function definition";
        let pos = self.token_reader.peek().map(position_of);

        self.consume_kwd(&Kwd::Fun, process_descr)?;
        let fn_name = self.token_reader.advance_or(
            self.parsing_err()
                .with_message("Expected function name".to_string())
                .while_(process_descr)
                .build(),
        )?;

        let (id, _) = self.consume_identifier("Expected function name")?;

        let args: Vec<Expr> = self.parse_fn_arguments();
        let fn_body = self.scoped_program()?;
        let fn_def = FunctionDefinition {
            name: id.clone(),
            args: args,
            body: fn_body,
        };

        Ok(Statement::DefStmt(pos.unwrap(), fn_def))
    }

    fn fn_def_args(&self) -> LoxResult<Vec<Token>> {
        let info = "parsing function definition arguments";
        let mut args = Vec::new();

        self.consume_punct(&LeftParen, info)?;

        while self.consume_punct(&RightParen, info).is_ok() {
            let next_arg = self.token_reader.advance()?;
            match next_arg {
                IdentifierToken(id, pos) => args.push(next_arg),
                _ => return self.parsing_err().
            }
            args.push(next_arg);
            self.consume_punct(&Comma, info)?;
        }
        
        Ok(args)
    }

    fn fn_arguments(&self) -> LoxResult<Vec<Expr>> {
        let info = "parsing function arguments";
        let mut args = Vec::new();

        self.consume_punct(&LeftParen, info)?;

        while self.consume_punct(&RightParen, info).is_ok() {
            let next_arg = self.expr_decider()?;
            args.push(next_arg);
            self.consume_punct(&Comma, info)?;
        }
        
        Ok(args)
    }

    fn while_stmt(&self) -> LoxResult<Statement> {
        self.consume_kwd(&Kwd::While, "")
            .unwrap_or_else(|_err| panic!("Statement decider did something wrong. Attempted to parse a 'while statement' but no `while` keyword found"));
        let cond = self.parenthesized_expr()?;
        let prog = self.scoped_program()?;
        Ok(Statement::WhileLoop(cond, prog))
    }

    fn expr_stmt(&self) -> LoxResult<Statement> {
        let expr = self.expression_decider()?;
        Ok(Statement::ExprStmt(expr))
    }

    fn var_stmt(&self) -> LoxResult<Statement> {
        let info = "parsing assignment statement";
        self.consume_kwd(&Kwd::Var, info)?;

        let (identifier, _) = self.consume_identifier(info)?;
        self.consume_punct(&Equal, info)?;

        let expr = self.expression_decider()?;
        let lval = LVal {
            identifier: identifier,
        };
        let rval = RVal { expr };
        return Ok(Statement::LetStmt(lval, rval));
    }

    fn print_stmt(&self) -> LoxResult<Statement> {
        self.consume_kwd(&Kwd::Print, "Parsing `print statement`")?;
        Ok(Statement::PrintStmt(self.expression_decider()?))
    }

    fn if_stmt(&self) -> LoxResult<Statement> {
        self.consume_kwd(&Kwd::If, "Parsing `if statement`")?;

        let condition = self.parenthesized_expr()?;
        let inside_if = self.scoped_program()?;
        Ok(Statement::IfStmt(condition, inside_if))
    }

    fn expression_decider(&self) -> LoxResult<Expr> {
        let is_parenthesized = self.token_reader.peek().map(|t| t.equals(&LeftParen)).unwrap_or(false);
        if is_parenthesized {
            return self.parenthesized_expr();
        }
        self.expression()
    }

    fn expression(&self) -> LoxResult<Expr> {
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
        self.abstract_recursive_descent(Self::unary_decider, |t: &Token| {
            t.equals(&Star) || t.equals(&Slash)
        })
    }

    fn unary_decider(&self) -> LoxResult<Unary> {
        let first_token = self
            .token_reader
            .peek_or(self.expected_next_token_err("Parsing first token of a unary expression"))?;

        if first_token.equals(&LeftParen) {
            return self.recursive_noop_unary();
        }

        if first_token.can_be_unary_op() {
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

    fn recursive_noop_unary(&self) -> LoxResult<Unary> {
        let info = "Parsing parenthesized recursive unary expression";
        self.consume_punct(&LeftParen, info)?;
        let expr = self.expression()?;
        self.consume_punct(&RightParen, info)?;
        return Ok(Unary::Recursive(None, Box::new(expr)));
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

    fn parenthesized_expr(&self) -> LoxResult<Expr> {
        let info = "Processing parenthesized expression";

        self.consume_punct(&LeftParen, info)?;
        let expr_inside_parenth = self.expression_decider()?;
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
        ErrBuilder::new()
            .at(position_of(relevant_token))
            .of_type(ParsingErr)
    }

    fn expected_next_token_err(&self, info: &str) -> LoxError {
        self.parsing_err()
            .expected_found_nothing("token")
            .while_(info)
            .build()
    }

    fn consume_punct(&self, expected: &Punct, info: &str) -> LoxResult<()> {
        let token = self.token_reader.advance_or(
            self.parsing_err()
                .expected_found_nothing(expected)
                .while_(info)
                .build(),
        )?;
        token.satisfies_or(
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

    fn consume_identifier(&self, info: &str) -> LoxResult<(String, Position)> {
        let token = self.token_reader.advance_or(
            self.parsing_err()
                .expected_found_nothing("identifier")
                .while_(info)
                .build(),
        )?;
        match token {
            Token::IdentifierToken(id, pos) => Ok((id.clone(), pos.clone())),
            _ => self
                .parsing_err()
                .expected_but_found("identifier", token)
                .while_(info)
                .to_result(),
        }
    }
}
