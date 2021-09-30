use std::collections::HashMap;

fn get_mapping() -> HashMap<String, u8> {
    let mut mapping = HashMap::new();
    mapping.insert(String::from("🚀"), 0b1);
    return mapping;
}

pub fn get_byte_wise_emote(utf8: String) -> Option<u8> {
    let mapping = get_mapping();
    let result = mapping.get(&utf8);

    return match result {
        Some(&val) => Option::Some(val),
        None => Option::None,
    };
}
