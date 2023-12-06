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

    let reader = io::BufReader::new(file);
    let it = reader.lines();

    let mut sum = 0;
    for line in it {
        let l = line.unwrap();
        let ls = l.split(|c| c == ':' || c == '|').collect::<Vec<_>>();
        let num_win: Vec<u32> = ls[1]
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let num_my: Vec<u32> = ls[2]
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let mut points = 0;
        for num in num_my {
            if num_win.contains(&num) {
                points = if points == 0 { 1 } else { points * 2 }
            }
        }
        sum += points;
    }
    println!("Solution: {}", sum);
}
