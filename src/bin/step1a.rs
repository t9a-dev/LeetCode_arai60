// Step1a
// 目的: 問題を理解できていれば手作業で解けるのか試す。

/*
  問題の理解
  - 柱の本数n、色の種類kが与えられる。柱を色でペイントするパターンの合計を返す。
  - 連続して2本以上同じ色でペイントすることは禁止

  何を考えて解いていたか
  - n==1のときパターンはkになる
  - n <= 0,k <= 0は0を返しておく。異常値はパターン数0であれば意味が通る。（Errを返してもいいし、-1のような無効値でも良い）
  - n==2であれば、パターン数は k + k * (k - 1)になる <- k + k * (k - 1)はk ^ 2で良い
    - 末尾が2連続で同じ色になるパターンはk個
    - 末尾が2連続で同じ色にならないパターンはk * (k - 1)個

  ここまで考えたがstep1以外の解法を思いつかなかった。
  別の解法を写経して先に進む。

  別の解法の理解
  https://github.com/ryosuketc/leetcode_arai60/pull/43/files#diff-dbcdd2453440f1f0267364e2c50c8cb106de4daca27a8e95e70363810371f1a1R36-R41
  - nが0~2の場合に対応するパターン数を計算してtotal_ways配列に入れておく
  - nが3以上の時[3~n+1)回以下の計算を繰り返す。
  https://github.com/ryosuketc/leetcode_arai60/pull/43/files#diff-7926d40b1f4831dda549cc97581eef4963ad820bb1051b0784b7c455ab272c98R38-R43
    - 今見ているフェンス[i]を直前のフェンス[i-1]と違う色で塗る。k色-1 * total_ways[i-1]
    - 今見ているフェンス[i]を直前のフェンス[i-1]同じ色で塗る。このとき直前のフェンス[i-1]と2つ前のフェンス[i-2]が違う色であれば可能。
    今見ているフェンスを直前のフェンス[i-1]として、さらに直前のフェンスを[i-2]としたとき、上記の例と同じく　k色-1 * total_ways[-2]となる。
    k=2,n=3のとき（A色、B色）
  n-> 0 12
      A AB
      B BA
      A BA
      A BB
      B AA
      B AB
    - total_ways[n]に対応するパターンの数を入れる
    - 最後に入力のnに対応するパターン数を返す。

  所感
  - 理解が難しいと思う箇所が以下の計算式を求める箇所であることを考えると、LeetCodeでコーディングを素振りしているように中高数学の素振りもしたほうが良さそう。
    というのも受験勉強を全くしたことが無く、数学にまともに触れてこなかったのが理解を妨げている気がする。受験勉強で要求される数学ができるかというよりは数を見たときにそれを処理するモデルが自分にない感じがする。
    もう少し具体的に感じていることを書くと、(k - 1) * total_ways[i - 1]までは理解できたが、これをスライドさせて (k - 1) * total_ways[i - 2]が出てこない。
    `(k - 1) * total_ways[(i - 1) as usize] + (k - 1) * total_ways[(i - 2) as usize]`
*/

pub struct Solution {}
impl Solution {
    pub fn num_ways(n: i32, k: i32) -> i32 {
        if n <= 0 || k <= 0 {
            return 0;
        };

        let mut total_ways = vec![0, k, k * k];

        for i in 3..=n {
            let num_ways =
                (k - 1) * total_ways[(i - 1) as usize] + (k - 1) * total_ways[(i - 2) as usize];

            total_ways.push(num_ways);
        }

        total_ways[n as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step1a_test() {
        assert_eq!(Solution::num_ways(3, 2), 6);
        assert_eq!(Solution::num_ways(2, 2), 4);
        assert_eq!(Solution::num_ways(3, 1), 0);
        assert_eq!(Solution::num_ways(4, 2), 10);

        // --- basic cases ---
        assert_eq!(Solution::num_ways(1, 1), 1);
        assert_eq!(Solution::num_ways(1, 2), 2);
        assert_eq!(Solution::num_ways(1, 3), 3);
        assert_eq!(Solution::num_ways(2, 1), 1);
        assert_eq!(Solution::num_ways(2, 3), 9);

        // --- constraint cases ---
        assert_eq!(Solution::num_ways(3, 1), 0);
        assert_eq!(Solution::num_ways(4, 1), 0);
        assert_eq!(Solution::num_ways(5, 1), 0);
        assert_eq!(Solution::num_ways(3, 3), 24);

        // --- larger values ---
        assert_eq!(Solution::num_ways(4, 3), 66);
        assert_eq!(Solution::num_ways(5, 2), 16);
        assert_eq!(Solution::num_ways(5, 3), 180);

        // --- n = 0 をどう扱うか（学習用） ---
        assert_eq!(Solution::num_ways(0, 1), 0);
        assert_eq!(Solution::num_ways(0, 3), 0);
    }
}
