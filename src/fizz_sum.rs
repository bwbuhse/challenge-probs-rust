// https://www.codewars.com/kata/514b92a657cdc65150000006/train/rust
pub fn sol(num: i32) -> i32 {
    (0..num).filter(|i| i % 5 == 0 || i % 3 == 0).sum()
}
