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
  - is_splitable_word の命名で悩んでいたが、いくつか選択肢の幅があった。
  is_segmentable,can_be_segmented など
  can_segment,can_split が良いかなと思った。
  https://github.com/olsen-blue/Arai60/pull/39#discussion_r1983043320

  - step1のキャッシュ変数の命名について。
  確かにindex_to_breakable,index_to_segmentableとかのほうが分かりやすいと思った。
  https://github.com/ryosuketc/leetcode_arai60/pull/52/files#r2225024995

  - 入力チェックをすっかり忘れていことに気付いた。
  https://github.com/docto-rin/leetcode/pull/44/files#diff-9942a679b9535897a1d10eaaf67b9243ebbfa70786f9252cefa19d384314c8c9R133-R136

  文字列sが空文字の時にtrueを返しているのがよく分からなのでGPT-5.1に聞いてみる。
  > 空文字列は「単語を 0 個使うことで構成できる」と解釈するため、標準的には
  空文字列は構成可能 → true
  です。

  上記の理由からDP実装で空文字を表すdp[0] = trueと設定しているとのこと。制約上あり得ない入力なのでそこまで重要ではないと思っていたがそんなことはなかった。
  DP的な考え方をするうえで大切な部分だと思った。

  - 「Trie木」という単語を初めて聞いた。ソフトウェアエンジニアの常識には含まれていないとのことで深追いせずメモのみとする。
  https://github.com/hayashi-ay/leetcode/pull/61#discussion_r1536822342
  Wikiで少し説明を見たがよく分からない。
  https://ja.wikipedia.org/wiki/%E3%83%88%E3%83%A9%E3%82%A4_(%E3%83%87%E3%83%BC%E3%82%BF%E6%A7%8B%E9%80%A0)

  - step1で問題の理解と、どういう気持で処理を行っているのかは分かったと思うのでDPテーブルによる解法も練習する。
  > というわけで、先頭から DP が"模範解答"だろうな、とは思います。
  https://discord.com/channels/1084280443945353267/1200089668901937312/1221781262109380699

  改善する時に考えたこと
  - 文字列の内、ある区間の文字列を表す変数名はsplited_sよりもpeeked_sの方が良さそう。
    - これに付随してs_start_index,s_end_indexよりはpeeked_start,peeked_endの方が良さそう
  - 入力チェックを行う。テストケースを追加する。
  - is_splitable_word -> can_segment
  - splitable_word_cache -> index_to_segmentable
  - anyのクロージャ内でキャッシュの登録を行っていたが、早期リターンした場合にキャッシュされないことに気付いたので修正した。
  チェックしようとしている文字列区間の開始位置から分割可能であるかの結果をキャッシュしているので、区間ごとに見るために外側でキャッシュするのが正しい。
  - word_dict.into_iter()はword_dict.iter()にするべき。共有参照(&T)しか使わないのが分かっているので。

  所感
  - 最近コメント集を見に行くのを忘れていることに気付いたので忘れないようにする。
  レビュー依頼するときの癖で、直近のレビュー依頼のPull Requestを見ていたがコメント集から見ていくようにする。
  - 正直この解法が一番自然に感じるが、DP実装も練習のために実装する。step2a_dp.rs
*/

use std::collections::HashMap;

pub struct Solution {}
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        if s.is_empty() {
            return true;
        }
        if word_dict.is_empty() {
            return false;
        }

        let mut index_to_segmentable = HashMap::new();
        Self::can_segment(&s, 0, &word_dict, &mut index_to_segmentable)
    }

    fn can_segment(
        s: &str,
        peeked_start: usize,
        word_dict: &[String],
        index_to_segmentable: &mut HashMap<usize, bool>,
    ) -> bool {
        if s.len() <= peeked_start {
            return true;
        }

        if let Some(segmentable) = index_to_segmentable.get(&peeked_start) {
            return *segmentable;
        }

        let segmentable = word_dict.iter().any(|word| {
            let peeked_end = peeked_start + word.len();
            let Some(peeked_s) = s.get(peeked_start..peeked_end) else {
                return false;
            };

            if peeked_s != word {
                return false;
            }

            Self::can_segment(s, peeked_end, word_dict, index_to_segmentable)
        });
        index_to_segmentable.insert(peeked_start, segmentable);

        segmentable
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step2_test() {
        let s = "leetcode".to_string();
        let word_list = vec!["leet", "code"]
            .into_iter()
            .map(str::to_string)
            .collect();
        assert_eq!(Solution::word_break(s, word_list), true);

        let s = "applepenapple".to_string();
        let word_list = vec!["apple", "pen"]
            .into_iter()
            .map(str::to_string)
            .collect();
        assert_eq!(Solution::word_break(s, word_list), true);

        let s = "cars".to_string();
        let word_list = vec!["car", "ca", "rs"]
            .into_iter()
            .map(str::to_string)
            .collect();
        assert_eq!(Solution::word_break(s, word_list), true);

        let s = "catsandog".to_string();
        let word_list = vec!["cats", "dog", "sand", "and", "cat"]
            .into_iter()
            .map(str::to_string)
            .collect();
        assert_eq!(Solution::word_break(s, word_list), false);

        let s = "".to_string();
        let word_list = vec!["cats", "dog", "sand", "and", "cat"]
            .into_iter()
            .map(str::to_string)
            .collect();
        assert_eq!(Solution::word_break(s, word_list), true);

        let s = "abc".to_string();
        let word_list = Vec::new();
        assert_eq!(Solution::word_break(s, word_list), false);

        let s = "".to_string();
        let word_list = Vec::new();
        assert_eq!(Solution::word_break(s, word_list), true);
    }
}
