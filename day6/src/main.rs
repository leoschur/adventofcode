fn main() {
    let times = [35, 93, 73, 66]; //  [7, 15, 30];
    let distances = [212, 2060, 1201, 1044]; //  [9, 40, 200];
    let mut nums = [0; 4];

    for (n, (&t, &d)) in std::iter::zip(nums.iter_mut(), times.iter().zip(distances.iter())) {
        println!("Race Time: {:3} Longest Distance: {:3}", t, d);
        for i in 0..t {
            if d < i + (i * (t - i - 1)) {
                *n = *n + 1;
                // println!("Faster time: {:3}ms pressed", i);
            }
        }
        println!("{:3} possibilities found", n);
    }
    let res = nums.into_iter().reduce(|p, n| p * n).unwrap();
    println!("Result: {}", res);
}
