mod add_two_numbers;
mod anagrams;
mod sock_merchant;
mod two_sum;
use std::env;

fn main() {
    let mut args = env::args().collect::<Vec<String>>();

    if args.len() > 1 {
        match args.swap_remove(1).as_ref() {
            "sock_merchant" => sock_merchant::run(),
            "two_sum" => println!("{:?}", two_sum::two_sum(vec![2, 7, 11, 15], 9)),
            "add_two_numbers" => {
                // Set up the first linked list
                let mut l1n1 = Box::new(add_two_numbers::ListNode::new(2));
                let mut l1n2 = Box::new(add_two_numbers::ListNode::new(4));
                let l1n3 = Box::new(add_two_numbers::ListNode::new(3));

                l1n1.next = Some(l1n2.to_owned());
                l1n2.next = Some(l1n3.to_owned());

                // Set up the second linked list
                let mut l2n1 = Box::new(add_two_numbers::ListNode::new(5));
                let mut l2n2 = Box::new(add_two_numbers::ListNode::new(6));
                let l2n3 = Box::new(add_two_numbers::ListNode::new(4));

                l2n1.next = Some(l2n2.to_owned());
                l2n2.next = Some(l2n3.to_owned());

                // Run the program
                println!(
                    "{:?}",
                    add_two_numbers::add_two_numbers(Some(l1n1), Some(l2n1))
                )
            }
            "anagrams" => {
                println!(
                    "{:?}",
                    anagrams::find_anagrams(String::from("abab"), String::from("ab"))
                );
                println!(
                    "{:?}",
                    anagrams::find_anagrams(String::from("cbaebabacd"), String::from("abc"))
                );
                println!(
                    "{:?}",
                    anagrams::find_anagrams(
                        String::from("aaaaaaaaaa"),
                        String::from("aaaaaaaaaaaaa")
                    )
                );
                println!(
                    "{:?}",
                    anagrams::find_anagrams(String::from("aaa"), String::from("aaa"))
                );
            }
            _ => println!("Invalid program name"),
        };
    } else {
        println!("You need to pass program that you want to run as an arg");
    }
}
