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

    let mut d = vec![0; n];

    for i in 0..n {
        d[i] = read::<usize>();
    }

    let mut ans = 0;
    for i in 0..n {
        for j in i..n {
            if i == j {
                continue
            }
            ans += d[i] * d[j];
        }
    }
    println!("{}", ans)
}
