use std::usize;

fn main() {
    proconio::input! {
        n: usize,
        l: usize,
        r: usize,
    }
    let mut v = vec![];
    for i in 1..=n {
        // `1..n`だとnを含まないので、`1..=n`と書く
        v.push(i)
    }
    // Vecのコピーを一度に抑える解答
    /*
    let mut vc = v.clone();
    for i in (l - 1)..r {
        vc[i] = v[l + r - 2 - i];
    }
    for i in &vc {
        print!("{} ", &i);
    }
    println!();
    */

    let mut hv = v[..(l - 1)].to_vec();
    let mut mv = v[(l - 1)..r].to_vec();
    mv.reverse();
    let mut tv = v[r..].to_vec();
    // dbg!(&tv);
    hv.append(&mut mv);
    hv.append(&mut tv);
    for i in hv {
        print!("{} ", &i)
    }
    // dbg!(&v);
}
