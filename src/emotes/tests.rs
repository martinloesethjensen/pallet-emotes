#[cfg(test)]
mod tests {
    use crate::emotes::mapping::get_emotes_from_decimal;

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
    fn pow_math_test() {
        assert_eq!(10_u128.checked_pow(2), Some(100));
    }

    #[test]
    fn emotes_from_decimal_test() {
        let result = get_emotes_from_decimal(1);
        assert_eq!(result, vec!["1f600"]);
        let result = get_emotes_from_decimal(10);
        assert_eq!(result, vec!["1f603", "1f601"]);
        let result = get_emotes_from_decimal(15);
        assert_eq!(result, vec!["1f600", "1f603", "1f604", "1f601"])
    }
}
