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
    let a = read::<String>().chars().collect::<Vec<char>>();

    for i in 0..n/2 {
        if a[i] != a[i+n/2] {
            println!("No");
            return;
        }
    }

    if n % 2 == 0 {
        println!("Yes");
        return;
    }
    println!("No");
}
