// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  問題の理解
  -　文字列の配列が渡されるので、文字列のうち同じアルファベットで構成されているものをグループ化して同じ配列にまとめて返す。

  何を考えて解いていたか
  - 1.Two-Sumで利用したアルゴリズムと同じ感じでハッシュマップのキーにソートしたアルファベットを入れていき、キーと重複するワードを
  値として入れていく感じで解けそう

  正解してから気づいたこと
  - if let Someの部分をmatch文にすると値が見つかったときの操作と、そうでなかった時の操作が自然に書けそう
  - group_anagrams_hash_mapの操作でentry(),get()でどちらもキーによる参照しているので、可読性が落ちるかもしれないが
  ワンライナーでかけるかも
  - Rustにはstr型があるので、forの変数名でstrを使うのは気持ち悪いかも
*/

use std::collections::HashMap;

pub struct Solution {}
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut group_anagrams_hash_map: HashMap<String, Vec<String>> = HashMap::new();
        for str in strs.into_iter() {
            let mut str_chars = str.chars().collect::<Vec<_>>();
            str_chars.sort();
            let sorted_str = str_chars.iter().collect::<String>();

            // 対応するキーがなければ、キーを登録して値には空の配列をセット
            group_anagrams_hash_map
                .entry(sorted_str.clone())
                .or_insert(vec![]);

            if let Some(group) = group_anagrams_hash_map.get_mut(&sorted_str) {
                group.push(str);
            }
        }
        group_anagrams_hash_map.into_values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step1_test() {
        let mut result = Solution::group_anagrams(
            vec!["eat", "tea", "tan", "ate", "nat", "bat"]
                .iter()
                .map(ToString::to_string)
                .collect(),
        );
        let mut expect = vec![vec!["bat"], vec!["nat", "tan"], vec!["ate", "eat", "tea"]];
        //　assertの比較のために結果をソートしている
        result.iter_mut().for_each(|v| v.sort());
        result.sort();
        expect.iter_mut().for_each(|v| v.sort());
        expect.sort();
        assert_eq!(result, expect);

        let mut result =
            Solution::group_anagrams(vec![""].iter().map(ToString::to_string).collect());
        let mut expect = vec![vec![""]];
        //　assertの比較のために結果をソートしている
        result.iter_mut().for_each(|v| v.sort());
        expect.iter_mut().for_each(|v| v.sort());
        expect.sort();
        assert_eq!(result, expect);

        let mut result =
            Solution::group_anagrams(vec!["a"].iter().map(ToString::to_string).collect());
        let mut expect = vec![vec!["a"]];
        //　assertの比較のために結果をソートしている
        result.iter_mut().for_each(|v| v.sort());
        expect.iter_mut().for_each(|v| v.sort());
        expect.sort();
        assert_eq!(result, expect);
    }
}
