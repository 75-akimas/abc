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
    let n = read::<i64>();
    let y = read::<i64>();

    let mut b;
    let mut c;
    for a in 0..n+1 {
        b = (y - 1000 * n - 9000 * a) / 4000;
        c = n - a - b;
        if (y - 1000*n - 9000*a) % 4000 == 0 && c >= 0 && b >= 0 {
            println!("{} {} {}", a, b, c);
            return;
        }
    }

    println!("-1 -1 -1");
}