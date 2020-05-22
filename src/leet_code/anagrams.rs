/// https://leetcode.com/explore/challenge/card/may-leetcoding-challenge/536/week-3-may-15th-may-21st/3332/
use std::collections::HashMap;

fn frequency_map(word: String) -> HashMap<u8, u32> {
    let mut map = HashMap::new();

    let bytes = word.bytes().collect::<Vec<u8>>();
    for byte in bytes {
        if let Some(count) = map.remove(&byte) {
            map.insert(byte, count + 1);
        } else {
            map.insert(byte, 1);
        }
    }
    map
}

pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
    // We don't need to actually run these
    if p.len() > s.len() {
        return Vec::new();
    }
    if s == p {
        return vec![0];
    }

    let p_freq_map = frequency_map(p.clone());
    let s_bytes = s.bytes().collect::<Vec<u8>>();
    let mut checked: HashMap<String, bool> = HashMap::new();

    // Go through the anagrams
    let mut indices = Vec::new();
    for i in 0..(s_bytes.len() - p.bytes().len() + 1) {
        let mut word = String::new();
        for j in i..(i + p.bytes().len()) {
            // word += &s_bytes[j].to_string();
            word += std::str::from_utf8(&vec![s_bytes[j]]).unwrap();
        }

        if let Some(res) = checked.get(&word) {
            if *res {
                indices.push(i as i32);
            }
        } else {
            let map = frequency_map(word.clone());

            if map == p_freq_map {
                indices.push(i as i32);
                checked.insert(word, true);
            } else {
                checked.insert(word, false);
            }
        }
    }

    indices
}
