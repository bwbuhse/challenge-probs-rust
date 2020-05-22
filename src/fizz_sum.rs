pub fn sol(num: i32) -> i32 {
    (0..num).filter(|i| i % 5 == 0 || i % 3 == 0).sum()
}
