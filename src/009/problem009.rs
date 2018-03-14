fn is_pythagorean_triplet(a: i64, b: i64, c: i64) -> bool {
    a * a + b * b == c * c
}

fn solve() -> i64 {
    let mut product = 0;

    for a in 0..500 {
        for b in 0..500 {
            let c = 1000 - a - b;
            let sum = a + b + c;
            if is_pythagorean_triplet(a, b, c) && sum == 1000 {
                product = a * b * c;
                break;
            }
        }
    }
    product
}

fn main() {
    let result = solve();
    println!("{}", result);
}