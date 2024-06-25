use proconio::input;

fn main() {
    input! {
        n: i32,
        a: i32,
        b: i32
    }
    let numbers: Vec<i32> = (1..n + 1).collect();
    println!(
        "{}",
        numbers
            .iter()
            .filter(|x| is_range(sum_single_number(x), a, b))
            .sum::<i32>(),
    )
}

fn is_range(x: i32, a: i32, b: i32) -> bool {
    a <= x && x <= b
}

fn sum_single_number(i: &i32) -> i32 {
    let mut result = Vec::new();
    let mut number: i32 = *i;
    if number.to_string().len() == 1 {
        return number;
    }
    for _ in 1..number.to_string().len() + 1 {
        result.push(number % 10);
        number = number / 10;
    }
    result.iter().sum()
}
