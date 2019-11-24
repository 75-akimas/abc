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
    let x = read::<u64>();

    let (en, be) = kk(a, b, x); 
    for i in (be..en+1).rev() {
        if (a*i + b*keta(i)) as u64 <= x {
            println!("{}", i);
            return;
        }
    }
    println!("0");
}

fn kk(a:usize, b:usize, x:u64) -> (usize, usize) {
    let mut num = 100_000_000_0;
    for _i in 0..10001 {
        if (a*num + b*keta(num)) as u64 <= x {
            if num == 0 {
                num = 1;
            }
            return ((num)*10, num);
        }
        num -= 100_000;
    }
    return (0, 0);
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
