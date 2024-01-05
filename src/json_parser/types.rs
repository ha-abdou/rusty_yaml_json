#[derive(Debug)]
pub struct JsonNodeString {
    pub start: usize,
    pub end: usize,
    pub value: String,
}

#[derive(Debug)]
pub struct JsonNodeNull {
    pub start: usize,
}

#[derive(Debug)]
pub struct JsonNodeNumber {
    pub start: usize,
    pub end: usize,
    pub value: String,
}

#[derive(Debug)]
pub struct JsonNodeBoolean {
    pub start: usize,
    pub value: bool,
}

#[derive(Debug)]
pub struct JsonNodeArray {
    pub start: usize,
    pub end: usize,
    pub value: Vec<JsonNodeObjectValues>,
}

#[derive(Debug)]
pub struct JsonNodeObject {
    pub start: usize,
    pub end: usize,
    pub value: Box<Vec<JsonNodeObjectProprty>>,
}

#[derive(Debug)]
pub struct JsonNodeObjectProprty {
    pub start: usize,
    pub end: usize,
    pub key: String,
    pub value: JsonNodeObjectValues,
}

#[derive(Debug)]
pub enum JsonNodeObjectValues {
    JsonNodeString(JsonNodeString),
    JsonNodeNull(JsonNodeNull),
    JsonNodeNumber(JsonNodeNumber),
    JsonNodeBoolean(JsonNodeBoolean),
    JsonNodeArray(JsonNodeArray),
    JsonNodeObject(JsonNodeObject),
    EndOfObject
}
