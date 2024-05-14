fn main() {
    proconio::input! {
        n: usize,
        a: [i64; n],
    }
    let mut hit = false;
    for (i, v) in a.iter().enumerate() {
        if v > &a[0] {
            println!("{}", i + 1);
            hit = true;
            break;
        }
    }
    if !hit {
        println!("{}", -1)
    }
}
