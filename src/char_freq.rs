use std::collections::{BTreeMap, HashMap};

pub fn frequency_sort(s: String) -> String {
    // Calculate the frequency of each eltter
    let mut freq_map: HashMap<u8, u32> = HashMap::new();
    let bytes = s.as_bytes();
    for byte in bytes {
        let count_entry = freq_map.entry(*byte).or_insert(0);
        *count_entry += 1;
    }

    // Sort the letters but by frequency into vectors
    let mut sorted_map: BTreeMap<u32, Vec<u8>> = BTreeMap::new();
    for (letter, count) in freq_map {
        let sorted_entry = sorted_map.entry(count).or_insert(Vec::new());
        sorted_entry.push(letter);
    }

    // Add all the letters into the string
    let mut ret = String::from("");
    for (count, letters) in sorted_map.iter().rev() {
        for letter in letters {
            for _ in 0..*count {
                ret.push(*letter as char);
            }
        }
    }

    ret
}
