#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }
    let mut dp = vec![false; n * 100 + 1];
    dp[0] = true;
    for i in p {
        for j in (0..dp.len()).rev() {
            if i + j >= dp.len() {
                continue;
            }
            if dp[j] {
                dp[i + j] = true;
            }
        }
    }
    println!("{}", dp.into_iter().filter(|&x| x).count());
}
