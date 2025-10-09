// Step5
// 目的: step3の時間・空間計算量の見積もりをする

use std::collections::HashMap;

/*
  空間計算量は入出力を含まない補助空間計算量とする。
  入力(strs)のサイズをN、Nの要素である文字列平均長さをLとする。

  時間計算量: O(N * L log L) ループ内で行っているソート処理が支配的
  空間計算量: O(N * L) ハッシュテーブルのメモリ利用が支配的
*/

pub struct Solution {}
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        // 空間計算量: O(N * L)
        let mut group_anagrams_hash_map: HashMap<String, Vec<String>> =
            HashMap::with_capacity(strs.len());
        for s in strs.into_iter() {
            // 時間計算量: O(N) 反復の時間計算量

            // 時間計算量: O(L) 文字列を分解して全ての文字を小文字に変換するして集める
            // 補助空間計算量: O(L) 文字列を分解したときに文字を保持する。
            let mut s_chars = s.to_ascii_lowercase().chars().collect::<Vec<_>>();

            // 時間計算量: O(L log L) ソート処理
            // 補助空間計算量: O(N) ソート処理で利用するメモリ
            s_chars.sort();

            // 時間計算量: O(L) 文字を全て走査して文字列にする
            // 補助空間計算量: O(L) 文字列を保持するメモリ
            let sorted_s = s_chars.iter().collect::<String>();

            // 時間計算量: O(L) 固定長でない文字列をキーに追加するときにハッシュ化関数を通り、文字列全てを走査するので
            // 補助空間計算量: O(N)　バケットのメモリ利用
            group_anagrams_hash_map.entry(sorted_s).or_default().push(s);
        }

        // 時間計算量: O(N) 入力のサイズと等しい
        // 補助空間計算量: O(N) 入力のサイズと等しい
        group_anagrams_hash_map.into_values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step5_test() {
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
