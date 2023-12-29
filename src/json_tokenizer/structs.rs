use std::fmt;

pub struct JsonToken {
    pub token_type: JsonTokenTypes,
    pub at: usize,
}

pub enum JsonTokenTypes {
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    Colon,
    Comma,
    DoubleQuote,
    Null,
    Number,
    True,
    False,
}

impl fmt::Debug for JsonTokenTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            JsonTokenTypes::OpenBrace => write!(f, "OpenBrace"),
            JsonTokenTypes::CloseBrace => write!(f, "CloseBrace"),
            JsonTokenTypes::OpenBracket => write!(f, "OpenBracket"),
            JsonTokenTypes::CloseBracket => write!(f, "CloseBracket"),
            JsonTokenTypes::Colon => write!(f, "Colon"),
            JsonTokenTypes::Comma => write!(f, "Comma"),
            JsonTokenTypes::DoubleQuote => write!(f, "DoubleQuote"),
            JsonTokenTypes::Null => write!(f, "Null"),
            JsonTokenTypes::Number => write!(f, "Number"),
            JsonTokenTypes::True => write!(f, "True"),
            JsonTokenTypes::False => write!(f, "False"),
        }
    }
}
