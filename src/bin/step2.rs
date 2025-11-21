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
  - step1a_naive.rsでキャッシュなしの場合の計算利用見積もりについて。
  正直よくわからないが後で見返すかもしれないのでメモ。
  https://github.com/olsen-blue/Arai60/pull/33#discussion_r1966730122

  - 階乗による解法は利用していないのでしっかりと読んでいないが階乗周りの最適化についてのトピック。
  https://github.com/olsen-blue/Arai60/pull/33/files#r1966848205
  https://github.com/nittoco/leetcode/pull/26/files#r1677082520
  https://qiita.com/AkariLuminous/items/1b2e964ebabde9419224

  - この問題は数学的に解けるらしい。
  https://github.com/nanae772/leetcode-arai60/pull/32/files#diff-c240887f537c0740f4e6187e26cc71e1d27c36006d590468a24fd1a453172c25R11-R15
  というか自分の採用した解法も解説動画を見てこうしたらこうなるということだけを理解したが、背景に数学的な証明があることは変わらないなと思った。
  NeetCode解説動画
  https://www.youtube.com/watch?v=IlEsdxuD4lY

  - 理路整然と実装方針を考えた記録が残っており読んでいて参考になった。自分が理解できない問題を理解できる人はこう考えているのかという観点で。
  これができると見える世界が変わる(違う視点から見れる)んだろうななどと思った。
  https://github.com/docto-rin/leetcode/pull/38/files#diff-8a6ff63343e74cc80d732f8899ddf515ccf1d52a91f0f73f0d8b022fd23400cbR16-R19

  改善する時に考えたこと
  - step1の解法を採用するにあたり、min(m,n)を外側のループで利用するように改善する。
  - calculate_rowについて、calculated_row,calculating_rowも選択肢にあるなと思った。日本語から考えると計算中の行なので、calculating_rowが良いかと思ったが正しいかの判断基準が自分の中に無いのでChatGPT(GPT-5.1)に聞く。
  普通に変数名に動詞を使うのは良くないと言われた。new_row,current_rowを提案されたので、シンプルにcurrent_rowを採用する。(new_rowはオブジェクト指向言語におけるnewと被るので忌避感がある。)
*/

pub struct Solution {}
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        if m <= 0 || n <= 0 {
            panic!("m and n grater than 0 required.");
        }

        let (mut rows, mut cols) = (m as usize, n as usize);

        if rows > cols {
            std::mem::swap(&mut rows, &mut cols);
        }

        let mut tail_row = vec![1; cols];

        for _ in 0..rows - 1 {
            let mut current_row = vec![1; cols];

            for col in (0..cols - 1).rev() {
                current_row[col] = current_row[col + 1] + tail_row[col];
            }

            tail_row = current_row;
        }

        tail_row[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step2_test() {
        assert_eq!(Solution::unique_paths(4, 3), 10);
        assert_eq!(Solution::unique_paths(3, 7), 28);
        assert_eq!(Solution::unique_paths(3, 2), 3);
        assert_eq!(Solution::unique_paths(1, 1), 1);
    }

    #[test]
    #[should_panic]
    fn step2_panic_test() {
        Solution::unique_paths(0, 1);
        Solution::unique_paths(1, 0);
        Solution::unique_paths(0, 0);
    }
}
