// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  n = word_list.len()
  m = word_list[i].len()
  時間計算量: O(n^2 * m) adjacency_words_by_word構築のループでn^2かつ、このループ内で隣接ワードなのか判定を行う際に文字列の全走査をしている。
  空間計算量: O(n^2) word_list単語を利用して隣接ワードのペアを作成している。
*/

/*
  1回目: 9分13秒
  2回目: 7分51秒
  3回目: 分秒 <-今回は時間無いのでスキップ
*/

/*
  所感
  - この問題を見たときに解けそうだなと思い、かなり時間を使って粘ってしまった。
  結果として他の人のコードを読んだり、別の実装を行うことに時間を使えなかったのでもっと早く答えを見て先にstepを進めるべきだったと反省した。
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

        merged_word_list.extend_from_slice(&word_list);
        for (i, word1) in merged_word_list.iter().enumerate() {
            for word2 in merged_word_list.iter().skip(i + 1) {
                if !Self::is_adjacency_word(&word1, &word2) {
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

        let mut visited_words = HashSet::new();
        let mut laddering_word = VecDeque::new();

        laddering_word.push_back((begin_word.as_str(), 1));
        while let Some((word, depth)) = laddering_word.pop_front() {
            if word == &end_word {
                return depth;
            }

            if !visited_words.insert(word) {
                continue;
            }

            let Some(words) = adjacency_words_by_word.get(word) else {
                continue;
            };

            for word in words {
                laddering_word.push_back((word, depth + 1));
            }
        }

        0
    }

    fn is_adjacency_word(a: &str, b: &str) -> bool {
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
    fn step3_ladder_word_judge_test() {
        // ladder
        assert!(Solution::is_adjacency_word("abc", "abx"));
        assert!(Solution::is_adjacency_word("abx", "abc"));
        assert!(Solution::is_adjacency_word("abc", "dbc"));
        assert!(Solution::is_adjacency_word("dbc", "abc"));
        assert!(Solution::is_adjacency_word("code", "codx"));
        assert!(Solution::is_adjacency_word("codx", "code"));
        assert!(Solution::is_adjacency_word("axc", "abc"));
        assert!(Solution::is_adjacency_word("abc", "axc"));

        // not ladder
        assert!(!Solution::is_adjacency_word("abc", "abc"));
        assert!(!Solution::is_adjacency_word("hit", "cog"));
        assert!(!Solution::is_adjacency_word("cog", "hit"));
        assert!(!Solution::is_adjacency_word("", ""));
    }

    #[test]
    fn step3_test() {
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
    fn step3_not_exists_end_word_test() {
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
    fn step3_invalid_input_test() {
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
