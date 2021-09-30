#[cfg(test)]
mod tests {
    use crate::emotes::emotes::get_emote;

    use super::*;

    #[test]
    fn get_emote__returns_none_if_invalid_input() {
        let input = "foo";

        let result = get_emote(input);
        assert_eq!(result.is_none(), true)
    }
    
    #[test]
    fn get_emote__returns_proper_emoji() {
        let input = "🚀";

        let result = get_emote(input);

        assert_eq!(result.is_some(), true);
        let emoji = result.unwrap();

        assert_eq!(emoji.as_str(), "🚀")
    }
}
