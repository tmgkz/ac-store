use proconio::input;

fn main() {
    input! {
        n: usize,
        mut s: [usize;n]
    }
    let mut cnt = 0;

    loop {
        if s.iter().all(|&x| x % 2 == 0) {
            s = s.iter().map(|x| x / 2).collect();
            cnt += 1;
        } else {
            break;
        }
    }

    println!("{:?}", cnt)
}
