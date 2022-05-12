use std::cell::Cell;
use super::reader::{ReaderBase, Reader};

pub struct TextReader {
    source: Vec<char>,
    pos: Cell<usize>,
}

impl ReaderBase<char> for TextReader {
    fn from_vec(v: Vec<char>) -> Self {
        TextReader {
            source: v,
            pos: Cell::new(0),
        }
    }

    fn pos(&self) -> usize {
        self.pos.get()
    }

    fn advance(&self) -> Option<&char> {
        let pos = self.pos.get();
        let output = self.source.get(pos);
        if output.is_some() {
            self.pos.set(pos + 1);
        }
        output
    }

    fn peek(&self) -> Option<&char> {
        self.source.get(self.pos.get())
    }

    fn back(&self) -> Option<()> {
        let pos = self.pos.get();
        if pos as i32 - 1 >= 0 {
            self.pos.set(pos - 1);
            return Some(())
        }

        None
    }
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn step_forward() {
//         use super::Reader;
//         use super::ReaderBase;
//         use super::TextReader;

//         let text = "sadfasdfdfs";
//         let r = TextReader::from_vec(text.to_string().chars().collect());
//         assert_eq!(r.get_pos(), 0);
//         assert_eq!(r.advance(), Some('s'));
//         assert_eq!(r.get_pos(), 1);
//     }

//     #[test]
//     fn step_forward_2() {
//         let text = "abcdefgh";
//         let r = TextReader::from_vec(text.to_string());
//         for _ in 0..4 {
//             r.advance();
//         }
//         assert_eq!(r.get_pos(), 4);
//         assert_eq!(r.advance(), Some('e'));
//         assert_eq!(r.get_pos(), 5);
//     }

//     #[test]
//     fn adv_newline() {
//         let text = "ab\ncdef safwr4r 23424 2qr \n*fdsaf";
//         let r = TextReader::new(text.to_string());

//         r.advance_until_newline();
//         assert_eq!(r.get_pos(), 3);
//         assert_eq!(r.advance(), Some('c'));

//         r.advance_until_newline();
//         assert_eq!(r.advance(), Some('*'));

//         r.advance_until_newline();
//         assert_eq!(r.advance(), None);
//     }

//     #[test]
//     fn go_back() {
//         let text = "absadfasd";
//         let r = TextReader::new(text.to_string());

//         assert_eq!(r.advance(), Some('a'));
//         assert_eq!(r.get_pos(), 1);

//         assert_eq!(r.back(), Some(()));
//         assert_eq!(r.back(), None);

//         assert_eq!(r.get_pos(), 0);
//         assert_eq!(r.advance(), Some('a'));
//         assert_eq!(r.get_pos(), 1);
//         assert_eq!(r.advance(), Some('b'));
//         assert_eq!(r.get_pos(), 2);
//     }
// }
