mod sock_merchant;
use std::env;

fn main() {
    let mut args = env::args().collect::<Vec<String>>();

    if args.len() > 1 {
        match args.swap_remove(1).as_ref() {
            "sock_merchant" => sock_merchant::run(),
            _ => println!("Invalid program name"),
        };
    } else {
        println!("You need to pass program that you want to run as an arg");
    }
}
