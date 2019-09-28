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
 
use std::collections::BinaryHeap;

fn main() {
    let n = read::<usize>();
    let m = read::<usize>();
    let mut a = BinaryHeap::new();
    let mut sum = 0;
 
    for i in 0..n {
        let tmp = read::<usize>();
        a.push(tmp);
        sum += tmp;
    }
 
    for _i in 0..m {
        let tmp = a.pop().unwrap();
        sum -= (tmp-tmp/2);
        a.push(tmp/2);
    }
 
    println!("{}",  sum);
}