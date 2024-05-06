fn main() {
    proconio::input! {
        a: i64,
        b: i64,
    }
    if a * b % 2 == 1 {
        println!("Odd")
    } else {
        println!("Even")
    }
}
