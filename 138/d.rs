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
struct UnionFind {
    par: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        let mut vec = vec![0;n];
        for i in 0..n {
            vec[i] = i;
        }
        UnionFind {
            par : vec,
            rank : vec![0;n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if x == self.par[x] {
            x
        }else{
            let par = self.par[x];
            let res = self.find(par);
            self.par[x] = res;
            res
        }
    }

    fn same(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }

    fn unite(&mut self, a: usize, b: usize){
        let apar = self.find(a);
        let bpar = self.find(b);
        if self.rank[apar] > self.rank[bpar] {
            self.par[bpar] = apar;
        }else{
            self.par[apar] = bpar;
            if self.rank[apar] == self.rank[bpar] {
                self.rank[bpar] += 1;
            }
        }
    }
}

fn main() {

    let n = read::<usize>();
    let q = read::<usize>();

    let mut uf = UnionFind::new(n+1);
    let mut ab = vec![(0, 0); n];
    for i in 0..n-1 {
        ab[i] = (read::<usize>(), read::<usize>());
        uf.unite(ab[i].0, ab[i].1);
    }

    let mut px = vec![(0, 0); q];
    let mut ans = vec![0; n];
    for i in 0..q {
        px[i] = (read::<usize>(), read::<usize>());
        for j in 1..n+1 {
            if uf.same(j, px[i].0) {
                ans[j-1] += px[i].1;
            }
        }
    }

    for i in 0..n {
        print!("{} ", ans[i]);
    }
}