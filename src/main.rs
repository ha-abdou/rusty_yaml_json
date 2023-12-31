pub mod json_parser;
pub mod json_tokenizer;

use crate::json_parser::json_parser;
use crate::json_tokenizer::{debug_json_vec_token, json_tokenizer};

fn main() {
    let str = "{
    \"ss\": [{
        \"sssd\": 123
    }],
}";
    //  \"aa\": [\"lolo\"],
    //   \"key\": [true, false, 111.111, \"sss\"],
    //   \"sss\": 012,
    //  \"sasdasdss\": 1.2,

    // \"key3\": \"value1\",
    // \"key1\": {
    //   \"sub 1\": 111,
    // },
    // \"key5\": null,

    let tokens = json_tokenizer(&str);

    // debug_json_vec_token(&tokens);

    let astree = json_parser(&str, &tokens);

    println!("astree: {astree:#?}");

    // println!("{:?}", str.chars().nth(6).offset());
    // println!("{:?}", &str[5..6]);
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
