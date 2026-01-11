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
  コメント集、他の人のコードを読んで考えたこと
  https://discord.com/channels/1084280443945353267/1200089668901937312/1216054396161622078
    > 実は、ビットの数を調べるのは、たまに大事で、有名なアルゴリズムがあります。
    > ビット演算だけでできる計算って実はかなり豊かなんですよね。
    > https://stackoverflow.com/questions/109023/count-the-number-of-set-bits-in-a-32-bit-integer#109025
  - x86アーキテクチャのCPUではハードウェアレベルのサポート（命令セットにpopcnt命令がある）があるとのこと。
  ハードウェアサポートの有無に依存しないポータブルなソフトウェア実装のアルゴリズムが紹介されている。
  「ハミング重み」とはbit列に出現する1の個数のこと。
    https://ja.wikipedia.org/wiki/%E3%83%8F%E3%83%9F%E3%83%B3%E3%82%B0%E9%87%8D%E3%81%BF
  popcntのRust実装があった。
    https://github.com/BartMassey/popcount
    READMEで言及されている「Hacker’s Delight」はnodchipさんのコメントでも翻訳版「ハッカーのたのしみ」として言及されていていわゆる名著なのかなと思った。
    https://github.com/hroc135/leetcode/pull/44#discussion_r2007607576

  https://github.com/hayashi-ay/leetcode/pull/46/changes
  - 読みやすい。Binary Treeとして見るという視点はなかったので参考になった。

  https://github.com/hayashi-ay/leetcode/pull/46#issuecomment-1986824146
  https://github.com/olsen-blue/Arai60/pull/47/changes#r2002307405
  - nに依存せずkから答えを求められる。

  https://github.com/hroc135/leetcode/pull/44/changes#r2019738588
  - 場合分けのときはif elseで書きたいという意見。
  ifブロックの中身がreturnだけのときはネストを浅くしたいという気持ちもわかるし、場合分けの対称性が分かるようにあえてelseとするという方針も理解できる。
  どちらも正しいと思うので一度コーディング規約で決めてしまって、この部分の議論に時間を使いたくないなという感じ。

  改善する時に考えたこと
  - 引数チェックの追加。

  所感
  - 問題を見てbit演算の考え方で効率的な解法が思いつかなくても、この解法は思いつきたいなと思える解法だった。シンプルかつ問題の答えとして十分だと感じたため。

  一番読みやすく理解しやすいと思ったコードを参考にした。
  https://github.com/hayashi-ay/leetcode/pull/46/changes
*/

pub struct Solution {}
impl Solution {
    pub fn kth_grammar(n: i32, k: i32) -> i32 {
        if n <= 0 || k <= 0 {
            panic!("n and k must be greater than 0")
        }

        if n == 1 {
            return 0;
        }

        let previous_value = Self::kth_grammar(n - 1, (k + 1) / 2);
        if previous_value == 0 {
            if k % 2 == 0 {
                return 1;
            } else {
                return 0;
            }
        } else {
            if k % 2 == 0 {
                return 0;
            } else {
                return 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step2_test() {
        assert_eq!(Solution::kth_grammar(1, 1), 0);
        assert_eq!(Solution::kth_grammar(2, 1), 0);
        assert_eq!(Solution::kth_grammar(2, 2), 1);
    }

    #[test]
    #[should_panic]
    fn kth_grammar_n1_k0_panics() {
        Solution::kth_grammar(1, 0);
    }

    #[test]
    #[should_panic]
    fn kth_grammar_n0_k1_panics() {
        Solution::kth_grammar(0, 1);
    }

    #[test]
    #[should_panic]
    fn kth_grammar_n0_k0_panics() {
        Solution::kth_grammar(0, 0);
    }

    #[test]
    #[should_panic]
    fn kth_grammar_negative_n_panics() {
        Solution::kth_grammar(-1, 0);
    }

    #[test]
    #[should_panic]
    fn kth_grammar_negative_k_panics() {
        Solution::kth_grammar(0, -1);
    }
}
