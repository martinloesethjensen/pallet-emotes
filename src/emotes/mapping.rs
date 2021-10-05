use std::collections::HashMap;
use std::iter;


//Mocked storage
pub struct NftStorage {
    emotes: HashMap<String, Vec<bool>>,
}


impl NftStorage {
    pub fn new() -> Self {
        Self {
            emotes: HashMap::new()
        }
    }

    pub fn emote(&mut self, address: String, emote: &Vec<bool>) {
        let user_has_already_emoted = self.emotes.contains_key(&address);

        let mut old_emote_vec: &Vec<bool> = &Vec::new();

        //If the user already voted on the NFT we need to fetch is previous emotes
        if user_has_already_emoted {
            old_emote_vec = self.emotes.get(&address).unwrap();
        }

        let new_emote_vec = self.xor_emote_vec(&old_emote_vec, &emote);
        self.emotes.insert(address, new_emote_vec);
    }

    pub fn get_emotes(&self) -> &HashMap<String, Vec<bool>> { &self.emotes }

    fn xor_emote_vec(&self, old: &Vec<bool>, new: &Vec<bool>) -> Vec<bool> {
        //We need to iterative through the vector which is the longest
        let target = old.len() > new.len();

        return match target {
            true => self.xor_vec(old, new),
            false => self.xor_vec(new, old)
        };
    }
    fn xor_vec(&self, v1: &Vec<bool>, v2: &Vec<bool>) -> Vec<bool> {
        v1.iter()
            //we fill the shorter vec with false values
            .zip(v2.iter().chain(iter::repeat(&false)))
            .map(|(x1, x2)| x1 ^ x2)
            .collect()
    }
}







