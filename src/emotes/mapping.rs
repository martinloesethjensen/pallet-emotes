use std::collections::HashMap;

// TODO: store binary instead of String as key
fn populate_emojis() -> HashMap<u128, String> {
    let mut mapping = HashMap::new();
    mapping.insert(1u128, String::from("1f600"));
    mapping.insert(10u128, String::from("1f603"));
    mapping.insert(100u128, String::from("1f604"));
    mapping.insert(1000u128, String::from("1f601"));
    mapping.insert(10000u128, String::from("1f606"));
    return mapping;
}

pub fn get_emotes_from_decimal(decimal: u128) -> Vec<String> {
    let mapping = populate_emojis();

    // decimal to binary string representation
    let binary = format!("{:b}", decimal);

    let _binary_u128 = binary.parse::<u128>().unwrap_or(0);

    let mut emotes: Vec<String> = Vec::new();

    let reversed_binary = binary.chars().rev();
    for (i, bit) in reversed_binary.enumerate() {
        match bit {
            '1' => {
                // We check with the index to see how much the binary u128 should be.
                // If we are at the very first index, then it's `1` we need to query for.
                // Else we do `10 ^ i` to get our bit representation for the mapping.
                let binary_query = if i == 0 { 1 } else { 10_u128.checked_pow(i as u32).unwrap_or(u128::default()) };

                match mapping.get(&binary_query) {
                    Some(val) => emotes.push(val.into()),
                    None => {
                        // Something went wrong with querying for emojis.
                        // Skip for now and don't do anything.
                        // TODO: maybe throw error in pallet `NotFoundEmote`
                    }
                }
            }
            _ => {}
        }
    }

    emotes
}
