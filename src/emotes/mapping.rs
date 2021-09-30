use std::collections::HashMap;

use emojis::Emoji;

fn get_mapping() -> HashMap<String, u8> {
    let mut mapping = HashMap::new();
    mapping.insert(String::from("🚀"), 0b1);
    return mapping;
}

pub fn get_bytewise_emote(emote: &Emoji) -> Option<u8> {
    let mapping = get_mapping();
    let result = mapping.get(emote.as_str());

    return match result {
        Some(&val) => Option::Some(val),
        None => Option::None,
    };
}

pub fn get_emote_from_bitewise(byte: u8) -> Option<Emoji> {
    let mapping = get_mapping();
    let result = mapping.iter().find(|(key, &val)| return val == byte);

    return match result {
        Some(val) => {
            return emojis::lookup(val.0)
        },
        None => Option::None,
    };
}
