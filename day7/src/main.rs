use itertools::Itertools;
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

enum _Combinations {
    Fives,
    Fours,
    FullHouse,
    TwoPairs,
    Pairs,
    Cards,
}

// 23456789

fn main() {
    let path = Path::new("test.txt");
    let display = path.display();

    let file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => panic!("Can't open {}: {}", display, e),
    };

    let reader = io::BufReader::new(file);
    let it = reader.lines();

    let _value_hand = move |s: &str| s.chars().sorted();

    for line in it {
        let l = line.unwrap();
        let Some((hand, _bet)) = l.split_whitespace().collect_tuple() else {
            panic!("Can't parse line: {}", l);
        };
        println!("{}", hand.chars().sorted().collect::<String>());
    }
}
