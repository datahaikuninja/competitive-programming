fn main() {
    proconio::input! {
        n: usize,
        a: [i64; n],
    }

    let mut cnt = 1;
    let base: i64 = 2;

    loop {
        let mut break_frag = false;
        for i in &a {
            if i % base.pow(cnt as u32) != 0 {
                break_frag = true;
            }
        }

        if break_frag {
            break;
        }

        cnt += 1
    }

    println!("{}", cnt - 1);
}
