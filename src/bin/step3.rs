// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  n = s.len();
  m =  HashSet::<_>::from_iter(s.chars().into_iter()).len();
  時間計算量: O(n)
  空間計算量: O(m)
*/

/*
  1回目: 1分49秒
  2回目: 2分11秒
  3回目: 2分4秒
*/

use std::collections::HashMap;

pub struct Solution {}
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max_substring_length = 0;
        let mut start = 0;
        let mut character_to_index = HashMap::new();

        for (end, c) in s.chars().enumerate() {
            if let Some(i) = character_to_index.insert(c, end) {
                start = start.max(i + 1);
            }
            max_substring_length = max_substring_length.max((end - start) + 1);
        }

        max_substring_length as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step3_test() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring("bbbbb".to_string()),
            1
        );
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring("ab abc ab ".to_string()),
            4
        );
        assert_eq!(Solution::length_of_longest_substring("ab".to_string()), 2);
        assert_eq!(Solution::length_of_longest_substring("a".to_string()), 1);
        assert_eq!(Solution::length_of_longest_substring("".to_string()), 0);
    }
}
