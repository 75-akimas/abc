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
    let mut a = read::<String>();
    match &a[..] {
        "SUN" => {
            println!("7")
        },
        "SAT" => {
            println!("1")
        },
        "FRI" => {
            println!("2")
        },
        "THU" => {
            println!("3")
        },
        "WED" => {
            println!("4")
        },
        "TUE" => {
            println!("5")
        },
        "MON" => {
            println!("6")
        },
        _ => {
            return;
        }
    }
}

