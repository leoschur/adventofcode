use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let path = Path::new("input.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => panic!("Can't open {}: {}", display, e),
    };

    let reader = io::BufReader::new(file);

    let mut total = 0u32;
    for line in reader.lines() {
        let mut l = line.unwrap();
        // l = l.replace("one", "1");
        // l = l.replace("two", "2");
        // l = l.replace("three", "3");
        // l = l.replace("four", "4");
        // l = l.replace("five", "5");
        // l = l.replace("six", "6");
        // l = l.replace("seven", "7");
        // l = l.replace("eight", "8");
        // l = l.replace("nine", "9");

        // (?<!tw)one
        // (?<!eigh)two
        // (?<!eigh)three
        // four
        // five
        // six
        // seven
        // (?<!on|thre|fiv|nin)eight
        // nine
        // issue with this: twoneightwo | twoneighthree
        // does not match but apparently none stays left anyways?

        println!("{}", l);
        let digits = l.chars().filter_map(|c| c.to_digit(10));
        let v: Vec<u32> = digits.collect();
        if let (Some(a), Some(b)) = (v.first(), v.last()) {
            let number = a * 10 + b;
            println!("{}", number);
            total += number;
        }
        print!("\n")
    }
    println!("\n{}", total);
}
