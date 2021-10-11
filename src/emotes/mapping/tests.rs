#[cfg(test)]
mod tests {
    use std::iter;
    use codec::alloc::collections::HashMap;
    use frame_support::traits::Len;
    use crate::emotes::mapping::mapping::{ByteString, get_mapping, string_to_byte_vec};

    #[test]
    fn test_mapping() {
        let (unicode_to_byte, byte_to_unicode) = get_mapping();

        let test_unicode_char = String::from("1f440");
        let test_byte_char = String::from("10011110");

        assert_eq!(byte_to_unicode.get(&test_byte_char).unwrap(), &test_unicode_char);
        assert_eq!(string_to_byte_vec(unicode_to_byte.get(&test_unicode_char).unwrap()), string_to_byte_vec(&test_byte_char));
    }
}
