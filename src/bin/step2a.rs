// Step2a
// 目的: 別の解法の練習(再帰)

/*
  https://github.com/olsen-blue/Arai60/pull/58/changes#diff-0fa674be955a983fbef3f0f1952784256410e7cd97ff5fa05b6d6b5cfa09d3c0R25
  再帰の実装を写経しておく。

  所感
  - シンプルで分かりやすく選択肢として持っておきたい解法だと思った。
*/

pub struct Solution {}
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let s_chars = s.chars().collect::<Vec<_>>();
        let t_chars = t.chars().collect::<Vec<_>>();

        Self::traverse_s_t(&s_chars, &t_chars, 0, 0)
    }

    fn traverse_s_t(s_chars: &[char], t_chars: &[char], s_index: usize, t_index: usize) -> bool {
        if s_index == s_chars.len() {
            return true;
        }
        if t_index == t_chars.len() {
            return false;
        }

        if s_chars[s_index] == t_chars[t_index] {
            return Self::traverse_s_t(s_chars, t_chars, s_index + 1, t_index + 1);
        }

        Self::traverse_s_t(s_chars, t_chars, s_index, t_index + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step2a_subsequence_test() {
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
    fn step2a_not_subsequence_test() {
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
