use regex::Regex;
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let path = Path::new("input.txt");
    let display = path.display();

    let file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => panic!("Can't open {}: {}", display, e),
    };

    let number = Regex::new(r"\d+").unwrap();
    let special = Regex::new(r"[^0-9.]").unwrap();

    let reader = io::BufReader::new(file);
    let mut it = reader.lines();

    let mut sum = 0;
    let mut prev: String = String::new();
    let Some(Ok(mut curr)) = it.next() else {
        panic!("No line found")
    };
    let mut next: String = match it.next() {
        Some(Ok(str)) => str,
        _ => String::new(),
    };

    loop {
        println!("Prev: {}", prev);
        println!("Curr: {}", curr);
        println!("Next: {}", next);

        for m in number.find_iter(&curr) {
            println!("Number {:3} from {} to {}", m.as_str(), m.start(), m.end());
            let lb = match m.start() {
                0 => 0,
                i if 0 < i => i - 1,
                _ => panic!("Faulty Length for start"),
            };
            let rb = match m.end() {
                i if i == curr.len() => i,
                i if i < curr.len() => i + 1,
                _ => panic!("Length can be longer than curr String"),
            };
            let mut surrounding = String::new();
            if !prev.is_empty() {
                surrounding += &prev[lb..rb];
            }
            surrounding += &curr[lb..rb];
            if !next.is_empty() {
                surrounding += &next[lb..rb];
            }
            if special.is_match(&surrounding) {
                if let Ok(part_number) = m.as_str().parse::<i32>() {
                    println!("Match detected PN {:3}", part_number);
                    sum += part_number;
                } else {
                    panic!("Faulty part number detected: {}", m.as_str())
                }
            };
        }
        println!();

        prev = curr.clone();
        curr = next.clone();
        if curr.is_empty() {
            break;
        }
        if let Some(line) = it.next() {
            next = line.unwrap();
        } else {
            next.clear();
        }
    }

    println!("Solution is {}", sum);
}
