#[cfg(test)]
mod tests {
    //use crate::emotes::emotes::get_emotes;
    use crate::emotes::emotes::get_emotes_from_decimal;

    use super::*;

    // #[test]
    // fn get_emote__returns_none_if_invalid_input() {
    //     let input = "foo";

    //     let result = get_emotes(input);
    //     assert_eq!(result.is_none(), true)
    // }

    // #[test]
    // fn get_emote__returns_none_if_emoji_is_not_supported() {
    //     let input = "👨🏾‍🦱";

    //     let result = get_emotes(input);
    //     assert_eq!(result.is_none(), true);
    // }

    // #[test]
    // fn get_emote__returns_proper_emoji() {
    //     let input = "💩";

    //     let result = get_emotes(input);

    //     assert_eq!(result.is_some(), true);
    //     let emoji = result.unwrap();

    //     println!("{}", emoji);

    //     assert_eq!(emoji.as_str(), "💩");
    // }

    #[test]
    fn simple_binary_test() {
        assert_eq!(format!("{:b}", 1), "1");
        assert_eq!(format!("{:b}", 2), "10");
        assert_eq!(format!("{:b}", 4), "100");
        assert_eq!(format!("{:b}", 8), "1000");
        assert_eq!(format!("{:b}", 10), "1010");
        assert_eq!(format!("{:b}", 5), "101");
    }

    #[test]
    fn test_emotes_from_decimal() {
        let result = get_emotes_from_decimal(1).unwrap_or("0".to_string());
        assert_eq!(result, "1f600");

        let result = get_emotes_from_decimal(4).unwrap_or("0".to_string());
        println!("{}", result);

        let a = 1; // 001
        let b = 4; // 100

        let c = b & a; // 010
        println!("{:b}", c);
    }
}
