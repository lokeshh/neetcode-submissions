impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut anagrams_map_list = vec![];
        let mut super_list: Vec<Vec<usize>> = vec![];
        for (i, word) in strs.iter().enumerate() {
            let mut map = HashMap::new();
            for c in word.chars() {
                let value = map.entry(c).or_insert(0);
                *value += 1;
            }
            anagrams_map_list.push(map.clone());

            let mut match_found = false;
            for sublist in super_list.iter_mut() {
                if anagrams_map_list[*sublist.get(0).unwrap()] == map {
                    sublist.push(i);
                    match_found = true;
                    break;
                }
            }
            if !match_found {
                super_list.push(vec![i]);
            }
        }
        return super_list.iter().map(|sublist| sublist.iter().map(|i| strs[*i].clone()).collect()).collect();
    }
}