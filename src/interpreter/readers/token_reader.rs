use crate::interpreter::tokens::Token;

use super::ReaderBase;

use std::cell::Cell;

pub struct TokenReader {
    tokens: Vec<Token>,
    pos: Cell<usize>,
}

impl ReaderBase<Token> for TokenReader {
    fn from_vec(tokens: Vec<Token>) -> Self {
        TokenReader {
            tokens: tokens,
            pos: Cell::new(0),
        }
    }

    fn advance(&self) -> Option<&Token> {
        let pos = self.pos.get();
        let output = self.tokens.get(pos);
        if output.is_some() {
            self.pos.set(pos + 1);
        }
        output
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos())
    }

    fn pos(&self) -> usize {
        self.pos.get()
    }
}

// impl TokenReader {
//     pub fn new(tokens: Vec<Token>) -> Self {
//         TokenReader {
//             tokens: tokens,
//             pos: Cell::new(None),
//         }
//     }

//     pub fn pretty_display_state(&self) {
//         match self.pos.get() {
//             None => {
//                 println!(" --- Not Started --- | {:?} |  {:?} ", self.tokens.get(0), self.tokens.get(1));
//             },
//             Some(i) => {
//                 let fst = if i as i32 - 2 >= 0 { self.tokens.get(i - 2) } else { None };
//                 let snd = if i as i32 - 1 >= 0 { self.tokens.get(i - 1) } else { None };

//                 println!(" {:?} |  {:?} | --- {:?} --- | {:?} |  {:?} ", fst, snd, self.tokens.get(i), self.tokens.get(i+1), self.tokens.get(i+2));
//             }
//         }
//     }

//     pub fn advance(&self) -> Option<&Token> {
//         if self.peek().is_some() {
//             self.step_forward();
//             return self.curr_token();
//         }

//         None
//     }

//     pub fn advance_if(&self, predicate: fn(&Token) -> bool) -> Option<&Token> {
//         let next_token = self.peek()?;
//         if predicate(next_token) {
//             return self.advance();
//         }

//         None
//     }

//     fn step_forward(&self) {
//         match self.pos.get() {
//             None => self.pos.set(Some(0)),
//             Some(i) => self.pos.set(Some(i + 1)),
//         }
//     }

//     pub fn curr_token(&self) -> Option<&Token> {
//         self.tokens.get(self.pos.get()?)
//     }

//     pub fn peek(&self) -> Option<&Token> {
//         match self.pos.get() {
//             None => self.tokens.get(0),
//             Some(i) => self.tokens.get(i + 1),
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::TokenReader;
//     use crate::interpreter::readers::token_reader::Token;

//     fn token_vec(s: &str) -> Vec<Token> {
//         s.chars()
//             .map(|c| Token::from_string(format!("char{}", c), 0).unwrap())
//             .collect()
//     }

//     #[test]
//     fn step_forward() {
//         let tokens = token_vec("abcdefg");
//         let r = TokenReader::new(tokens.clone());

//         assert_eq!(r.curr_token(), None);
//         assert_eq!(r.advance_if(|_token| true), Some(&tokens[0]));
//         assert_eq!(r.curr_token(), Some(&tokens[0]));

//         assert_eq!(r.advance_if(|_token| false), None);
//         assert_eq!(r.curr_token(), Some(&tokens[0]));

//         assert_eq!(r.advance_if(|_token| true), Some(&tokens[1]));
//         assert_eq!(r.curr_token(), Some(&tokens[1]));

//         assert_eq!(r.advance(), Some(&tokens[2]));
//         assert_eq!(r.advance(), Some(&tokens[3]));
//     }

//     #[test]
//     fn step_forward_2() {
//         let tokens = token_vec("abcdefg");
//         let r = TokenReader::new(tokens.clone());

//         assert_eq!(r.advance(), Some(&tokens[0]));
//         assert_eq!(r.advance(), Some(&tokens[1]));
//         assert_eq!(r.advance(), Some(&tokens[2]));
//         assert_eq!(r.advance(), Some(&tokens[3]));

//         assert_eq!(r.curr_token(), Some(&tokens[3]));
//         assert_eq!(r.advance(), Some(&tokens[4]));
//         assert_eq!(r.curr_token(), Some(&tokens[4]));
//     }

//     // #[test]
//     // fn adv_newline() {
//     //     let text = "ab\ncdef safwr4r 23424 2qr \n*fdsaf";
//     //     let r = TextReader::new(text.to_string());

//     //     r.advance_until_newline();
//     //     assert_eq!(r.get_pos(), 3);
//     //     assert_eq!(r.advance(), Some('c'));

//     //     r.advance_until_newline();
//     //     assert_eq!(r.advance(), Some('*'));

//     //     r.advance_until_newline();
//     //     assert_eq!(r.advance(), None);
//     // }

//     // #[test]
//     // fn go_back() {
//     //     let text = "absadfasd";
//     //     let r = TextReader::new(text.to_string());

//     //     assert_eq!(r.advance(), Some('a'));
//     //     assert_eq!(r.get_pos(), 1);

//     //     assert_eq!(r.back(), Some(()));
//     //     assert_eq!(r.back(), None);

//     //     assert_eq!(r.get_pos(), 0);
//     //     assert_eq!(r.advance(), Some('a'));
//     //     assert_eq!(r.get_pos(), 1);
//     //     assert_eq!(r.advance(), Some('b'));
//     //     assert_eq!(r.get_pos(), 2);
//     // }
// }
