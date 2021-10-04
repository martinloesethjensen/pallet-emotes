mod mapping;
mod tests;

pub mod emotes {
    pub fn get_emotes_from_decimal(input: u128) -> Option<String> {
        match super::mapping::get_emotes_from_decimal(input) {
            Some(value) => Option::Some(value),
            None => Option::None,
        } 
    }

    // //This function takes a string as input argmuent and returns and Emoji type if the input is valid
    // pub fn get_emotes(input: &str) -> Option<Emoji> {
    //     // We need to iterate over the input with delimiter of ","
    //     // We should expect unicode

    //     let emoji = emojis::lookup(input);

    //     match emoji {
    //         Some(e) => {
    //             if emoji_is_supported(&e) {
    //                 return std::option::Option::Some(e);
    //             }
    //             return None;
    //         }
    //         None => return None,
    //     }

    //     //emoji_is_supported(&emoji);
    // }

    // fn emoji_is_supported(emoji: &Emoji) -> bool {
    //     match get_emotes_from_decimal(emoji) {
    //         Some(_) => true,
    //         None => false,
    //     }
    // }
}
