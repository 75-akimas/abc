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
    let mut n = read::<i64>();

    if n == 0 {
        println!("{}", 0);
        return;
    }


    let mut ans = "".to_string();
    while n != 0 {
        if n % 2 != 0 {
            n -= 1;
            ans = ans + "1";

        } else {
            ans = ans + "0";
        }
        n /= -2;

    }
    println!("{}", ans.chars().rev().collect::<String>());
}