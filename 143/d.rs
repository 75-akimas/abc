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

use std::cmp::Ordering;

/// Equivalent to std::lowerbound and std::upperbound in c++
pub trait BinarySearch<T> {
    fn lower_bound(&self, &T) -> usize;
    fn upper_bound(&self, &T) -> usize;
}

impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less => {
                    low = mid + 1;
                }
                Ordering::Equal | Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }

    fn upper_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less | Ordering::Equal => {
                    low = mid + 1;
                }
                Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }
}

#[test]
fn test_binary_search() {
    let vec = vec![1, 2, 4, 6, 7, 12, 54, 60];

    assert_eq!(vec.lower_bound(&4), 2);
    assert_eq!(vec.upper_bound(&4), 3);
}

fn main() {
    let n = read::<usize>();
    let mut l = vec![0; n];

    for i in 0..n {
        l[i] = read::<i64>();
    }

    l.sort();

    let mut ans = 0;
    for i in 0..n {
        for j in i+1..n {
            let t = std::cmp::max(l[i]-l[j], l[j]-l[i]);
            let s = l.lower_bound(&(l[i]+l[j]));
            let t = l.upper_bound(&t);
            let mut sa: i64 = s as i64 - t as i64;
            if t <= i && i < s {
                sa -= 1;
            }
            if t <= j && j < s {
                sa -= 1;
            }
            if sa > 0 {
                ans += sa;
            }
        }
    }

    println!("{}", ans/3);
}
