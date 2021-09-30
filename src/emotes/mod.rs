use frame_support::decl_error;

mod tests;

pub mod emotes {
    use emojis::Emoji;

    //This function takes a string as input argmuent and returns and Emoji type if the input is valid
    pub fn get_emote(input: &str) -> Option<Emoji> {
        let emoji = emojis::lookup(input);

        match emoji {
            Some(e) => std::option::Option::Some(e),
            None => None,
        }
    }

    fn emoji_is_supported(emoji: &Emoji) {}
}
