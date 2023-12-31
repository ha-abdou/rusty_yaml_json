pub mod structs;

pub use structs::{JsonToken, JsonTokenTypes};

pub fn json_tokenizer(str: &str) -> Vec<JsonToken> {
    let mut tokens = Vec::new();
    let mut chars = str.chars().peekable();
    let mut i = 0;

    loop {
        if let Some(char) = chars.peek() {
            if let Some(ch) = str.chars().peekable().nth(i) {
                if char != &ch {
                    println!(
                        "json_tokenizer mismatch at: {:?} betwin {:?} {:?}",
                        i, char, ch
                    );
                    debug_json_vec_token(&tokens);
                    panic!("json_tokenizer mismatch");
                }
            }

            match char {
                '{' => {
                    tokens.push(creat_json_token(JsonTokenTypes::OpenBrace, i));
                    chars.next();
                    i += 1;
                }
                '}' => {
                    tokens.push(creat_json_token(JsonTokenTypes::CloseBrace, i));
                    chars.next();
                    i += 1;
                }
                '[' => {
                    tokens.push(creat_json_token(JsonTokenTypes::OpenBracket, i));
                    chars.next();
                    i += 1;
                }
                ']' => {
                    tokens.push(creat_json_token(JsonTokenTypes::CloseBracket, i));
                    chars.next();
                    i += 1;
                }
                ':' => {
                    tokens.push(creat_json_token(JsonTokenTypes::Colon, i));
                    chars.next();
                    i += 1;
                }
                ',' => {
                    tokens.push(creat_json_token(JsonTokenTypes::Comma, i));
                    chars.next();
                    i += 1;
                }
                '"' => {
                    tokens.push(creat_json_token(JsonTokenTypes::DoubleQuote, i));
                    chars.next();
                    i += 1;

                    while let Some(char) = chars.next() {
                        i += 1;
                        if char == '"' {
                            tokens.push(creat_json_token(JsonTokenTypes::DoubleQuote, i));
                            break;
                        }
                    }
                }
                'n' => {
                    tokens.push(creat_json_token(JsonTokenTypes::Null, i));
                    chars.nth(3);
                    i += 4;
                }
                't' => {
                    tokens.push(creat_json_token(JsonTokenTypes::True, i));
                    chars.nth(3);
                    i += 4;
                }
                'f' => {
                    tokens.push(creat_json_token(JsonTokenTypes::False, i));
                    chars.nth(4);
                    i += 5;
                }
                char => {
                    if char.is_digit(10) {
                        let char_start = i;

                        let mut chars2 = str.chars().peekable();
                        chars2.nth(char_start);
                        let mut end = char_start;

                        while let Some(char) = chars2.peek() {
                            if !(*char).is_digit(10) && *char != '.' {
                                break;
                            }
                            end += 1;
                            chars2.next();
                        }

                        tokens.push(creat_json_token(JsonTokenTypes::Number, char_start));
                        i = end + 1;
                        chars.nth(end - char_start);
                    } else {
                        i += 1;
                        chars.next();
                    }
                }
            }
        } else {
            break;
        }
    }
    tokens
}

pub fn debug_json_token(token: &JsonToken) {
    println!("token_type: {:?}, at: {}", token.token_type, token.at);
}

pub fn debug_json_vec_token(tokens: &Vec<JsonToken>) {
    for token in tokens {
        debug_json_token(token);
    }
}

fn creat_json_token(token_type: JsonTokenTypes, at: usize) -> JsonToken {
    JsonToken { token_type, at }
}
