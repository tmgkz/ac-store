use proconio::input;

fn main() {
    input! {
        a: usize,
        x: [[usize;3]; a]
    }
    println!("{:?}", x)
}
