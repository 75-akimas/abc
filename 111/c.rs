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

use std::collections::HashMap;

fn main() {
    let n = read::<u32>();
    let mut even = HashMap::new();
    let mut odd = HashMap::new();

    for i in 0..n {
        let v = read::<u32>();
        if i % 2 == 1 {
            *even.entry(v).or_insert(0) += 1
        } else {
            *odd.entry(v).or_insert(0) += 1
        }
    }

    let even_max = max_kv(&even);
    let odd_max = max_kv(&odd);

    if even_max.0 != odd_max.0 {
        println!("{}", n - odd_max.1 - even_max.1);
    } else {
        even.remove(&even_max.0);
        odd.remove(&odd_max.0);

        let even_max_2 = max_kv(&even);
        let odd_max_2 = max_kv(&odd);
        if even_max_2.1 > odd_max_2.1 {
            println!("{}", n - odd_max.1 - even_max_2.1);
        } else {
            println!("{}", n - odd_max_2.1 - even_max.1);
        }
    }
}

fn max_kv (map: &HashMap<u32, u32>) -> (u32, u32) {
    let mut max = (0, 0);
    for (k, v) in map {
        if max.1 < *v {
            max = (*k, *v);
        }
    }
    max
}