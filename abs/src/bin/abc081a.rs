use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let arr = s.chars().collect::<Vec<char>>();
    let mut count = 0;

    for number in arr {
        match number {
            '1' => count += 1,
            _ => (),
        }
    }

    println!("{}", count)
}
