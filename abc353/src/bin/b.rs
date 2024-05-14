fn main() {
    proconio::input! {
        n: i64,
        k: i64,
        a: [i64; n],
    }

    let mut seat = k;
    let mut ans = 1;

    for i in &a {
        if seat < *i {
            seat = k;
            ans += 1;
        }
        // seat -= i; // compile ok
        seat -= *i;
    }
    println!("{}", ans)
}
