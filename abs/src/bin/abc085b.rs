use proconio::input;

fn main() {
    input! {
        n: i32,
        mut s: [i32;n]
    }
    s.sort();
    s.dedup();
    println!("{}", s.iter().count())
}
