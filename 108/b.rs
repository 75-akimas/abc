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
    let xy1 = (read::<i32>(), read::<i32>());
    let xy2 = (read::<i32>(), read::<i32>());
    print!("{} {} {} {}", xy2.0 - (xy2.1 - xy1.1), xy2.1 + (xy2.0 - xy1.0), xy1.0 - (xy2.1 - xy1.1), xy1.1 + (xy2.0 - xy1.0),);
}