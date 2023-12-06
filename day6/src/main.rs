fn main() {
    let times: [u64; 1] = [35937366]; //  [7, 15, 30];
    let distances: [u64; 1] = [212206012011044]; //  [9, 40, 200];
    let mut nums: [u64; 1] = [0; 1];

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
