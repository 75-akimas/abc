macro_rules! get {
      ($t:ty) => {
          {
              let mut line: String = String::new();
              std::io::stdin().read_line(&mut line).unwrap();
              line.trim().parse::<$t>().unwrap()
          }
      };
      ($($t:ty),*) => {
          {
              let mut line: String = String::new();
              std::io::stdin().read_line(&mut line).unwrap();
              let mut iter = line.split_whitespace();
              (
                  $(iter.next().unwrap().parse::<$t>().unwrap(),)*
              )
          }
      };
      ($t:ty; $n:expr) => {
          (0..$n).map(|_|
              get!($t)
          ).collect::<Vec<_>>()
      };
      ($($t:ty),*; $n:expr) => {
          (0..$n).map(|_|
              get!($($t),*)
          ).collect::<Vec<_>>()
      };
      ($t:ty ;;) => {
          {
              let mut line: String = String::new();
              std::io::stdin().read_line(&mut line).unwrap();
              line.split_whitespace()
                  .map(|t| t.parse::<$t>().unwrap())
                  .collect::<Vec<_>>()
          }
      };
      ($t:ty ;; $n:expr) => {
          (0..$n).map(|_| get!($t ;;)).collect::<Vec<_>>()
      };
}

fn main() {
    let s = get!(String);
    let ss = s.as_bytes().into_iter()
        .map(|as_char| {
            as_char - 48
        })
        .collect::<Vec<_>>();

    if ss[0] < ss[1] {
        print!("{}{}{}", ss[0] + 1, ss[0] + 1, ss[0] + 1);
    } else if ss[0] < ss[2] && ss[0] == ss[1] {
        print!("{}{}{}", ss[0]+1, ss[0]+1, ss[0]+1);
    } else {
        print!("{}{}{}", ss[0], ss[0], ss[0]);
    }
}