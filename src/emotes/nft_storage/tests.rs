#[cfg(test)]
mod tests {
    use std::iter;
    use codec::alloc::collections::HashMap;
    use frame_support::traits::Len;
    use crate::emotes::nft_storage::nft_storage::NftStorage;

    #[test]
    fn stores_emote_of_an_address() {
        let mut storage = NftStorage::new();
        let emotes = storage.get_emotes();

        assert_eq!(emotes.len(), 0);

        let input1 = vec![true];
        storage.emote(String::from("0x1"), &input1);

        let emotes = storage.get_emotes();
        assert_eq!(emotes.len(), 1);
        assert_eq!(emotes.get(&String::from("0x1")).unwrap(), &vec![true])
    }
    #[test]
    fn keeps_existing_emotes() {
        let mut storage = NftStorage::new();
        let emotes = storage.get_emotes();

        assert_eq!(emotes.len(), 0);

        storage.emote(String::from("0x1"), &vec![true, false, true]);
        storage.emote(String::from("0x1"), &vec![false, false, false, true]);

        let emotes = storage.get_emotes();
        assert_eq!(emotes.len(), 1);
        assert_eq!(emotes.get(&String::from("0x1")).unwrap(), &vec![true,false,true,true])
    }


}
