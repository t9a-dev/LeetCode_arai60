// Step1a
// 目的: 別の解法を実装してみる

/*
  問題の理解
  - 英数記号と空白で構成される文字列sが与えられる。
  重複する文字の存在しない最長部分文字列の長さを返す。
  部分文字列の定義として空白でない文字が連続していること。

  解法の理解
  https://leetcode.com/problems/longest-substring-without-repeating-characters/solutions/7417480/rust-implementation-by-rajeshkumarrobert-23fo/
  - 2ポインタで見ている区間(window)の始点と終点を管理しているイメージ。
  - 区間には重複した文字がない状態を維持している。
  - start = -1　とすることで開区間での初期化になっている。
    - end - start で区間の文字数を数えられる
    - start = start.max(i) とした時にiは過去に重複した文字の位置が入る。startは開区間として扱っているのでiをそのまま代入してもその位置の文字は区間に含まないといった扱いになると理解。
      | = startの境界位置
      - [|'a','b','a'] <- end = 0,start = -1
      - [|'a','b','a'] <- end = 1,start = -1
      - ['a',|'b','a'] <- end = 2, start = 0
  - HashMapで出現した文字cとsにおける位置を記録する。
  - 文字列sの文字cを先頭から見ていく。
    - 文字cを毎回HashMapにinsertする。HashMap.insertはキーが存在する時、値を更新する。更新前の値が戻り値となる。
      - 過去に文字cが登録されていて、その位置が区間内であれば、文字cの位置まで区間を狭める。(start = start.max(i))
  - 区間内の文字数を数えて最大文字数を更新する。
    - 不変条件として区間内には重複する文字が存在しない。
*/

use std::collections::HashMap;

pub struct Solution {}
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut character_to_index = HashMap::new();
        let mut max_substring_length = 0;
        let mut start = -1isize;

        for (end, c) in s.chars().enumerate() {
            if let Some(i) = character_to_index.insert(c, end) {
                start = start.max(i as isize);
            }
            max_substring_length = max_substring_length.max(end as isize - start);
        }

        max_substring_length as i32
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn step1a_playground() {
        let character_types = HashSet::<_>::from_iter("abca".chars().into_iter()).len();
        assert_eq!(character_types, 3);

        let mut maps: HashMap<char, i32> =
            HashMap::from_iter([('a', 0), ('b', 1), ('c', 2), ('d', 3)]);
        // HashMap::insertは重複するキーがある場合は値が更新される。戻り値として、更新前の値が返される。
        assert_eq!(maps.insert('a', 4), Some(0));
        // 更新されている。
        assert_eq!(maps.get(&'a'), Some(&4));
    }

    #[test]
    fn step1a_test() {
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
