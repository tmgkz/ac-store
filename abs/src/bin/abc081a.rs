use proconio::input;

fn main() {
    input! {
        s: String,
    }
    // let arr = s.chars().collect::<Vec<char>>();
    // let mut count = 0;
    //
    // for number in arr {
    //     match number {
    //         '1' => count += 1,
    //         _ => (),
    //     }
    // }

    let count = s.chars().filter(|&x| x == '1').count();

    println!("{}", count)
}
