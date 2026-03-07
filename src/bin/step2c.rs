// Step2c
// 目的: Follow upの解法を写経しておく

/*
 Follow upの解法を写経しておく。
 https://github.com/naoto-iwase/leetcode/pull/58/changes#diff-5fd32322bb6a97c24c6f249cdb696a317abc0f8aa8ab79d23c39f17aad61b701R155

 解法の理解
 - tが持つある文字にO(1)でアクセスできるようにHashMapを作る
   - 文字をkeyとして、indexを配列としてもつ
 - s_list[i]の文字列sを走査しながら、tのHashMapに文字が含まれているかを見ていく。
   - 文字を見つけたら、tの中での位置(index)が前回よりも大きいかを確認する。
     - 大きければ前回位置を更新
     - 大きくなければ必要な文字を見つけられなかったので早期リターンでfalseを返す
 - 最後まで到達すればtrue

  所感
  - 二分探索を使うのは思いつかなかった。
    - 配列の中からこれまで見た値(index)よりも大きい値の位置を知りたいという問題設定ができれば、線形探索O(n)ではなく二分探索O(log n)を利用する方向が思いつくかなと思った。
*/

use std::collections::HashMap;

pub struct Solution {}
impl Solution {
    /*
      LeetCodeのFollow upの解法です。採点システムではジャッジできません。
      https://leetcode.com/problems/is-subsequence/description/
      > Follow up: Suppose there are lots of incoming s, say s1, s2, ..., sk where k >= 109, and you want to check one by one to see if t has its subsequence. In this scenario, how would you change your code?
    */
    pub fn is_subsequence(s_list: Vec<String>, t: String) -> bool {
        let mut t_char_to_indecies: HashMap<_, Vec<_>> = HashMap::new();
        for (i, c) in t.chars().enumerate() {
            t_char_to_indecies.entry(c).or_default().push(i);
        }

        s_list
            .iter()
            .any(|s| Self::is_subsequence_helper(s, &t_char_to_indecies))
    }

    fn is_subsequence_helper(s: &str, t_char_to_indecies: &HashMap<char, Vec<usize>>) -> bool {
        let mut previous_t_char_index = 0usize;
        for s_char in s.chars() {
            let Some(t_char_indecies) = t_char_to_indecies.get(&s_char) else {
                return false;
            };

            // predicate: *i < previous_t_char_indexの評価結果がtrueとfalseで切り替わる境界が返される
            let t_char_index = t_char_indecies.partition_point(|i| *i < previous_t_char_index);
            if t_char_index == t_char_indecies.len() {
                return false;
            }
            previous_t_char_index = t_char_indecies[t_char_index] + 1;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step2c_subsequence_test() {
        assert_eq!(
            Solution::is_subsequence(
                vec!["abcf", "bcf", "abe"]
                    .into_iter()
                    .map(str::to_string)
                    .collect(),
                "abcde".to_string()
            ),
            true
        );
        assert_eq!(
            Solution::is_subsequence(
                vec!["ag", "bcf", "abe", "aa"]
                    .into_iter()
                    .map(str::to_string)
                    .collect(),
                "abcdea".to_string()
            ),
            true
        );
        assert_eq!(
            Solution::is_subsequence(
                vec!["あいうえおか", "きく", "さしすせそ", "うえ"]
                    .into_iter()
                    .map(str::to_string)
                    .collect(),
                "あいうえお".to_string()
            ),
            true
        );
    }

    #[test]
    fn step2c_not_subsequence_test() {
        assert_eq!(
            Solution::is_subsequence(
                vec!["abcf", "bcf", "def"]
                    .into_iter()
                    .map(str::to_string)
                    .collect(),
                "abcde".to_string()
            ),
            false
        );
        assert_eq!(
            Solution::is_subsequence(
                vec!["あいうえおか", "きく", "さしすせそ", "うえ"]
                    .into_iter()
                    .map(str::to_string)
                    .collect(),
                "てと".to_string()
            ),
            false
        );
    }
}
