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
    let a = read::<usize>();
    let b = read::<usize>();
    let x = read::<usize>();

    let mut top = 100_000_000_0;
    if (a*(top) + b*keta(top)) <= x {
        println!("{}", top);
        return;
    }
    let mut bottom = 0;
    while top - bottom != 1 {
        let i = (top+bottom)/2;
        if (a*(i) + b*keta(i)) <= x {
            bottom = i;
        } else {
            top = i;
        }
    }
    println!("{}", (top+bottom)/2);
}

fn keta(a: usize) -> usize {
    let mut x = a.clone();
    let mut num = 0;
    while x != 0 {
        x = (x as i64 / 10) as usize;
        num += 1;
    }
    return num;
}
