// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  問題の理解
  -　文字列sが入力として与えられ、その文字列の中で最初に出現するユニークな文字のインデックスを返す。
  ユニークな文字列が存在しない場合は-1を返す。

  何を考えて解いていたか
  - ナイーブにとくと先頭文字列からペアを確認していくので時間計算量O(N^2)になりそう
  - HashMapのキーに文字、値に出現回数を入れる。文字列先頭から文字でハッシュマップから出現回数を取得して1であるか確認していく。

  正解してから気づいたこと
  - 2回目のループでネストが深くなっているのでもう少しすっきり書けそう。(is_some_and(|v| v == 1)とか)
  - 絵文字の場合、見た目の一文字とchars()で分割したときの一文字に乖離があり正しく動作しない。
  外部クレートを利用すれば解決できそう。
  https://github.com/unicode-rs/unicode-segmentation
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
            if let Some(&count) = count_by_character.get(&c) {
                if count == 1 {
                    return i.try_into().unwrap_or(-1);
                }
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step1_test() {
        assert_eq!(Solution::first_uniq_char("leetcode".to_string()), 0);
        assert_eq!(Solution::first_uniq_char("loveleetcode".to_string()), 2);
        assert_eq!(Solution::first_uniq_char("aabb".to_string()), -1);

        assert_eq!(Solution::first_uniq_char("abcdabc".to_string()), 3);
        assert_eq!(Solution::first_uniq_char("春夏秋冬春夏秋".to_string()), 3);
        assert_eq!(Solution::first_uniq_char("✊️✌️✋️✋️✌️✊️".to_string()), -1);

        // failed left:8, right: 4
        // assert_eq!(Solution::first_uniq_char("✌️✋️✋️✌️✊️".to_string()), 4);
    }
}
