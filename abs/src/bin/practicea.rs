fn main() {
    proconio::input! {
        a: i32,
        b: i32,
        c: i32,
        s: String,
    }
    let sum = a + b + c;
    println!("{} {}", sum, s);
}
