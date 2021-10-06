#[cfg(test)]
mod tests {
    use crate::emotes::mapping::get_emotes_from_decimal;

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
