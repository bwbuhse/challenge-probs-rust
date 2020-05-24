pub fn get_middle(s: &str) -> &str {
    &s[s.len() / 2 - if s.len() & 0b1 == 0b0 { 1 } else { 0 }..s.len() / 2 + 1]
}
