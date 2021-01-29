#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        mut d: usize,
    }
    let p = [2, 3, 5];
    let mut cnt = [0; 3];

    for (c, &p) in cnt.iter_mut().zip(p.iter()) {
        // dを素因数分解して書く因数のカウント
        while d % p == 0 {
            *c += 1;
            d /= p;
        }
    }
    if d > 1 {
        println!("0");
        return;
    }
    let (a, b, c) = (cnt[0], cnt[1], cnt[2]);
    let mut dp = vec![vec![vec![0f64; c + 1]; b + 1]; a + 1];
    dp[0][0][0] = 1f64;
    let p = 1f64 / 6f64;

    for _ in 0..n {
        for i in (0..=a).rev() {
            for j in (0..=b).rev() {
                for k in (0..=c).rev() {
                    let v = dp[i][j][k] * p;
                    dp[i][j][k] = 0f64;
                    dp[i][j][k] += v;
                    dp[a.min(i + 1)][j][k] += v;
                    dp[i][b.min(j + 1)][k] += v;
                    dp[a.min(i + 2)][j][k] += v;
                    dp[i][j][c.min(k + 1)] += v;
                    dp[a.min(i + 1)][b.min(j + 1)][k] += v;
                }
            }
        }
    }
    let ans = dp[a][b][c];
    println!("{:.10}", ans);
}
