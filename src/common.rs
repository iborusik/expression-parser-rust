use core::fmt;


#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum UTokenType {
    Number,
    Left,
    Right,
    Plus,
    Minus,
    Star,
    Div,
    Pow,
}

#[derive(Clone)]
pub struct UToken {
    pub _type: UTokenType,
    pub _val: Option<String>
}

impl fmt::Display for UTokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match self {
        UTokenType::Number => write!(f, "num"),
        UTokenType::Left => write!(f, "("),
        UTokenType::Right => write!(f, ")"),
        UTokenType::Plus => write!(f, "+"),
        UTokenType::Minus => write!(f, "-"),
        UTokenType::Star =>write!(f, "*"),
        UTokenType::Div => write!(f, "/"),
        UTokenType::Pow => write!(f, "^"),
    }
    }
}

impl UToken {
    pub fn new(t: UTokenType) -> UToken {
        UToken {
            _type: t,
            _val: None
        }
    }
}