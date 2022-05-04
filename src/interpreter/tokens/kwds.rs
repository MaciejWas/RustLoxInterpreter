use crate::interpreter::errors::ErrType::TokenizingErr;
use crate::interpreter::errors::{LoxResult, LoxError};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Kwd {
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,  
    Comment(String)
} 

impl Kwd {
    pub fn is_valid(string: &String) -> bool {
        match Self::from(string, 0) {
            Ok(_) => true,
            _     => false
        }
    }

    pub fn from(string: &String, pos: usize) -> LoxResult<Self> {
        match string.as_str() {
            "and"    => Ok(Self::And),
            "class"  => Ok(Self::Class),
            "else"   => Ok(Self::Else),
            "false"  => Ok(Self::False),
            "fun"    => Ok(Self::Fun),
            "for"    => Ok(Self::For),
            "if"     => Ok(Self::If),
            "print"  => Ok(Self::Print),
            "return" => Ok(Self::Return),
            "super"  => Ok(Self::Super),
            "this"   => Ok(Self::This),
            "true"   => Ok(Self::True),
            "var"    => Ok(Self::Var),
            "while"  => Ok(Self::While),
            _        => LoxError::new_err("Failed to build Kwd from string".to_string(), pos, TokenizingErr)
        }
    }
}

#[cfg(test)]
mod tests {
    use quickcheck::quickcheck;
    use super::Kwd;

    quickcheck! {
        fn quickcheck_kwd(s: String) -> bool {
            let result = Kwd::from(&s, 0);
            if Kwd::is_valid(&s) { 
                return result.is_ok()
            } 

            result.is_err()
        }
    }
}

