mod code_wars;
mod hacker_rank;
mod leet_code;

use std::env;

fn main() {
    let mut args = env::args().collect::<Vec<String>>();

    if args.len() == 2 || args.len() == 3 {
        if args.len() == 3 {
            match args.swap_remove(1).as_ref() {
                "hacker_rank" => hacker_rank(args.swap_remove(1).as_ref()),
                "leet_code" => leet_code(args.swap_remove(1).as_ref()),
                "code_wars" => code_wars(args.swap_remove(1).as_ref()),
                _ => println!("{} is an invalid website name.", args.swap_remove(1)),
            }
        } else {
            println!(
                "You need to pass the challenge from {} that you want to run.",
                args.swap_remove(1)
            );
            println!("Please don't pass any extra arguments");
        }
    } else {
        println!("Please pass arguments for which website and which challenge to run.")
    }
}

// These are functions to match within the challengs that belong to a website
fn hacker_rank(challenge: &str) {
    match challenge {
        "sock_merchant" => hacker_rank::sock_merchant::run(),
        _ => println!("Invalid challenge name"),
    }
}

fn leet_code(challenge: &str) {
    match challenge {
        "two_sum" => println!("{:?}", leet_code::two_sum::two_sum(vec![2, 7, 11, 15], 9)),
        "add_two_numbers" => {
            // Set up the first linked list
            let mut l1n1 = Box::new(leet_code::add_two_numbers::ListNode::new(2));
            let mut l1n2 = Box::new(leet_code::add_two_numbers::ListNode::new(4));
            let l1n3 = Box::new(leet_code::add_two_numbers::ListNode::new(3));

            l1n1.next = Some(l1n2.to_owned());
            l1n2.next = Some(l1n3.to_owned());

            // Set up the second linked list
            let mut l2n1 = Box::new(leet_code::add_two_numbers::ListNode::new(5));
            let mut l2n2 = Box::new(leet_code::add_two_numbers::ListNode::new(6));
            let l2n3 = Box::new(leet_code::add_two_numbers::ListNode::new(4));

            l2n1.next = Some(l2n2.to_owned());
            l2n2.next = Some(l2n3.to_owned());

            // Run the program
            println!(
                "{:?}",
                leet_code::add_two_numbers::add_two_numbers(Some(l1n1), Some(l2n1))
            )
        }
        "anagrams" => {
            println!(
                "{:?}",
                leet_code::anagrams::find_anagrams(String::from("abab"), String::from("ab"))
            );
            println!(
                "{:?}",
                leet_code::anagrams::find_anagrams(String::from("cbaebabacd"), String::from("abc"))
            );
            println!(
                "{:?}",
                leet_code::anagrams::find_anagrams(
                    String::from("aaaaaaaaaa"),
                    String::from("aaaaaaaaaaaaa")
                )
            );
            println!(
                "{:?}",
                leet_code::anagrams::find_anagrams(String::from("aaa"), String::from("aaa"))
            );
        }
        "char_freq" => println!(
            "{}",
            leet_code::char_freq::frequency_sort(String::from("tree"))
        ),
        _ => println!("Invalid challenge name"),
    }
}

fn code_wars(challenge: &str) {
    match challenge {
        "fizz_sum" => println!("{}", code_wars::fizz_sum::sol(10)),
        "desc_order" => println!("{}", code_wars::desc_order::sol(145263)),
        "valid_braces" => {
            println!("{}", code_wars::valid_braces::is_valid("[({})](]")); // False
            println!("{}", code_wars::valid_braces::is_valid("([{}])")); // True
        }
        "tribonacci" => {
            println!("{:?}", code_wars::tribonacci::seq(&[0., 1., 1.], 10)); //  vec![0., 1., 1., 2., 4., 7., 13., 24., 44., 81.]
        }
        "get_middle_char" => println!("{}", code_wars::get_middle_char::get_middle("test")),
        "bit_count" => println!("{}", code_wars::bits::count_ones(1234)),
        _ => println!("Invalid challenge name"),
    }
}

#[test]
fn test_fizz_sum() {
    assert_eq!(code_wars::fizz_sum::sol(10), 23);
    assert_eq!(code_wars::fizz_sum::sol(11), 33);
    assert_eq!(code_wars::fizz_sum::sol(6), 8);
}

#[test]
fn test_tribonacci() {
    assert_eq!(
        code_wars::tribonacci::seq(&[0., 1., 1.], 10),
        vec![0., 1., 1., 2., 4., 7., 13., 24., 44., 81.]
    );
    assert_eq!(
        code_wars::tribonacci::seq(&[1., 0., 0.], 10),
        vec![1., 0., 0., 1., 1., 2., 4., 7., 13., 24.]
    );
    assert_eq!(
        code_wars::tribonacci::seq(&[0., 0., 0.], 10),
        vec![0., 0., 0., 0., 0., 0., 0., 0., 0., 0.]
    );
    assert_eq!(
        code_wars::tribonacci::seq(&[1., 2., 3.], 10),
        vec![1., 2., 3., 6., 11., 20., 37., 68., 125., 230.]
    );
    assert_eq!(
        code_wars::tribonacci::seq(&[3., 2., 1.], 10),
        vec![3., 2., 1., 6., 9., 16., 31., 56., 103., 190.]
    );
    assert_eq!(code_wars::tribonacci::seq(&[1., 1., 1.], 1), vec![1.]);
    assert_eq!(code_wars::tribonacci::seq(&[300., 200., 100.], 0), vec![]);
    assert_eq!(
        code_wars::tribonacci::seq(&[0.5, 0.5, 0.5], 30),
        vec![
            0.5, 0.5, 0.5, 1.5, 2.5, 4.5, 8.5, 15.5, 28.5, 52.5, 96.5, 177.5, 326.5, 600.5, 1104.5,
            2031.5, 3736.5, 6872.5, 12640.5, 23249.5, 42762.5, 78652.5, 144664.5, 266079.5,
            489396.5, 900140.5, 1655616.5, 3045153.5, 5600910.5, 10301680.5
        ]
    );
}

#[test]
fn test_get_middle_char() {
    assert_eq!(code_wars::get_middle_char::get_middle("test"), "es");
    assert_eq!(code_wars::get_middle_char::get_middle("testing"), "t");
    assert_eq!(code_wars::get_middle_char::get_middle("middle"), "dd");
    assert_eq!(code_wars::get_middle_char::get_middle("A"), "A");
    assert_eq!(code_wars::get_middle_char::get_middle("of"), "of");
}

#[test]
fn test_bit_count() {
    assert_eq!(code_wars::bits::count_ones(0), 0);
    assert_eq!(code_wars::bits::count_ones(4), 1);
    assert_eq!(code_wars::bits::count_ones(7), 3);
    assert_eq!(code_wars::bits::count_ones(9), 2);
    assert_eq!(code_wars::bits::count_ones(10), 2);
}
