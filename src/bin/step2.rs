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
  https://github.com/olsen-blue/Arai60/pull/58#discussion_r2033296195
    > while 1: と書くのはあまり見ないように思います。 whilte True: のほうをよく見ます。
  - 確かに while 1 は初めて見た。

  https://github.com/olsen-blue/Arai60/pull/58/changes#diff-0fa674be955a983fbef3f0f1952784256410e7cd97ff5fa05b6d6b5cfa09d3c0R25
  - 再帰の実装は思いつかなかった。読む練習に良さそう。

  https://github.com/hayashi-ay/leetcode/pull/64/changes#diff-0fa674be955a983fbef3f0f1952784256410e7cd97ff5fa05b6d6b5cfa09d3c0R57
  - 自分は２つのポインタ両方をループの中でインクリメントするよりは、片方をforで単調増加させて自動的に増やすみたいな感覚で頭の中から追い出すのが好きなんだなと思った。

  https://github.com/Yoshiki-Iwasa/Arai60/pull/62/changes#diff-5009dbf0a62afffd2bd2b44a9a2ab6dc842111a7b13503ad1fd3b2ca4ec87e11R32
  - 数え上げるのではなく、見つけ文字をpop()で取り出して、最後に全部取り出せたかを確認している。思いつかなかった。VecDequeを使うとreverseせずにいけるかと思った。

  https://github.com/Yoshiki-Iwasa/Arai60/pull/62/changes#diff-e791ee6de1405ea8b9a39688aba429a08271228a394e40fd01307838a566a994R87
  - Follow upの解法に見える。

  https://github.com/Yoshiki-Iwasa/Arai60/pull/62/changes#diff-e791ee6de1405ea8b9a39688aba429a08271228a394e40fd01307838a566a994R131
  - DPの解法。ぱっと見何をしているかわからない。読むのに時間かかりそう。

  https://github.com/naoto-iwase/leetcode/pull/58/changes#diff-5fd32322bb6a97c24c6f249cdb696a317abc0f8aa8ab79d23c39f17aad61b701R155
  - Follow upの解法。

  - indicesはindexの複数形ということを知った。英語的にはindexesでも良いらしい。Rustではメソッド名にindicesが使われていた。(char:char_indices)
    https://doc.rust-lang.org/std/primitive.str.html#method.char_indices


  改善する時に考えたこと
  - 文字数を数えるのは s.len() ではなくて、s.chars().count() を使う。
  - VecDequeで見つけた文字をpop_front()していき、最終的にすべて取り出せたかでsubsequenceであるかを判定する

  所感
  - よりシンプルになったと思う。
  - 再帰の実装を写経しておく。step2a.rs
    https://github.com/olsen-blue/Arai60/pull/58/changes#diff-0fa674be955a983fbef3f0f1952784256410e7cd97ff5fa05b6d6b5cfa09d3c0R25
  - 2ポインタとwhileで回す解法は少し苦手意識があるので書いておく。step2b.rs
    https://github.com/hayashi-ay/leetcode/pull/64/changes#diff-0fa674be955a983fbef3f0f1952784256410e7cd97ff5fa05b6d6b5cfa09d3c0R57
  - Follow upの解法を写経しておく。
    https://github.com/naoto-iwase/leetcode/pull/58/changes#diff-5fd32322bb6a97c24c6f249cdb696a317abc0f8aa8ab79d23c39f17aad61b701R49

*/

use std::collections::VecDeque;

pub struct Solution {}
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if t.chars().count() < s.chars().count() {
            return false;
        }

        let mut s_chars = VecDeque::from_iter(s.chars());
        for t_char in t.chars() {
            let Some(s_char) = s_chars.front() else {
                break;
            };

            if *s_char == t_char {
                s_chars.pop_front();
            }
        }

        s_chars.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step2_subsequence_test() {
        assert_eq!(
            Solution::is_subsequence("ace".to_string(), "abcde".to_string()),
            true
        );
        assert_eq!(
            Solution::is_subsequence("a".to_string(), "abcde".to_string()),
            true
        );
        assert_eq!(
            Solution::is_subsequence("".to_string(), "".to_string()),
            true
        );
    }

    #[test]
    fn step2_not_subsequence_test() {
        assert_eq!(
            Solution::is_subsequence("aec".to_string(), "abcde".to_string()),
            false
        );
        assert_eq!(
            Solution::is_subsequence("abcde".to_string(), "ace".to_string()),
            false
        );
    }
}
