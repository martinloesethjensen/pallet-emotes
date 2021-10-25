use std::collections::HashMap;
use std::fs;
use serde::{Serialize, Deserialize};
use serde_json::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct Row {
    pub byte: ByteString,
    pub unicode: String,
}

pub type ByteString = String;

// POC for using a HashMap as a mapping between bytewise storage and unicode
pub fn get_mapping() -> (HashMap<String, ByteString>, HashMap<ByteString, String>) {
    let f = fs::read_to_string("./src/emotes/mapping/mapping.json").expect("Unable to read file");

    let mapping: Result<Vec<Row>> = serde_json::from_str(&f);

    let mut unicode_to_byte: HashMap<String, ByteString> = HashMap::new();
    let mut byte_to_unicode: HashMap<ByteString, String> = HashMap::new();


    mapping.unwrap().iter().for_each(|row| {
        byte_to_unicode.insert(row.byte.clone(), row.unicode.clone());
        unicode_to_byte.insert(row.unicode.clone(), row.byte.clone());
    });

    (unicode_to_byte, byte_to_unicode)
}


pub fn string_to_byte_vec(byte_str: &ByteString) -> Vec<bool> {
    byte_str.chars().map(|c: char| {
        return match c {
            ('0') => false,
            ('1') => true,
            (_) => panic!("invalid bool string")
        };
    }).collect()
}
