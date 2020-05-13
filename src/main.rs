mod sock_merchant;
mod two_sum;
use std::env;

fn main() {
    let mut args = env::args().collect::<Vec<String>>();

    if args.len() > 1 {
        match args.swap_remove(1).as_ref() {
            "sock_merchant" => sock_merchant::run(),
            "two_sum" => println!("{:?}", two_sum::two_sum(vec![2, 7, 11, 15], 9)),
            _ => println!("Invalid program name"),
        };
    } else {
        println!("You need to pass program that you want to run as an arg");
    }
}
