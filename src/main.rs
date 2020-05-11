mod sock_merchant;
use std::env;

fn main() {
    let mut args = env::args().collect::<Vec<String>>();

    match args.swap_remove(1).as_ref() {
        "sock_merchant" => sock_merchant::run(),
        _ => println!("What'd you do"),
    };
}
