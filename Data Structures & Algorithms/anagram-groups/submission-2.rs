use itertools::Itertools;


impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut anagrams_hash: HashMap<String, Vec<String>> = HashMap::new();
        for word in strs {
            let sorted_word: String = word.chars().sorted().collect();
            anagrams_hash.entry(sorted_word).or_insert_with(Vec::new).push(word);
        }
        anagrams_hash.into_values().collect()
    }
}