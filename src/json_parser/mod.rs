pub mod types;

use crate::{
    json_parser::types::{JsonNodeArray, JsonNodeNumber},
    json_tokenizer::{JsonToken, JsonTokenTypes},
};
pub use types::{
    JsonNodeBoolean, JsonNodeNull, JsonNodeObject, JsonNodeObjectProprty, JsonNodeObjectValues,
    JsonNodeString,
};

pub fn json_parser(str: &str, tokens: &Vec<JsonToken>) -> JsonNodeObject {
    if !matches!(&tokens[0].token_type, JsonTokenTypes::OpenBrace) {
        // todo log where is the error
        panic!("json_parser: first token must be OpenBrace");
    }

    let mut cursor = 1;

    let obj = JsonNodeObject {
        start: 0,
        end: str.len() - 1,
        values: Box::new(get_object_properties(str, tokens, &mut cursor)),
    };

    obj
}

fn get_object_properties(
    str: &str,
    tokens: &Vec<JsonToken>,
    cursor: &mut usize,
) -> Vec<JsonNodeObjectProprty> {
    let mut properties = Vec::new();

    loop {
        let token: &JsonToken = tokens.get(*cursor).unwrap();

        if matches!(token.token_type, JsonTokenTypes::CloseBrace) {
            break;
        }

        let property: JsonNodeObjectProprty = get_next_object_property(&str, tokens, cursor);

        properties.push(property);

        let token: &JsonToken = tokens.get(*cursor).unwrap();

        if matches!(token.token_type, JsonTokenTypes::CloseBrace) {
            break;
        }

        *cursor += 1;
    }

    return properties;
}

fn get_next_object_property(
    str: &str,
    tokens: &Vec<JsonToken>,
    cursor: &mut usize,
) -> JsonNodeObjectProprty {
    let start_token = tokens.get(*cursor).unwrap();
    let start: usize = start_token.at;

    *cursor += 1;

    let end_token = tokens.get(*cursor).unwrap();

    let end: usize = end_token.at;
    let start = start + 1;

    let key = str
        .chars()
        .skip(start)
        .take(end - start - 1)
        .collect::<String>();

    *cursor += 1;

    let token = tokens.get(*cursor).unwrap();

    if !matches!(token.token_type, JsonTokenTypes::Colon) {
        println!("str: {:?}", str[token.at - 5..token.at + 5].to_string());
        println!("key: {key:?}");
        println!("cursor: {cursor:?}");
        println!("type: {:?}", token.token_type);
        panic!("Colon expected")
    }

    *cursor += 1;
    let value = get_next_value(str, tokens, cursor);

    let token = tokens.get(*cursor).unwrap();

    if !matches!(token.token_type, JsonTokenTypes::Comma) {
        *cursor += 1;
    }

    let res = JsonNodeObjectProprty {
        start,
        end: token.at,
        key,
        value,
    };
    res
}

fn get_next_value(str: &str, tokens: &Vec<JsonToken>, cursor: &mut usize) -> JsonNodeObjectValues {
    let token = tokens.get(*cursor).unwrap();

    println!("get_next_value ===> {:?}", token.token_type);

    use JsonNodeObjectValues as JV;
    use JsonTokenTypes as JT;
    let value = match token.token_type {
        JT::DoubleQuote => {
            let start = token.at + 1;
            *cursor += 1;
            let token = tokens.get(*cursor).unwrap();
            let end = token.at;

            let value = str[start..end - 1].to_string();
            let res = JV::JsonNodeString(JsonNodeString { start, end, value });

            res
        }
        JT::Null => {
            let res = JV::JsonNodeNull(JsonNodeNull {
                start: token.at + 1,
            });

            res
        }
        JT::True => {
            let start = token.at;
            let end = token.at + 4;

            let value = str[start..end].to_string();

            let res = JV::JsonNodeBoolean(JsonNodeBoolean {
                start: token.at + 1,
                value: value == "true",
            });

            res
        }
        JT::False => {
            let start = token.at;
            let end = token.at + 5;

            let value = str[start..end].to_string();

            let res = JV::JsonNodeBoolean(JsonNodeBoolean {
                start: token.at + 1,
                value: value != "false",
            });
            res
        }
        JT::Number => {
            let start = token.at;

            *cursor += 1;
            let token = tokens.get(*cursor).unwrap();

            let end = token.at;

            let value = str[start..end].to_string();

            let res = JV::JsonNodeNumber(JsonNodeNumber { start, end, value });

            res
        }
        JT::OpenBracket => {
            let start = token.at;
            let mut value: Vec<JsonNodeObjectValues> = Vec::new();

            loop {
                *cursor += 1;

                let token = tokens.get(*cursor).unwrap();

                println!("match ===> {:?}", token.token_type);

                if matches!(token.token_type, JT::Comma) {
                    // todo hanlde trailing comma
                    *cursor += 1;
                }

                let token = tokens.get(*cursor).unwrap();

                if matches!(token.token_type, JT::CloseBracket) {
                    break;
                }

                let v = get_next_value(str, tokens, cursor);

                if matches!(v, JV::EndOfObject) {
                    break;
                }

                value.push(v);        
            }

            let end = tokens.get(*cursor).unwrap().at;

            let res = JV::JsonNodeArray(JsonNodeArray { start, end, value });

            res
        }
        _ => JV::EndOfObject,
    };

    value
}
