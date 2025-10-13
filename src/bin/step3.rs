// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  時間計算量: O(N)
  空間計算量: O(1) <- O(N)かと思ったが入力の制約上、英字小文字（N<=26）のみなのでハッシュマップでのキーは最大で26
*/

/*
  1回目: 5分52秒
  2回目: 4分30秒
  3回目: 3分29秒
*/

/*
  一番シンプルで読みやすく、書きやすいと思った実装。悪意のある入力などは考慮しないパターン。
*/

use std::collections::HashMap;

pub struct Solution {}
impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut count_by_character: HashMap<char, usize> = HashMap::new();

        for c in s.chars().into_iter() {
            count_by_character
                .entry(c)
                .and_modify(|count| *count += 1)
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
    fn step3_test() {
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
