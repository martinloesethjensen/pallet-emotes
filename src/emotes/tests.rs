#[cfg(test)]
mod tests {
    use std::iter;
    use codec::alloc::collections::HashMap;
    use frame_support::traits::Len;
    use crate::emotes::mapping::{NftStorage};

    #[test]
    fn example() {
        let mut storage = NftStorage::new();
        let emotes = storage.get_emotes();

        assert_eq!(emotes.len(), 0);

        let input1 = vec![true];
        storage.emote(String::from("0x1"), &input1);

        let emotes = storage.get_emotes();
        assert_eq!(emotes.len(), 1);
        assert_eq!(emotes.get(&String::from("0x1")).unwrap(), &vec![true])
    }


    fn xor_vec(v1: &Vec<bool>, v2: &Vec<bool>) -> Vec<bool> {
        let cmp = |(x1, x2)| {
            x1 ^ x2
        };
        v1.iter()
            .zip(v2.iter().chain(iter::repeat(&false)))
            .map(cmp)
            .collect()
    }
}
