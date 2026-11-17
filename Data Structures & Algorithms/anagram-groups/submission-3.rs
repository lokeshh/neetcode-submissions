use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<Vec<char>, Vec<String>> = HashMap::new();

        for word in strs {
            let mut key: Vec<char> = word.chars().collect();
            key.sort_unstable();
            map.entry(key).or_default().push(word);
        }

        map.into_values().collect()
    }
}