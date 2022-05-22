use super::structure::*;

fn print_with_pad(text: String, pad: u8) {
    for _i in 0..pad {
        print!("\t")
    }
    print!("{}", text);
}

pub trait PrettyPrint {
    fn pretty_print(&self, pad: u8);
}

impl PrettyPrint for Program {
    fn pretty_print(&self, pad: u8) {
        println!("Program:");
        for stmt in self.iter() {
            match stmt {
                Or2::Opt1(expr) => { println!("\tExpression Stmt:"); expr.pretty_print(pad + 2) },
                Or2::Opt2(print_stmt) => { println!("\tPrint Stmt:"); print_stmt.pretty_print(pad + 2)},
            }
        }
    }
}

impl<A> PrettyPrint for Single<A>
where
    A: PrettyPrint + NamedType,
{
    fn pretty_print(&self, pad: u8) {
        print_with_pad(format!(" - {}:\n", A::type_name()), pad);
        self.value.pretty_print(pad + 1);
    }
}

impl<A> PrettyPrint for Many<A>
where
    A: PrettyPrint + NamedType,
{
    fn pretty_print(&self, pad: u8) {
        print_with_pad(format!(" - {}:\n", A::type_name()), pad);
        self.first.pretty_print(pad + 1);

        for (token, a) in &self.rest {
            print_with_pad(format!(" * {:?}\n", token), pad);
            a.pretty_print(pad + 1);
        }
    }
}

impl PrettyPrint for Unary {
    fn pretty_print(&self, pad: u8) {
        print_with_pad(format!("[ {:?}, {:?} ]\n", self.op, self.right), pad);
    }
}
