impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut s_counter = HashMap::new();
        let mut t_counter = HashMap::new();
        if s.len() != t.len() {
            return false
        }
        for c in s.chars() {
            s_counter.insert(c, s_counter.get(&c).copied().unwrap_or(0) + 1);
        }
        for c in t.chars() {
            t_counter.insert(c, t_counter.get(&c).copied().unwrap_or(0) + 1);
        }
        for (k, v) in s_counter {
            // println!("{} {}", k, v);
            if t_counter.get(&k).copied().unwrap_or(0) != v {
                return false
            }
        }
        true
    }
}
