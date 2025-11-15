// Step2
// 目的: 自然な書き方を考えて整理する

// 方法
// Step1のコードを読みやすくしてみる
// 他の人のコードを2つは読んでみること
// 正解したら終わり

// 以下をメモに残すこと
// 講師陣はどのようなコメントを残すだろうか？
// 他の人のコードを読んで考えたこと
// 改善する時に考えたこと

/*
  他の人のコードを読んで考えたこと
  - ぱっと見た感じ実装パターンも多く一番わかり易いと思った。
  記載されている説明文もわかりやすく、かなり理解の助けになった。
  https://github.com/ryosuketc/leetcode_arai60/pull/43/files

  - こちらもコードがシンプルで分かりやすい。
  漸化式がよくわからないのでやはり中高数学を勉強したほうが良さそう。（調べたら漸化式は高校数学だった。）
  今回の問題のような状態推移を自然言語で表すのではなく、式として表せると理解した。
  https://github.com/docto-rin/leetcode/pull/35/files

  - 数学を感じたやりとり
  https://github.com/tokuhirat/LeetCode/pull/30/files#r2126104372

  改善する時に考えたこと
  - previousは冗長なのでprevにした。
  - n==1,n==2のガード節をmatch文にまとめた。
*/

pub struct Solution {}
impl Solution {
    pub fn num_ways(n: i32, k: i32) -> i32 {
        if n <= 0 || k <= 0 {
            return 0;
        };

        let mut tail_two_same = k;
        let mut tail_two_diff = k * (k - 1);

        match n {
            1 => return tail_two_same,
            2 => return tail_two_same + tail_two_diff,
            _ => (),
        };

        for _ in 3..=n {
            let prev_tail_two_same = tail_two_same;
            let prev_tail_two_diff = tail_two_diff;

            tail_two_same = prev_tail_two_diff;
            tail_two_diff = (prev_tail_two_same + prev_tail_two_diff) * (k - 1);
        }

        tail_two_same + tail_two_diff
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step2_test() {
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
