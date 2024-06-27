use proconio::input;

fn main() {
    input! {
        n: i32,
        mut s: [i32;n]
    }
    let mut r: i32 = 0;
    s.sort_by(|a, b| a.cmp(&b).reverse());

    for (i, val) in s.iter().enumerate() {
        match i % 2 {
            0 => r += val,
            _ => r -= val,
        };
    }
    println!("{}", r)
}
