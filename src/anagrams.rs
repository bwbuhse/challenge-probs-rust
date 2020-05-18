/// https://leetcode.com/explore/challenge/card/may-leetcoding-challenge/536/week-3-may-15th-may-21st/3332/
pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
    if p.len() > s.len() {
        return Vec::new();
    }

    let mut p_letters = p.bytes().collect::<Vec<u8>>().to_ascii_lowercase();
    p_letters.sort();
    let s_letters = s.bytes().collect::<Vec<u8>>().to_ascii_lowercase();

    // Go through the anagrams
    let mut indices = Vec::new();
    for i in 0..s_letters.len() {
        let mut p_clone = p_letters.clone(); // So we can just re-use p_letters every iteration
        for j in i..s_letters.len() {
            if let Some(letter) = p_clone.iter().position(|x| *x == s_letters[j]) {
                p_clone.swap_remove(letter);
            } else {
                break;
            }
        }
        if p_clone.is_empty() {
            indices.push(i as i32);
        }
    }

    indices
}
