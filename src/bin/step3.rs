// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  n = t.len
  m = s.len
  時間計算量: O(n)
  空間計算量: O(m)
*/

/*
  1回目: 2分20秒
  2回目: 1分58秒
  3回目: 1分42秒
*/

/*
  所感
  - 文字列を文字に分割して扱う点でgrapheme clusterを思い出した。
  問題の制約では英字小文字しか入ってこないが、文字列という広い制約だった場合、見た目の一文字(grapheme cluster)と複数のコードポイントで表されるUnicode文字などに思いを馳せながら実装しないとやっかいなバグになりそう。
  自分で実装するのではなく、適切な場面で外部ライブラリを使うなどの判断ができればよいと思った。
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
    fn step3_subsequence_test() {
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
    fn step3_not_subsequence_test() {
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
