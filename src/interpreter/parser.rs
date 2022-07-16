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
/// comes next.
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
        while !self.is_finished() {
            let next_stmt = self.statement()?;
            stmts.push(next_stmt);
            self.consume_punct(Semicolon, "Reading statements")?;
        }

        Ok(stmts)
    }

    fn is_finished(&self) -> bool {
        self.token_reader
            .peek()
            .map(|t: &Token| t.equals(Eof))
            .unwrap_or(false)
    }

    fn scoped_program(&self) -> LoxResult<Program> {
        let info = "Parsing scoped statements";
        let end_of_scope = || {
            self.token_reader
                .peek()
                .map(|t| t.equals(Punct::RightBrace))
                .unwrap_or(false)
        };
        self.consume_punct(LeftBrace, info)?;

        let mut program = Vec::new();
        while !end_of_scope() {
            let next_stmt = self.statement()?;
            program.push(next_stmt);
            self.consume_punct(Semicolon, "Reading statements")?;
        }

        self.consume_punct(RightBrace, info)?;
        Ok(program)
    }

    fn statement(&self) -> LoxResult<Statement> {
        let stmt_kind = self.statement_decider()?;
        let stmt = match stmt_kind {
            StatementKind::Expr => self.expr_stmt(),
            StatementKind::Fun => self.function_definition(),
            StatementKind::Let => self.var_stmt(),
            StatementKind::If => self.if_stmt(),
            StatementKind::Return => self.return_(),
            StatementKind::WhileLoop => self.while_stmt(),
            StatementKind::Print => self.print_stmt(),
            StatementKind::Class => self.class_def_stmt(),
        };
        Ok(stmt?)
    }

    fn statement_decider(&self) -> LoxResult<StatementKind> {
        let first_token = self.token_reader.peek_or(
            self.parsing_err()
                .expected_found_nothing("first token of a statement")
                .while_("Parsing statement")
                .build(),
        )?;

        if let TokenValue::Kwd(kwd) = &first_token.val {
            return match kwd {
                Kwd::Print => Ok(StatementKind::Print),
                Kwd::If => Ok(StatementKind::If),
                Kwd::Var => Ok(StatementKind::Let),
                Kwd::While => Ok(StatementKind::WhileLoop),
                Kwd::Fun => Ok(StatementKind::Fun),
                Kwd::Return => Ok(StatementKind::Return),
                Kwd::Class => Ok(StatementKind::Class),
                _ => Ok(StatementKind::Expr),
            };
        }

        return Ok(StatementKind::Expr);
    }

    fn class_def_stmt(&self) -> LoxResult<Statement> {
        let info = "parsing class def";
        let mut fields = Vec::new();
        let mut methods = Vec::new();

        self.consume_kwd(Kwd::Class, info)?;
        let class_name = self
            .token_reader
            .advance_or(self.expected_next_token_err(info))?;
        if !class_name.is_identifier() {
            return self
                .parsing_err()
                .expected_but_found("identifier", class_name)
                .to_result();
        }

        self.consume_punct(Punct::LeftBrace, info)?;
        while let Some(next_token) = self.token_reader.peek() {
            if next_token.equals(Punct::RightBrace) {
                break;
            }

            let next_stmt = self.statement()?;
            self.consume_punct(Punct::Semicolon, info)?;
            match next_stmt {
                Statement::Let(lval, rval) => fields.push((lval, rval)),
                Statement::Fun(_, fn_def) => methods.push(fn_def),
                _ => {
                    return self
                        .parsing_err()
                        .expected_but_found("function or let statement", next_stmt)
                        .to_result()
                }
            }
        }
        self.consume_punct(Punct::RightBrace, info)?;
        let class = ClassDefinition {
            name: class_name.clone(),
            fields: fields,
            methods: methods,
        };

        Ok(Statement::Class(class))
    }

    fn return_(&self) -> LoxResult<Statement> {
        self.consume_kwd(Kwd::Return, "this basically cant fail")?;
        let expr = self.expression()?;
        Ok(Statement::Return(expr))
    }

    fn function_definition(&self) -> LoxResult<Statement> {
        let pos = self.token_reader.peek().map(|t| t.pos);
        self.consume_kwd(Kwd::Fun, "Parsing function definition")?;

        let (fn_name, _) = self.consume_identifier("Expected function name")?;
        let args = self.fn_def_args()?;
        let fn_body = self.scoped_program()?;

        let fn_def = FunctionDefinition {
            name: fn_name.clone(),
            args: args,
            body: fn_body,
        };

        Ok(Statement::Fun(pos.unwrap(), fn_def))
    }

    fn fn_def_args(&self) -> LoxResult<Vec<String>> {
        let info = "parsing function definition arguments";
        let next_token_is_comma = || {
            self.token_reader
                .peek()
                .map(|t| t.equals(Comme))
                .unwrap_or(false)
        };
        let reached_end_of_args = || {
            self.token_reader
                .peek()
                .map(|t| t.equals(Punct::RightParen))
                .unwrap_or(false)
        };
        let mut args = Vec::new();

        self.consume_punct(LeftParen, info)?;

        while next_token_is_comma() || args.is_empty() {
            if next_token_is_comma() {
                self.token_reader.advance();
            }

            if reached_end_of_args() {
                self.token_reader.advance();
                return Ok(args);
            }

            let (id, _) = self.consume_identifier("parsing function definition arguments")?;
            args.push(id);
        }

        self.consume_punct(RightParen, info)?;
        Ok(args)
    }

    fn while_stmt(&self) -> LoxResult<Statement> {
        self.consume_kwd(Kwd::While, "")
            .unwrap_or_else(|_err| panic!("Statement decider did something wrong. Attempted to parse a 'while statement' but no `while` keyword found"));
        let cond = self.parenthesized_expr()?;
        let prog = self.scoped_program()?;
        Ok(Statement::WhileLoop(cond, prog))
    }

    fn expr_stmt(&self) -> LoxResult<Statement> {
        let expr = self.expression()?;
        Ok(Statement::Expr(expr))
    }

    fn var_stmt(&self) -> LoxResult<Statement> {
        let info = "parsing assignment statement";
        self.consume_kwd(Kwd::Var, info)?;

        let (identifier, _) = self.consume_identifier(info)?;
        self.consume_punct(Equal, info)?;

        let expr = self.expression()?;
        let lval = LVal {
            identifier: identifier,
        };
        let rval = RVal { expr };
        return Ok(Statement::Let(lval, rval));
    }

    fn print_stmt(&self) -> LoxResult<Statement> {
        self.consume_kwd(Kwd::Print, "Parsing a print statement")?;
        Ok(Statement::Print(self.expression()?))
    }

    fn if_stmt(&self) -> LoxResult<Statement> {
        self.consume_kwd(Kwd::If, "Parsing an if statement")?;

        let condition = self.parenthesized_expr()?;
        let inside_if = self.scoped_program()?;
        Ok(Statement::If(condition, inside_if))
    }

    fn expression(&self) -> LoxResult<Expr> {
        let eq: Eqlty = self.equality()?;
        Ok(Expr::Eqlty(eq))
    }

    fn equality(&self) -> LoxResult<Eqlty> {
        self.abstract_recursive_descent(Self::comparison, |t: &Token| {
            t.equals(EqualEqual) || t.equals(BangEqual)
        })
    }

    fn comparison(&self) -> LoxResult<Comp> {
        self.abstract_recursive_descent(Self::term, |t: &Token| {
            t.equals(LessEqual) || t.equals(GreaterEqual) || t.equals(Less) || t.equals(Greater)
        })
    }

    fn term(&self) -> LoxResult<Term> {
        self.abstract_recursive_descent(Self::factor, |t: &Token| t.equals(Plus) || t.equals(Minus))
    }

    fn factor(&self) -> LoxResult<Factor> {
        self.abstract_recursive_descent(Self::unary, |t: &Token| t.equals(Star) || t.equals(Slash))
    }

    fn unary(&self) -> LoxResult<Unary> {
        let unary_kind = self.unary_decider()?;
        match unary_kind {
            UnaryKind::Call(with_unary) => self.unary_call(with_unary),
            UnaryKind::Final(with_unary) => self.unary_final(with_unary),
            UnaryKind::Recursive(with_unary) => self.unary_recursive(with_unary),
        }
    }

    fn unary_final(&self, with_unary: bool) -> LoxResult<Unary> {
        let unary = match with_unary {
            true => Some(
                self.token_reader
                    .advance_or(self.expected_next_token_err("unary final"))?
                    .clone(),
            ),
            false => None,
        };
        let val = self
            .token_reader
            .advance_or(self.expected_next_token_err("unary final"))?
            .clone();
        Ok(Unary::Final(unary, val))
    }

    fn unary_recursive(&self, with_unary: bool) -> LoxResult<Unary> {
        let unary = match with_unary {
            true => Some(
                self.token_reader
                    .advance_or(self.expected_next_token_err("unary rec"))?
                    .clone(),
            ),
            false => None,
        };
        let expr = self.parenthesized_expr()?;
        Ok(Unary::Recursive(unary, Box::new(expr)))
    }

    fn unary_call(&self, with_unary: bool) -> LoxResult<Unary> {
        let unary = match with_unary {
            true => Some(
                self.token_reader
                    .advance_or(self.expected_next_token_err("unary call"))?
                    .clone(),
            ),
            false => None,
        };
        let identifier = self.token_reader.advance_if(Token::is_identifier).ok_or(
            self.parsing_err()
                .expected_but_found("identifier", "not identifier")
                .build(),
        )?;
        let args = self.fn_arguments()?;

        Ok(Unary::Call(unary, identifier.clone(), args))
    }

    fn unary_decider(&self) -> LoxResult<UnaryKind> {
        let mut with_op = false;
        let token_1 = self
            .token_reader
            .peek_or(self.expected_next_token_err("Parsing first token of an unary expression"))?;
        let token_2 = self
            .token_reader
            .peek_n(1)
            .ok_or(self.expected_next_token_err("Parsing second token of an unary expression"))?;
        let maybe_token_3 = self.token_reader.peek_n(2);

        if token_1.can_be_unary_op() {
            with_op = true;
            let is_parenthesized_expr = token_2.equals(Punct::LeftParen);
            if is_parenthesized_expr {
                return Ok(UnaryKind::Recursive(with_op));
            }

            let token_3 = maybe_token_3.ok_or(
                self.expected_next_token_err("Parsing third token of an unary expression"),
            )?;
            let is_function_call = token_2.is_identifier() && token_3.equals(Punct::LeftParen);
            if is_function_call {
                return Ok(UnaryKind::Call(with_op));
            }

            return Ok(UnaryKind::Final(with_op));
        }

        let is_parenthesized_expr = token_1.equals(Punct::LeftParen);
        if is_parenthesized_expr {
            return Ok(UnaryKind::Recursive(with_op));
        }

        let is_function_call = token_1.is_identifier() && token_2.equals(Punct::LeftParen);
        if is_function_call {
            return Ok(UnaryKind::Call(with_op));
        }

        if !token_2.is_identifier() && !token_2.is_value() {
            return Ok(UnaryKind::Final(with_op));
        }

        panic!(
            "Not recognized unary :(, [{:?}, {:?}, {:?}]",
            token_1, token_2, maybe_token_3
        );
    }

    fn fn_arguments(&self) -> LoxResult<Vec<Box<Expr>>> {
        let info = "parsing function arguments";
        let mut args = Vec::new();
        let next_token_is_comma = || {
            self.token_reader
                .peek()
                .map(|t| t.equals(Comme))
                .unwrap_or(false)
        };

        self.consume_punct(LeftParen, info)?;
        if self
            .token_reader
            .peek()
            .map(|t| t.equals(RightParen))
            .unwrap_or(false)
        {
            self.token_reader.advance(); // eat the right parenthesis
            return Ok(args);
        }

        while next_token_is_comma() || args.is_empty() {
            if next_token_is_comma() {
                self.token_reader.advance();
            }
            let next_arg = self.expression()?;
            args.push(Box::new(next_arg));
        }

        self.consume_punct(RightParen, info)?;

        Ok(args)
    }

    /// Used only in if statements and while loops
    fn parenthesized_expr(&self) -> LoxResult<Expr> {
        let info = "Processing parenthesized expression";

        self.consume_punct(LeftParen, info)?;
        let expr_inside_parenth = self.expression()?;
        self.consume_punct(RightParen, info)?;

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
        ErrBuilder::new().at(relevant_token.pos).of_type(ParsingErr)
    }

    fn expected_next_token_err(&self, info: &str) -> LoxError {
        self.parsing_err()
            .expected_found_nothing("token")
            .while_(info)
            .build()
    }

    fn consume_punct(&self, expected: Punct, info: &str) -> LoxResult<()> {
        let exp2 = expected.clone(); // TODO: win the borrow checker
        let next = self.token_reader.advance_or(
            self.parsing_err()
                .expected_found_nothing(&expected)
                .while_(info)
                .build(),
        )?;

        if next.equals(expected) {
            return Ok(());
        }

        Err(self
            .parsing_err()
            .expected_but_found(exp2, next)
            .while_(info)
            .build())
    }

    fn consume_kwd(&self, expected: Kwd, info: &str) -> LoxResult<()> {
        let exp2 = expected.clone(); // TODO: win the borrow checker
        let next = self.token_reader.advance_or(
            self.parsing_err()
                .expected_found_nothing(&expected)
                .while_(info)
                .build(),
        )?;

        if next.equals(expected) {
            return Ok(());
        }

        Err(self
            .parsing_err()
            .expected_but_found(exp2, next)
            .while_(info)
            .build())
    }

    fn consume_identifier(&self, info: &str) -> LoxResult<(String, Position)> {
        let token = self.token_reader.advance_or(
            self.parsing_err()
                .expected_found_nothing("identifier")
                .while_(info)
                .build(),
        )?;
        match &token.val {
            TokenValue::Id(id) => Ok((id.clone(), token.pos)),
            _ => self
                .parsing_err()
                .expected_but_found("identifier", token)
                .while_(info)
                .to_result(),
        }
    }
}
