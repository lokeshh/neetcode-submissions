use itertools::Itertools;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut anagrams_list: Vec<(String, String)> = strs.iter().map(|word| {
            (word.chars().sorted().collect(), word.to_string())
        }).collect();
        anagrams_list.sort_by(|a, b| a.0.cmp(&b.0));
        let mut super_list = vec![];
        let mut sub_list = vec![];
        for (sorted_word, word) in anagrams_list {
            if sub_list.is_empty() {
                sub_list.push((sorted_word, word));
            } else if sub_list[0].0 == sorted_word {
                sub_list.push((sorted_word, word));
            } else {
                super_list.push(sub_list);
                sub_list = vec![(sorted_word, word)];
            }
        }
        if !sub_list.is_empty() {
            super_list.push(sub_list);
        }
        super_list.iter().map(|sub_list| {
            sub_list.iter().map(|(_, word)| word).cloned().collect()
        }).collect()
    }
}

