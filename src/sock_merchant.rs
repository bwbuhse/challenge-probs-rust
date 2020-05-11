use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Problem 1:
/// https://www.hackerrank.com/challenges/sock-merchant/problem?h_l=interview&playlist_slugs%5B%5D=interview-preparation-kit&playlist_slugs%5B%5D=warmup

pub fn run() {
    // Set-up the file and read in its lines
    let file =
        File::open("/home/ben/projects/interview-probs-rust/inputs/sock_merchant.dat").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(|l| l.unwrap());

    // Parse the lines
    // let _n = lines.next().unwrap().parse::<usize>().unwrap();
    let _n = lines.next();
    let socks = lines
        .next()
        .unwrap()
        .split(" ")
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    // Figure out how many pairs of socks there are
    // Check if ar[x] is in already the set
    //   - If it is, remove it and increment counter
    //   - If not, add it to the set
    let mut set: HashSet<u32> = HashSet::new();
    let mut counter = 0;

    for sock in socks {
        if set.contains(&sock) {
            set.remove(&sock);
            counter += 1;
        } else {
            set.insert(sock);
        }
    }

    println!("Found {} pairs of socks", counter);
}
