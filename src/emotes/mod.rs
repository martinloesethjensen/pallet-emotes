use frame_support::decl_error;

mod mapping;
mod tests;

pub mod emotes {
    use emojis::Emoji;

    use super::mapping::get_byte_wise_emote;

    //This function takes a string as input argmuent and returns and Emoji type if the input is valid
    pub fn get_emote(input: &str) -> Option<Emoji> {
        let emoji = emojis::lookup(input);

        match emoji {
            Some(e) => {
                if emoji_is_supported(&e) {
                    return std::option::Option::Some(e);
                }
                return None;
            }
            None => return None,
        }

        //emoji_is_supported(&emoji);
    }

    fn emoji_is_supported(emoji: &Emoji) -> bool {
        match get_byte_wise_emote(emoji.to_string()) {
            Some(_) => true,
            None => false,
        }
    }
}
