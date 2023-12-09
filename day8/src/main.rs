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

    let reader = io::BufReader::new(file);
    let mut it = reader.lines();

    let re = Regex::new(r"(?P<id>[A-Z]{3}) = \((?P<l>[A-Z]{3}), (?P<r>[A-Z]{3})\)").unwrap();

    let lr = it.next().unwrap().unwrap();
    it.next();

    let mut v = vec![];
    for line in it {
        let l = line.unwrap();
        let cap = re.captures(&l).unwrap();
        v.push((
            cap.name("id").unwrap().as_str().to_owned(),
            (
                cap.name("l").unwrap().as_str().to_owned(),
                cap.name("r").unwrap().as_str().to_owned(),
            ),
        ));
    }
    // dirty cloning to make it searchable in closure
    let vc = v.clone();
    let find = move |s: String| vc.iter().position(|(id, _)| id.eq(&s)).unwrap();

    let mut steps = 1u32;
    let mut i = find("AAA".to_owned());
    for c in lr.chars().cycle() {
        let (left, right) = v[i].1.clone();
        match c {
            'L' => i = find(left),
            'R' => i = find(right),
            _ => (),
        }
        if v[i].0.eq("ZZZ") {
            break;
        } else {
            steps += 1;
        }
    }
    println!("{:?}", steps);
}
