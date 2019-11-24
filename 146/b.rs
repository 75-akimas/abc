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
    let n = read::<u8>();
    let mut a = read::<String>().chars().collect::<Vec<char>>();
    for c in a {
        if c as u8 + n > 90 {
            print!("{}", (c as u8 - 26 + n) as char);
        } else {
            print!("{}", (c as u8 + n) as char);
        }
    }

}

