/*
  このコードはGPT-5.1を利用して出力しました。
*/

use leetcode_practice::chart::draw_multi_line_chart;
use leetcode_practice::solutions::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let coins = vec![1, 2, 5, 186, 419, 83, 408];
    let inputs = vec![10, 20, 40, 80, 160, 320, 6249];

    // A vs B vs C の複数シリーズ
    let mut series: Vec<(&str, Vec<(f64, f64)>)> = vec![];

    let mut recursive_dfs = vec![];
    for amount in &inputs {
        let t = measure(|| step1a::Solution::coin_change(coins.clone(), *amount));
        recursive_dfs.push((*amount as f64, t));
    }
    series.push(("recursive_dfs(step1a.rs)", recursive_dfs));

    let mut bfs = vec![];
    for amount in &inputs {
        let t = measure(|| step1b_bfs::Solution::coin_change(coins.clone(), *amount));
        bfs.push((*amount as f64, t));
    }
    series.push(("bfs(step1b_bfs.rs)", bfs));

    let mut dp = vec![];
    for amount in &inputs {
        let t = measure(|| step1c_dp::Solution::coin_change(coins.clone(), *amount));
        dp.push((*amount as f64, t));
    }
    series.push(("dp(step1c_dp.rs)", dp));

    let mut bfs_v2 = vec![];
    for amount in &inputs {
        let t = measure(|| step2_bfs::Solution::coin_change(coins.clone(), *amount));
        bfs_v2.push((*amount as f64, t));
    }
    series.push(("bfs_v2(step2_bfs.rs)", bfs_v2));

    let mut dp_v2 = vec![];
    for amount in &inputs {
        let t = measure(|| step2a_dp::Solution::coin_change(coins.clone(), *amount));
        dp_v2.push((*amount as f64, t));
    }
    series.push(("dp_v2(step2a_dp)", dp_v2));

    // グラフ出力
    draw_multi_line_chart(
        "multi_line.svg",
        &series,
        "runtime comparison",
        "amount",
        "time per call [ns]",
    )?;

    println!("multi_line.png を出力しました！");
    Ok(())
}

/// 単純な計測関数：closure を N 回実行し平均 ns を返す
fn measure<F: Fn() -> i32>(f: F) -> f64 {
    let repeat = 200;
    let start = std::time::Instant::now();
    for _ in 0..repeat {
        f();
    }
    start.elapsed().as_nanos() as f64 / repeat as f64
}
