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
  https://github.com/naoto-iwase/leetcode/pull/49/changes#diff-3b6e163021d78fcc9ee1d093525ad3943834e1362abebdf326ea2d7e3d5129f0R72
  - 区間start側を閉区間として扱う実装。与えられた文字列sに存在する文字cを見るのであれば、区間の両端は閉区間にしてしまったほうが素直だという感覚は分かると思った。

  https://github.com/Ryotaro25/leetcode_first60/pull/52/changes#diff-f8174339e096bcd076272c3e7cd89b087025459c8fd6c42938166e2f87636a9aR1
  - 問題の制約からASCII文字しか出現しないのでサイズ128の配列を初期化して管理する方法。実装の幅として良いと思った。
  実際に使うようなメソッドでこの実装をするのであれば、メソッド名でASCII文字にしか対応していないことを示唆する命名にするかなと考えた。

  https://github.com/Ryotaro25/leetcode_first60/pull/52/changes#r2003497726
    > ご参考までに共有ですが、リーダブルコードには、begin/endが [begin, end)半開区間、start/lastが [start, last]閉区間のイメージということが記載されていました。
  - 特定の変数名と端点の開閉状態は関係がないように見えるので、個人的には一般化しないほうが良いと思った。
    たとえば、Pythonのstr,findではstart/endが[start,end]で閉区間に対応している。
      https://docs.python.org/ja/3.13/library/stdtypes.html#str.find
    PythonのRangeでは、start/stopが[start,stop)なleft-close,right-openな半開区間になっている。
      https://docs.python.org/ja/3.13/library/stdtypes.html#range
    RustのRangeでは、start/endが[start,end)なleft-close,right-openな半開区間になっている。
      https://doc.rust-lang.org/std/ops/struct.Range.html
    ここまで書いていて思ったが、コーディング規約のようなもので端点の開閉状態に合わせてある程度変数名を統一しましょうという温度感であればむしろ好ましいのかとも思った。
    制約が無く、自由度が高すぎるためにコードレビューで本質的では無い点（2ポインタの変数命名）に時間を使うよりは最初からコーディング規約による制約があるほうが良いという観点。

  改善する時に考えたこと
  - start側を閉区間として扱う実装を行ってみる。

  所感
  - 閉区間で扱うとstart,endともにusizeで扱えるので、区間の中からある値を探すみたいな場合では閉区間の方が自然だと思った。
  - Binary Searchの問題で区間について時間をかけて理解しようとしたおかげで、区間で見るという視点でSliding windowをいい感じに理解できているように感じた。
    2ポインタによる探索範囲（区間）の管理はポインタの更新条件が異なるものの、Binary Searchでの探索範囲管理と同じように見える。
*/

use std::collections::HashMap;

pub struct Solution {}
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut character_to_index = HashMap::new();
        let mut max_substring_length = 0;
        let mut start = 0;

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
    fn step2_test() {
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
