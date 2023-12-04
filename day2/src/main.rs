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

    let mut sum = 0;
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let games = Regex::new(r"Game (?P<id>\d+):(?P<drawings>.*)").unwrap();
    let sets = Regex::new(r"(?P<set>[^;\n]+)").unwrap();
    let cubes = Regex::new(r" (?P<cube>[^,\n]+)").unwrap();
    let digit_color = Regex::new(r"(?P<num>\d+) (?P<color>(red|green|blue))").unwrap();

    for line in reader.lines() {
        let l = line.unwrap();
        //  is the game valid or not
        let mut valid = true;
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        let cap = games.captures(&l).unwrap();
        let id: i32 = cap.name("id").unwrap().as_str().parse().unwrap();
        let drawings_string = cap.name("drawings").unwrap().as_str();
        // println!("{}", drawings_string);

        for (_, [set]) in sets.captures_iter(&drawings_string).map(|c| c.extract()) {
            // println!("{}", set);
            for (_, [cube]) in cubes.captures_iter(set).map(|c| c.extract()) {
                // println!("{:?}", cube);
                let dc = digit_color.captures(cube).unwrap();
                let num_color: i32 = dc.name("num").unwrap().as_str().parse().unwrap();
                match dc.name("color").unwrap().as_str() {
                    "red" => {
                        if red < num_color {
                            red = num_color
                        }
                    }
                    "green" => {
                        if green < num_color {
                            green = num_color
                        }
                    }
                    "blue" => {
                        if blue < num_color {
                            blue = num_color
                        }
                    }
                    _ => (),
                }
            }
            // if max_red < red || max_green < green || max_blue < blue {
            //     valid = false;
            // }
        }
        println!("id {:3} r {:3} g {:3} b{:3}", id, red, green, blue);
        sum += red * green * blue;
        // if valid {
        //     sum += id;
        // }
    }
    println!("{}", sum);
}
