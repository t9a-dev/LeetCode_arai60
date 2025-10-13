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
  - 見た目の一文字はグラフィムクラスタ（grapheme cluster）と呼ぶそう。見た目は1文字に見えるが、文字列に含まれているときにインデックスアクセスするとずれるなど問題が起こる。
  マルチバイト文字関連は注意が必要くらいの認識しかなく、実際に絵文字のインデックス位置がずれるという現象に遭遇したのが初めてだったので良い経験になった。
  https://github.com/nanae772/leetcode-arai60/pull/16/files#diff-e308788644c01fa89479cb10fb1675ed0a1d04fd0acdf5153ea12f658889942c

  Rustの場合はunicode-segmentationなるクレートが良さそう
  https://github.com/unicode-rs/unicode-segmentation

  文字列の操作を行うときに入力に絵文字やマルチバイト文字が含まれる場合は、一旦立ち止まって文字コードについて考えられれば良さそう。
  様々な文字コードに対応するような実装は、これ自体が実装の目的でない限り行うべきではない種類の問題だと思った。

  - 文字列が非常に長い場合に入力全てを見るのではなく早期リターンするというのは良い考え方だと思った。
  特に自分は偏ったり、悪意のある入力が行われるケースを考慮しきれていないことが多いように感じているので、こういう視点を意識的に持ちたいなと思った。
  https://github.com/docto-rin/leetcode/pull/15/files#diff-7edcc4c1426405d421fc29bd0711bf94515bc082deab76160ea42efc9b5324d1R45-R46

  - 配列を左右から見ていって同じインデックスであればユニークであるという解法は思いつかなかった。
  https://github.com/docto-rin/leetcode/pull/15/files#diff-7edcc4c1426405d421fc29bd0711bf94515bc082deab76160ea42efc9b5324d1R137-R138

  改善する時に考えたこと
  - 入力が非常に長い場合(DoS攻撃など)を想定して英字小文字が全て2回繰り返されたら早期リターンする。
  - leet codeの入力制約である英字小文字にしか対応できないが、絵文字などを網羅的に対応しようとすると自前で実装すること自体がアンチパターンになると考える。
  このような場合は外部クレートを導入すべきだと考える。
  - ユニークな文字のインデックスを探すループの条件分岐を一行にしてコードの意味としてもわかりやすくする。
  - ユニークな文字が見つからなかったときに-1を返しているが、-1をnot_foundのような変数に入れるか迷った。
  以下の理由から変数に入れるのはやめた。
   - インデックスを返すというコンテキストから-1は無効な値（見つからなかった）であることがわかる
   - 全体的な処理の長さを考えても-1が出てきたからといって読みてがマジックナンバーが出てきたとパニックにならない
   - 何でもかんでも分かりやすい変数名をつけて変数に入れておけばよいというものではない。読む助けにならず冗長になる。
   https://github.com/t9a-dev/LeetCode_arai60/pull/4#discussion_r2422465834
*/

use std::collections::HashMap;

pub struct Solution {}
impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        const CHARACTER_COUNT: u8 = 26;
        let mut repeat_character_count: u8 = 0;
        let mut count_by_character: HashMap<char, usize> = HashMap::new();

        for c in s.chars().into_iter() {
            // 全てのアルファベットが2回出現したら早期リターン
            if CHARACTER_COUNT <= repeat_character_count {
                return -1;
            }

            count_by_character
                .entry(c)
                .and_modify(|count| {
                    *count += 1;
                    // 2回出現したアルファベットをカウント
                    if *count == 2 {
                        repeat_character_count += 1;
                    }
                })
                .or_insert(1);
        }

        for (i, c) in s.chars().enumerate() {
            if count_by_character.get(&c).is_some_and(|count| *count == 1) {
                return i.try_into().unwrap_or(-1);
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step2_test() {
        assert_eq!(Solution::first_uniq_char("leetcode".to_string()), 0);
        assert_eq!(Solution::first_uniq_char("loveleetcode".to_string()), 2);
        assert_eq!(Solution::first_uniq_char("aabb".to_string()), -1);

        assert_eq!(Solution::first_uniq_char("abcdabc".to_string()), 3);
        assert_eq!(Solution::first_uniq_char("春夏秋冬春夏秋".to_string()), 3);
        assert_eq!(Solution::first_uniq_char("✊️✌️✋️✋️✌️✊️".to_string()), -1);
        assert_eq!(Solution::first_uniq_char("itwqbtcdprfsuprkrjkausiterybzncbmdvkgljxuekizvaivszowqtmrttiihervpncztuoljftlxybpgwnjb".to_string()),61);

        // failed left:8, right: 4
        // assert_eq!(Solution::first_uniq_char("✌️✋️✋️✌️✊️".to_string()), 4);
    }
}
