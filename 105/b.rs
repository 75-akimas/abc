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
    let n = read::<i32>();

    for i in 0..n/4+1 {
        for j in 0..n/7+1 {
            if 4*(i) + 7*(j) == n {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}