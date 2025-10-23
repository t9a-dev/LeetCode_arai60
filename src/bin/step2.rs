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
  講師陣はどのようなコメントを残すだろうか？
  - 具体的な案は浮かばないが、再帰処理のメソッド名が分かりづらいなど。

  他の人のコードを読んで考えたこと
  - 再帰下降に言及されているコメントがあり、学習目的でコンパイラを作るときに使った手法だと思うので、再帰下降型でも書いてみる。
  https://github.com/akmhmgc/arai60/pull/6/files#r2292629454

  - Valid Parenthesesはチョムスキー階層、タイプ-2、文脈自由文法を連想するのは常識らしい。自分はどれも知らなかった。
  https://discord.com/channels/1084280443945353267/1201211204547383386/1202604857664475227
  以下でまとめられている。
  https://github.com/bumbuboon/Leetcode/pull/7/files#diff-d6ab6ab43ae442a1d7f23d67893348a226b06cf4bee29194b31eb5498da4f268R83-R102

  その他
  - 「速さの問題よりも、余計なことができるということは余計なことが必要であるという意図であろうと判断されながら読まれるということかと思います。」
  以前自分が行った実装でHashMapを初期化するとき改善にならないのにwith_capacityでサイズ指定していたことを思い出した。
  https://github.com/SanakoMeine/leetcode/pull/7#discussion_r1903352357

  改善する時に考えたこと
  - 再帰下降型の実装を行う。step1の解法は改善する点が見当たらず、ここで繰り返し書いても練習にならないと考えた。
  - 現状、再帰処理の読み書き書きが難しいのでChatGPTに実装を書いてもらった。実装を何回か写経して見なくても書けるように練習する。
  - match文のパターンマッチ部分はmacro_ruleにできるが、再帰処理を理解することが目的なのでやらない。
  - 再帰処理のメソッド名がどちらもあまりしっくりこないので、ChatGPTに聞いたもののどれもしっくりこないので、自身の理解が追いついていないのが原因な気がする。
  - 再帰処理を追っていくと頭のワーキングメモリからあふれる。
  再帰処理の読み方のコツをChatGPTに相談したところ、「処理の流れを追わずに、構造としてそれぞれのメソッドが何をしているのかを見る」という案が出た。
  この方法で少し理解が進んだので良い視点だと思った。
  - 記号のハードコーディングが気になるが、設定ファイルから読み込むといった要件がなければ、メソッド内に閉じているのでそこまで気にすることでもないかと思った。

  再帰下降パーサの理解と所感
  open='(','[','{'、close=')',']','}'
  - 文字列を先頭から再帰処理で見ていく。再帰処理終了時に走査したカウントが文字列の長さと等しいか（全部チェックしたか）確認する。
  全部チェックできていなければ、最後まで文字列を走査する前に再帰処理が中断されている（不正なペアの発見）のでfalse。
   - open文字を見つけたら対応するclose文字かを確認していく。
   はじめは対応するclose文字は存在しない（open文字を見つけていないので）Noneになる。
     - close文字を探しているときにopenを見つけたら再帰処理に入って最初に戻る。
     - open,close文字どちらでもない場合はカウントだけしてスキップする。（LeetCodeの制約上ありえないが、これくらいの対応はしておいても良いだろうと思った。）

  - 自分が思いついたナイーブな実装では見つけた文字を取り除こうとしていたので、数え上げれば記述量が減りシンプルになると感心した。
  - while chars.next()の使いすぎで思いつかなかったが、任意のタイミングで現在文字を進めていく手法は応用が効くと思った。
  - 答えを隠して実装できるが覚えて書いている感じであり、別の問題で再帰処理の構造を設計できる気はしない。
  ここで足踏みしても仕方がないのでとりあえず次に進む。
*/

use std::usize;

pub struct Solution {}
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let chars = s.chars().collect::<Vec<_>>();
        let mut i = 0;

        if !Self::parse_brackets(&chars, &mut i, None) {
            return false;
        }

        chars.len() == i
    }

    fn parse_brackets(chars: &[char], i: &mut usize, close_bracket: Option<char>) -> bool {
        while let Some(c) = chars.get(*i) {
            if let Some(close_bracket) = close_bracket {
                if *c == close_bracket {
                    *i += 1;
                    return true;
                }
            }

            match c {
                '(' | '[' | '{' => {
                    if !Self::parse_bracket_group(chars, i) {
                        return false;
                    }
                }
                ')' | ']' | '}' => return false,
                _ => *i += 1,
            };
        }

        close_bracket.is_none()
    }

    fn parse_bracket_group(chars: &[char], i: &mut usize) -> bool {
        if let Some(open_bracket) = chars.get(*i) {
            let close_bracket = match open_bracket {
                '(' => ')',
                '[' => ']',
                '{' => '}',
                _ => return false,
            };

            *i += 1;
            return Self::parse_brackets(chars, i, Some(close_bracket));
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step2_test() {
        assert_eq!(Solution::is_valid("[]".to_string()), true);
        assert_eq!(Solution::is_valid("[()]".to_string()), true);
        assert_eq!(Solution::is_valid("[](){}".to_string()), true);
        assert_eq!(Solution::is_valid("[a]a(a)a{a}a".to_string()), true);
        assert_eq!(Solution::is_valid("([)]".to_string()), false);
    }
}
