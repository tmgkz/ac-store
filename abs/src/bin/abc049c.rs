use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        mut s: String,
    }
    if s.len() <= 4 {
        println!("NO");
        return;
    }
    // let a = s
    //     .replace("eraser", "")
    //     .replace("erase", "")
    //     .replace("dreamer", "")
    //     .replace("dream", "");
    // if a == "" {
    //     println!("YES")
    // } else {
    //     println!("NO")
    // }

    let mut str: VecDeque<char> = s.chars().rev().collect();
    loop {
        if str.len() == 0 {
            println!("YES");
            return;
        }
        let a = str.drain(0..5).collect::<VecDeque<_>>();
        let mut joined_str = join_str(&a);
        if joined_str == "maerd" || joined_str == "esare" {
            continue;
        }
        let a = str.drain(0..1).collect::<VecDeque<_>>();
        joined_str.push_str(join_str(&a).as_str());
        if joined_str == "resare" {
            continue;
        }
        let a = str.drain(0..1).collect::<VecDeque<_>>();
        joined_str.push_str(join_str(&a).as_str());
        if joined_str == "remaerd" {
            continue;
        }
        println!("NO");
        return;
    }
}

fn join_str(chars: &VecDeque<char>) -> String {
    chars
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("")
}
