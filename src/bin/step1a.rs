// Step1a
// 目的: step1でテストケースを通らないコードが出来上がったので答えを見て実装する

/*
  参考にした実装の理解
  一番上のadjacency list + BFSを見て何をしているのか理解する。
  https://leetcode.com/problems/word-ladder/solutions/1765599/rust-4-implementations-150ms-150ms-30ms1-s55f/

  - 自分のコードとほぼやっていることはほぼ同じ。
  単語のカウント=BFSで探索している階層となっていて、探索した階層にある単語を全て探索済としてHashSetに入れている。
  階層は1からカウントし始めている。begin_wordから確認し始めるのでこの時点でカウントは1になる。（正し、end_wordがword_listに存在するとき）

  - 一文字違いの判定方法が間違っている。
  intersectionで集合を取ると、同じ文字が重複して取り除かれるので正しく判定できない。
  "aba","aaa"は一文字だけ違うので隣接すべきだが、HashSetにすると、set("a","b"),set("a")となり、intersectionの結果のカウントは1、元の文字列長さとの差が2となり隣接として判定できない。

  正解してから気づいたこと
  - intersectionの動き方を誤って理解していたのが致命的だった。普段使わないのでしっかりとドキュメントを確認するべきだった。
  - DFS,BFSは理解したと思っていたが、木構造の階層をカウントするといった発想がなかった。
  - このナイーブな実装だとLeet Code上の採点システムで100~200ms前後であり、より速い実装がある。
    - より速い実装例もあるようだがだいぶ時間を使ったのでスキップ
      - 殆どの問題が一発で解けないが、解けそうに見える問題が増えて来たこともあり、変に粘ってしまって時間を浪費している気がするのでもう少し早めに切り上げようと思った。
*/

use std::collections::{HashMap, HashSet, VecDeque};

pub struct Solution {}
impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        if begin_word.is_empty() || end_word.is_empty() || word_list.is_empty() {
            return 0;
        }

        let mut adjacency_words_by_word: HashMap<&str, Vec<&str>> = HashMap::new();
        let mut merged_word_list = vec![begin_word.clone()];
        merged_word_list.extend_from_slice(word_list.as_slice());

        for (i, word1) in merged_word_list.iter().enumerate() {
            for word2 in merged_word_list.iter().skip(i + 1) {
                if !Self::is_one_diff(&word1, &word2) {
                    continue;
                }

                adjacency_words_by_word
                    .entry(&word1)
                    .and_modify(|words| words.push(&word2))
                    .or_insert(vec![&word2]);
                adjacency_words_by_word
                    .entry(&word2)
                    .and_modify(|words| words.push(&word1))
                    .or_insert(vec![&word1]);
            }
        }

        let mut visited_words: HashSet<&str> = HashSet::new();
        let mut laddering_words = VecDeque::new();
        laddering_words.push_back((begin_word.as_str(), 1));

        while let Some((word, depth)) = laddering_words.pop_front() {
            if word == &end_word {
                return depth;
            }

            if !visited_words.insert(&word) {
                continue;
            }

            let Some(words) = adjacency_words_by_word.get(word) else {
                continue;
            };

            for word in words {
                laddering_words.push_back((word, depth + 1));
            }
        }
        0
    }

    fn is_one_diff(a: &str, b: &str) -> bool {
        if a.len() != b.len() {
            return false;
        }

        let mut diff_count = 0;
        for (ac, bc) in a.chars().zip(b.chars()) {
            if diff_count > 1 {
                return false;
            }

            if ac != bc {
                diff_count += 1;
            }
        }

        diff_count == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step1a_ladder_word_judge_test() {
        // ladder
        assert!(Solution::is_one_diff("abc", "abx"));
        assert!(Solution::is_one_diff("abx", "abc"));
        assert!(Solution::is_one_diff("abc", "dbc"));
        assert!(Solution::is_one_diff("dbc", "abc"));
        assert!(Solution::is_one_diff("code", "codx"));
        assert!(Solution::is_one_diff("codx", "code"));
        assert!(Solution::is_one_diff("axc", "abc"));
        assert!(Solution::is_one_diff("abc", "axc"));

        // not ladder
        assert!(!Solution::is_one_diff("abc", "abc"));
        assert!(!Solution::is_one_diff("hit", "cog"));
        assert!(!Solution::is_one_diff("cog", "hit"));
        assert!(!Solution::is_one_diff("", ""));
    }

    #[test]
    fn step1a_test() {
        let (begin_word, end_word, word_list) = (
            "hit",
            "cog",
            vec!["hot", "dot", "dog", "lot", "log", "cog"]
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>(),
        );
        assert_eq!(
            Solution::ladder_length(
                begin_word.into(),
                end_word.into(),
                word_list.into_iter().collect()
            ),
            5
        );

        let (begin_word, end_word, word_list) = (
            "a",
            "c",
            vec!["a", "b", "c"]
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>(),
        );
        assert_eq!(
            Solution::ladder_length(
                begin_word.into(),
                end_word.into(),
                word_list.into_iter().collect()
            ),
            2
        );

        let (begin_word, end_word, word_list) = (
            "leet",
            "code",
            vec!["lest", "leet", "lose", "code", "lode", "robe", "lost"]
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>(),
        );
        assert_eq!(
            Solution::ladder_length(
                begin_word.into(),
                end_word.into(),
                word_list.into_iter().collect()
            ),
            6
        );
    }

    #[test]
    fn step1a_not_exists_end_word_test() {
        let (begin_word, end_word, word_list) = (
            "hit",
            "cog",
            vec!["hot", "dot", "dog", "lot", "log"]
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>(),
        );
        assert_eq!(
            Solution::ladder_length(
                begin_word.into(),
                end_word.into(),
                word_list.into_iter().collect()
            ),
            0
        );
    }

    #[test]
    fn step1a_invalid_input_test() {
        let (begin_word, end_word, word_list) = (
            "hit",
            "",
            vec!["hot", "dot", "dog", "catcat", "lot", "log"]
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>(),
        );
        assert_eq!(
            Solution::ladder_length(
                begin_word.into(),
                end_word.into(),
                word_list.into_iter().collect()
            ),
            0
        );

        let (begin_word, end_word, word_list) = (
            "",
            "",
            vec![""]
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>(),
        );
        assert_eq!(
            Solution::ladder_length(
                begin_word.into(),
                end_word.into(),
                word_list.into_iter().collect()
            ),
            0
        );
    }
}
