
pub mod json_tokenizer;
pub mod json_parser;


use crate::json_tokenizer::{json_tokenizer, debug_json_vec_token};
use crate::json_parser::json_parser;

fn main() {
    let str = "{
  \"key3\": \"value1\",
  \"key\": [true, false, 111.111, \"sss\"],
  \"key5\": null,
}";
//  \"aa\": [\"lolo\"],
//   \"sss\": 012,
//  \"sasdasdss\": 1.2,
    let tokens = json_tokenizer(&str);
    
    debug_json_vec_token(&tokens);
    
    let astree = json_parser(&str, &tokens);

    println!("astree: {astree:#?}");
}

// pub fn to_yaml_one_passe_parser (str: &str) {
//     let mut indent = -1;
//     let newStr = String::new();
//     let mut i = 0;

//     let ss = &str[1..3];

//     while i < str.len() {
//         let char = str.chars().nth(i).unwrap();

//         if char.is_whitespace() {
//             i += 1;
//             continue;
//         }

//         match char {
//             '{' => {
//                 indent += 1;
//             }
//             '}' => {
//                 indent -= 1;
//             }
//             '"' => {
//                 // get closing one
//             }
//             ':' => {
//                 // just add it
//             }
//             ',' => {
//                 // new line
//             }
//             _ => {
//                 // numbers
//                 // true
//                 // false
//                 // null
//                 // array
//             }
//         }

//         i += 1;
//     }
// }

/*

ts strip ts
todo number

data selector


lexer:


grammer

lexical analys
    tokenizer


{
    name: "toto",
    aa: ["lolo"]
}


name: "toto"
aa:
  - "lolo"


types:

Object = {
    identifar: Object | Array | String | number | boolean
}

Array<Array | String | number | boolean>

String
number
boolean


*/

// '"' => {
