use std::collections::HashSet;

// https://www.codewars.com/kata/5277c8a221e209d3f6000b56
pub fn is_valid(s: &str) -> bool {
    let openers: HashSet<char> = ['(', '[', '{'].iter().cloned().collect();
    let closers: HashSet<char> = [')', ']', '}'].iter().cloned().collect();

    let mut stk = Vec::new();

    for ch in s.chars() {
        if openers.contains(&ch) {
            stk.push(ch);
            println!("push: {}", ch);
        } else if closers.contains(&ch) {
            match stk.pop() {
                Some(popped) => {
                    if popped != flip(ch) {
                        return false;
                    }
                }
                None => return false,
            }
        }
    }

    stk.is_empty()
}

fn flip(ch: char) -> char {
    match ch {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        _ => panic!("You messed up. {} is not valid in input to is_valid", ch),
    }
}
