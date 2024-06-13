use proconio;

fn main() {
    proconio::input! {
        a: i32,
        b: i32,
    };
    match (a * b) % 2 {
        0 => println!("Even"),
        _ => println!("Odd"),
    }
}
