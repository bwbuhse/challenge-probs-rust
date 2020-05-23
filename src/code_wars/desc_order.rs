use std::collections::BTreeMap;

// https://www.codewars.com/kata/5467e4d82edf8bbf40000155
pub fn sol(mut x: u64) -> u64 {
    let mut digit_counts = BTreeMap::new();

    // Get the count of each digit in x
    while x > 0 {
        let count_entry = digit_counts.entry(x % 10).or_insert(0);
        *count_entry += 1;
        x /= 10;
    }

    // Then iterate through the map and add count x's onto res, multiplying by 10 each time
    let mut x_descending = 0;
    for (digit, count) in digit_counts.iter().rev() {
        for _ in 0..*count {
            x_descending *= 10;
            x_descending += *digit;
        }
    }

    x_descending
}
