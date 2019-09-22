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
    let mut txy = vec![(0, 0, 0); n + 1];

    for i in 1..n+1 {
        
        txy[i] = (read::<i64>(), read::<i64>(), read::<i64>());
    }

    for i in 1..n+1 {
        if ((txy[i].0 - txy[i-1].0).abs() >= (txy[i].1 - txy[i-1].1).abs() + (txy[i].2 - txy[i-1].2).abs())
        && ((txy[i].0 - txy[i-1].0).abs() % 2 == ((txy[i].1 - txy[i-1].1).abs() + (txy[i].2 - txy[i-1].2).abs()) % 2) {
            continue;
        } else {
            println!("No");
            return;
        }
    }
    println!("Yes")
}