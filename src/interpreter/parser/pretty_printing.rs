use super::expression_structure::*;

fn print_with_pad(text: String, pad: u8) {
        for _i in 0..pad {print!("\t")};
        print!("{}", text);
}

pub trait PrettyPrint {
    fn pretty_print(&self, pad: u8);
}

impl <A> PrettyPrint for Single<A> where A: PrettyPrint {
    fn pretty_print(&self, pad: u8) {
        print_with_pad(" - Single:\n".to_string(), pad);
        self.value.pretty_print(pad + 1);
    }
}

impl <A> PrettyPrint for Many<A> where A: PrettyPrint {
    fn pretty_print(&self, pad: u8) {
        print_with_pad(" - Many: \n".to_string(), pad);
        self.first.pretty_print(pad + 1);

        for (token, a) in &self.rest {
            print_with_pad(format!(" * {:?}\n", token), pad);
            a.pretty_print(pad + 1);
        }

    }
}

impl PrettyPrint for Unary {
    fn pretty_print(&self, pad: u8) {
        print_with_pad(
            format!("[ {:?}, {:?} ]\n", self.op, self.right),
            pad
        );
    } 
}