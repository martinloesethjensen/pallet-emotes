use std::collections::HashMap;

use emojis::Emoji;

// TODO: store binary instead of String as key
fn populate_emojis() -> HashMap<String, String> {
    let mut mapping = HashMap::new();
    mapping.insert(String::from("1"), String::from("1f600"));
    mapping.insert(String::from("10"), String::from("1f603"));
    mapping.insert(String::from("100"), String::from("1f604"));
    mapping.insert(String::from("1000"), String::from("1f601"));
    mapping.insert(String::from("10000"), String::from("1f606"));
    return mapping;
}

pub fn get_emotes_from_decimal(decimal: u128) -> Option<String> {
    let mapping = populate_emojis();

    // decimal to binary representation
    let binary = format!("{:b}", decimal);
    //let intval = isize::from_str_radix(bin_idx, 2).unwrap();

    println!("{}", binary);

    let mut emotes: Vec<String> = Vec::new();

    let mut lookup = String::new();

    let reversed_binary = binary.chars().rev();
    for (i, bit) in reversed_binary.enumerate() {
        match bit {
            '1' => {
                let binary_query = format!("1{}", lookup);
                println!("{}", binary_query);
            }
            _ => {}
        }
        lookup.push('0');
    }

    match mapping.get(&binary) {
        Some(val) => Option::Some(val.into()),
        None => Option::None,
    }
}

// pub fn get_bytewise_emote(emote: &Emoji) -> Option<u8> {
//     let mapping = populate_emotijs();
//     let result = mapping.get(emote.as_str());

//     return match result {
//         Some(&val) => Option::Some(val),
//         None => Option::None,
//     };
// }

// pub fn get_emote_from_bytewise(byte: u8) -> Option<Emoji> {
//     let mapping = populate_emotijs();
//     let result = mapping.iter().find(|(_, &val)| return val == byte);

//     return match result {
//         Some(val) => emojis::lookup(val.0),
//         None => Option::None,
//     };
// }
