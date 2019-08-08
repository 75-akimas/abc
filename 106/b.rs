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
    let n = read::<usize>();

    let mut count = 0;
    for i in 0..n {
        let mut count2 = 0;
        if i % 2 == 0 {
            for j in 0..i {
                if (i+1) % (j+1) == 0 {
                    count2 += 1;
                }
            }
            if count2 == 7 {
                count += 1;
            }
        }
    }
    println!("{}", count);
}