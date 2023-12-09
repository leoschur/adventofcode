use itertools::Itertools;
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn card_to_value(c: &char) -> u64 {
    u64::from(
        c.to_digit(10).unwrap_or_else(|| match c {
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => 1,
        }) - 1,
    )
}

fn value_hand(hand: &str) -> u64 {
    // the cards value is encoded in 33 bits
    // Five Four Thre Pair Pair Single  Cards
    // ---- ---- ---- ---- ---- -------------
    // Higher combinations are stored in higher valued bits
    // => larger value wins against lower value
    let mut value = 0u64;
    let mut double = false;

    let debug_info = hand
        .chars()
        .map(|c| (c, card_to_value(&c)))
        .sorted_by(|a, b| a.1.cmp(&b.1))
        .map(|(c, _)| c)
        .collect::<String>();
    print!("Hand {}", debug_info);

    for (v, num) in hand
        .chars()
        .counts()
        .into_iter()
        .map(|(c, num)| (card_to_value(&c), num))
        .sorted_by(|a, b| Ord::cmp(&a.0, &b.0))
    {
        match num {
            1 => value |= 1 << v - 1,
            2 => {
                if !double {
                    value |= v << 13;
                    double = true;
                } else {
                    value |= v << 17;
                }
            }
            3 => value |= v << 21,
            4 => value |= v << 25,
            5 => value |= v << 29,
            _ => (),
        }
    }
    println!(
        " {:04b} {:04b} {:04b} {:04b} {:04b} {:013b} Full {:033b}",
        (value >> 29) & !(u64::MAX << 4),
        (value >> 25) & !(u64::MAX << 4),
        (value >> 21) & !(u64::MAX << 4),
        (value >> 17) & !(u64::MAX << 4),
        (value >> 13) & !(u64::MAX << 4),
        value & !(u64::MAX << 13),
        value
    );
    value
}

fn main() {
    let path = Path::new("input.txt");
    let display = path.display();

    let file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => panic!("Can't open {}: {}", display, e),
    };

    let reader = io::BufReader::new(file);
    let it = reader.lines();

    let mut res = vec![];
    for line in it {
        let l = line.unwrap();
        let Some((hand, bet)) = l.split_whitespace().collect_tuple() else {
            panic!("Can't parse line: {}", l);
        };
        res.push((value_hand(hand), bet.parse::<u64>().unwrap()));
    }

    let mut sum: u64 = 0;
    for (i, bet) in (1..res.len() + 1).zip(
        res.into_iter()
            .sorted_by(|a, b| a.0.cmp(&b.0))
            .map(|(_, bet)| bet),
    ) {
        sum += i as u64 * bet;
    }
    println!("{:?}", sum);
}
