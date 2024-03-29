use super::structure::*;

fn print_with_pad(text: String, pad: u8, newline: bool) {
    for _i in 0..pad {
        print!("\t")
    }
    print!("{}", text);

    if newline {
        print!("\n")
    }
}

pub trait PrettyPrint {
    fn pretty_print(&self, pad: u8);
}

impl PrettyPrint for Program {
    fn pretty_print(&self, pad: u8) {
        println!("Program: ");
        for stmt in self.iter() {
            stmt.pretty_print(pad + 1);
            print!("\n");
        }
    }
}

impl PrettyPrint for Statement {
    fn pretty_print(&self, pad: u8) {
        print_with_pad(self.type_name() + ":", pad, true);
        match self {
            Self::Class(_) => {
                print_with_pad("class definition".to_string(), pad, true);
            }
            Self::Return(expr) => {
                print_with_pad("return of:".to_string(), pad, false);
                expr.pretty_print(pad + 1)
            }
            Self::Let(lval, rval) => {
                print_with_pad(format!("{:?}", lval.identifier), pad, true);
                rval.expr.pretty_print(pad + 1);
            }
            Self::Print(expr) => expr.pretty_print(pad + 1),
            Self::Expr(expr) => expr.pretty_print(pad + 1),
            Self::If(cond, prog) => {
                cond.pretty_print(pad + 1);
                prog.pretty_print(pad + 1)
            }
            Self::WhileLoop(cond, prog) => {
                cond.pretty_print(pad + 1);
                prog.pretty_print(pad + 1)
            }
            Self::Fun(_, function_definition) => {
                function_definition
                    .args
                    .iter()
                    .for_each(|a| print_with_pad(format!("{:?}", a), pad + 1, true));

                function_definition.body.pretty_print(pad + 1);
            }
        }
    }
}

impl PrettyPrint for Expr {
    fn pretty_print(&self, pad: u8) {
        print_with_pad(self.type_name(), pad, true);
        match self {
            Self::Eqlty(eqlty) => eqlty.pretty_print(pad + 1),
            Self::Call(_, _) => {
                panic!("todo: impl")
            }
        }
    }
}

impl PrettyPrint for Eqlty {
    fn pretty_print(&self, pad: u8) {
        print_with_pad(format!(" - {}:", self.type_name()), pad, true);
        self.first.pretty_print(pad + 1);

        for (token, a) in &self.rest {
            print_with_pad(format!(" * {:?}", token), pad, true);
            a.pretty_print(pad + 1);
        }
    }
}

impl PrettyPrint for Comp {
    fn pretty_print(&self, pad: u8) {
        print_with_pad(format!(" - {}:", self.type_name()), pad, true);
        self.first.pretty_print(pad + 1);

        for (token, a) in &self.rest {
            print_with_pad(format!(" * {:?}", token), pad, true);
            a.pretty_print(pad + 1);
        }
    }
}

impl PrettyPrint for Term {
    fn pretty_print(&self, pad: u8) {
        print_with_pad(format!(" - {}:", self.type_name()), pad, true);
        self.first.pretty_print(pad + 1);

        for (token, a) in &self.rest {
            print_with_pad(format!(" * {:?}", token), pad, true);
            a.pretty_print(pad + 1);
        }
    }
}

impl PrettyPrint for Factor {
    fn pretty_print(&self, pad: u8) {
        print_with_pad(format!(" - {}:", self.type_name()), pad, true);
        self.first.pretty_print(pad + 1);

        for (token, a) in &self.rest {
            print_with_pad(format!(" * {:?}", token), pad, true);
            a.pretty_print(pad + 1);
        }
    }
}

impl PrettyPrint for Unary {
    fn pretty_print(&self, pad: u8) {
        match self {
            Self::Final(op, val) => {
                print_with_pad(format!("[ {:?} | {:?} ]", op, val), pad + 1, true)
            }
            Self::Recursive(op, expr) => {
                print_with_pad(format!("{:?}", op), pad, true);
                expr.pretty_print(pad + 1)
            }
            Self::Call(op, token, expr) => {
                todo!()
            }
        }
    }
}
