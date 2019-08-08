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

    let mut flag = 0;
    if s[0] == 'A' {
        for i in 1..s.len() {
            if 'A' <= s[i] && s[i] <= 'Z' {
                if flag == 1 {
                    flag = 0;
                    break;
                } else if 2 <= i && i <= s.len()-2 && s[i] == 'C' {
                    flag = 1;
                } else {
                    break;
                }
            }
        }
    }

    println!("{}", if flag == 1 {"AC"} else {"WA"}.to_string())

}