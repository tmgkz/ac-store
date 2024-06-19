use proconio::input;

fn main() {
    input! {
        n: i32,
        a: i32,
        b: i32
    }
    let mut numbers = Vec::<i32>::new();
    for c in 1..n + 1 {
        let s = c.chars().sum();
        if a <= s && s <= b {
            numbers.push(c)
        }
    }
    println!("{}", numbers.iter().sum())
}
