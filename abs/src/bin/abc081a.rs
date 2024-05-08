use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        ss: Chars
    }
    let mut cnt = 0;
    for s in ss {
        if s == '1' {
            cnt += 1
        }
    }
    println!("{}", cnt)
}
