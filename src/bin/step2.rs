// Step2
// 目的: 自然な書き方を考えて整理する

// 方法
// Step1のコードを読みやすくしてみる
// 他の人のコードを2つは読んでみること
// 正解したら終わり

// 以下をメモに残すこと
// 講師陣はどのようなコメントを残すだろうか？
// 他の人のコードを読んで考えたこと
// 改善する時に考えたこと

/*
  他の人のコードを読んで考えたこと
  - HashMapの操作でメソッドチェーンでキーが見つからなければ、空の配列をインサートして、見つかれば文字列を配列にプッシュする
  といったコードがあったが、自身の経験不足からか少し認知負荷が高いと感じた。
  https://leetcode.com/problems/group-anagrams/solutions/6223523/hash-maps-sorting-anagram-magic-perfect-pwgn2/

  - 以下の例ではハッシュマップでキーが見つかれば変更するといった、and_modify()メソッドを利用していて分かりやすいと思った。
  HashMapの初期化でサイズを明示的に指定しているところが空間計算量の観点から良いと思った。ハッシュマップのサイズは入力の配列の大きさより常に等しいか小さくなるため。
  自身の実装より実行速度が速いのはアナグラムなのかの判断を文字列をソートせず、文字の出現回数を配列で表し、ハッシュマップのキーとすることで
  判定しているものがあった。少し理解するのに時間がかかった。配列をハッシュマップのキーとする引き出しが自分の中に存在しなかったためだと考えた。
  https://leetcode.com/problems/group-anagrams/solutions/3392102/rust-0ms-easy-understanding/

  - アルファベット小文字しか考慮していなかったが、アナグラムかの判定時にto_lower_case()で小文字にそろえてから判定しているコードがあるが、
  leet codeの制約に入力の配列に入る文字列は小文字の英字であると明示されているので不要では？と思った。
  しかし、コーディング練習会の典型コメントなどを見ていると、そもそもleet codeでアクセプトされることを重視するのが目的ではないので、
  このメソッドが何かのシステムで使われることを想定してみると、大文字・小文字混在のケースに対応しておくぐらいのことはしておいても良いのかなどと考えた。

  改善する時に考えたこと
  - anagramかの判定をする場合にソートしているが、大文字の英字にも対応する。
  - HashMapで値が見つからなかったときにand_modify()メソッドを使って意味が通るような書き方にする。
  - HashMapでand_modifyすろと、常にs.clone()が発生するのが気持ち悪いと感じた。キーが見つかる場合と、見つからない場合どちらかの実行パスしか存在しないので、
  常にclone()で複製する必要は無いという理由。String型はヒープ領域上にメモリを確保するので、primitive型のようにclone()によるコストは無視できない。
  - HashMapへの値の追加は常に発生するので、and_modify()ではなく、or_default()を利用する。or_default()は常にミュータブルな参照を返してくる。
  HashMapに対応するキーが無ければデフォルトの型である空のVec<String>が返されるのでこれにsを追加する。
  HashMapに対応するキーがあれば、キーに対応する値(Vec<String>のanagramのグループ)が返されるのでこれにsを追加する。
  この書き方だと無駄なclone()が発生しない。
  - HashMap初期化時に明示的にサイズを指定する。
  - Rustにはstr型があるので、forの変数名でstrとはせずsとする。
*/

use std::collections::HashMap;

pub struct Solution {}
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut group_anagrams_hash_map: HashMap<String, Vec<String>> =
            HashMap::with_capacity(strs.len());
        for s in strs.into_iter() {
            let mut s_chars = s.to_ascii_lowercase().chars().collect::<Vec<_>>();
            s_chars.sort();
            let sorted_s = s_chars.into_iter().collect::<String>();

            group_anagrams_hash_map.entry(sorted_s).or_default().push(s);
        }
        group_anagrams_hash_map.into_values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step2_test() {
        let mut result = Solution::group_anagrams(
            vec!["eat", "tea", "tan", "ate", "nat", "bat"]
                .iter()
                .map(ToString::to_string)
                .collect(),
        );
        let mut expect = vec![vec!["bat"], vec!["nat", "tan"], vec!["ate", "eat", "tea"]];
        //　assertの比較のために結果をソートしている
        result.iter_mut().for_each(|v| v.sort());
        expect.iter_mut().for_each(|v| v.sort());
        result.sort();
        expect.sort();
        assert_eq!(result, expect);

        let mut result =
            Solution::group_anagrams(vec![""].iter().map(ToString::to_string).collect());
        let mut expect = vec![vec![""]];
        //　assertの比較のために結果をソートしている
        result.iter_mut().for_each(|v| v.sort());
        expect.iter_mut().for_each(|v| v.sort());
        result.sort();
        expect.sort();
        assert_eq!(result, expect);

        let mut result =
            Solution::group_anagrams(vec!["a"].iter().map(ToString::to_string).collect());
        let mut expect = vec![vec!["a"]];
        //　assertの比較のために結果をソートしている
        result.iter_mut().for_each(|v| v.sort());
        expect.iter_mut().for_each(|v| v.sort());
        result.sort();
        expect.sort();
        assert_eq!(result, expect);
    }
}
