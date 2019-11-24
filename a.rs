use std::io::*;
use std::str::FromStr;

pub fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.expect("failed to read char") as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().expect("failed to parse token")
}

fn main() {
    let s = read::<String>().chars().collect::<Vec<char>>();
    let mut a = vec![0; s.len()+1];

    let mut min = 0;
    for i in 1..a.len() {
        if s[i-1] == '<' {
            a[i] = a[i-1] + 1;
        } else {
            a[i] = a[i-1] - 1;
            min = std::cmp::min(min, a[i]);
        }
    }

    println!("{:?}", a);
    let mut sum = 0;
    for i in 0..a.len() {
        sum += a[i] + min*-1;
        print!("{} ", a[i] + min*-1);
    }
    println!("{}", sum);
}
