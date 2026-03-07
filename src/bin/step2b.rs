// Step2b
// 目的: 別の解法の練習(2ポインタ+while)

/*
  https://github.com/hayashi-ay/leetcode/pull/64/changes#diff-0fa674be955a983fbef3f0f1952784256410e7cd97ff5fa05b6d6b5cfa09d3c0R57
  2ポインタの実装を写経しておく。

  所感
  - 解法を理解した状態だったのでスムーズに実装できた。
*/

pub struct Solution {}
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if t.chars().count() < s.chars().count() {
            return false;
        }

        let s_chars = s.chars().collect::<Vec<_>>();
        let t_chars = t.chars().collect::<Vec<_>>();
        let mut s_char_index = 0;
        let mut t_char_index = 0;

        while s_char_index < s_chars.len() && t_char_index < t_chars.len() {
            if s_chars[s_char_index] == t_chars[t_char_index] {
                s_char_index += 1;
            }
            t_char_index += 1;
        }

        s_char_index == s_chars.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step2b_subsequence_test() {
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
    fn step2b_not_subsequence_test() {
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
