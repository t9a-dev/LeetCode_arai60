// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  問題の理解
  - 文字列begin_word,end_wordと文字列を持つ配列word_listが与えられる。
  word_listの文字列はそれぞれ隣接する文字列と一文字異なるような文字列で構成されている。
  word_list = ["hot","dot","dog","lot","log","cog"]
  begin_word= "hit",end_word="cog"のとき、word_listに含まれるend_wordへの最短変換シーケンスの単語数を返す。
  hit->hot->dot->dog->cogとなり戻り値は5となる。

  何がわからなかったか
  - begin_word="a",end_word="c",word_list=["a","b","c"]の場合にcountが2になる理由が分からなかった。
  問題の仕様を理解できていないことが原因だと思われる。ChatGPTと壁打ちして理解する。
   - 単語の数を返すところをエッジの数を返していた。ただ、問題を理解したところでこのケースに対応する方法がわからなかったので答えをみてstep1a.rsで実装する。

  何を考えて解いていたか
  - word_listから隣接リストのHashMapを作る。keyに単語、valueにkeyの単語の隣接単語リスト
  - Group Anagramので行ったように、単語をソートして隣接ワードなのか判定できないか？差の文字が常に末尾に来ないと無理だ。
    intersectionで文字列長さ-1と等しいか見ればよさそう
  -　隣接リスト関係のHashMapがうまく作れればBFSで探索する。
  深さ優先探索(DFS)だと最初に見つけた経路でend_wordまでのカウントを出してしまうので最短距離を求められなさそう。
  BFSでend_word見つけたらすぐに探索を終了したい。
*/

use std::collections::{HashMap, HashSet, VecDeque};

pub struct Solution {}
impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        /*
          このコードは以下の入力ケースをパスできません。
          begin_word="a",end_word="c",word_list=["a","b","c"]
          記録のために残しています。
        */

        /*
          正解した後に気づいたが、そもそもintersectionで単語の隣接を行っているというバグがある。
          なので、このコードは読むだけ時間の無駄だと思います。
        */
        let mut adjacency_words_by_word: HashMap<String, Vec<String>> = HashMap::new();
        let mut merged_word_list = vec![begin_word.clone()];
        merged_word_list.extend_from_slice(&word_list);

        for (i, word1) in merged_word_list.iter().enumerate() {
            let hashed1: HashSet<char> = HashSet::from_iter(word1.chars());

            for word2 in merged_word_list.iter().skip(i + 1) {
                let expect_diffirence_count = 1;
                let hashed2: HashSet<char> = HashSet::from_iter(word2.chars());

                if hashed1
                    .intersection(&hashed2)
                    .count()
                    .abs_diff(word1.chars().count())
                    != expect_diffirence_count
                {
                    continue;
                }

                adjacency_words_by_word
                    .entry(word1.clone())
                    .and_modify(|words| words.push(word2.clone()))
                    .or_insert(vec![word2.clone()]);
                adjacency_words_by_word
                    .entry(word2.clone())
                    .and_modify(|words| words.push(word1.clone()))
                    .or_insert(vec![word1.clone()]);
            }
        }

        if !adjacency_words_by_word.contains_key(&end_word) {
            return 0;
        }

        let mut visited_words = HashSet::new();
        Self::explore_ladder_word(
            &adjacency_words_by_word,
            begin_word,
            end_word,
            &mut visited_words,
        )
    }

    fn explore_ladder_word(
        adjacency_words_by_word: &HashMap<String, Vec<String>>,
        begin_word: String,
        end_word: String,
        visited_words: &mut HashSet<String>,
    ) -> i32 {
        let mut ladder_count = 0;
        let mut ladder_words = VecDeque::new();
        ladder_words.push_back(begin_word);

        while let Some(word) = ladder_words.pop_front() {
            if !visited_words.insert(word.clone()) {
                continue;
            }

            let Some(words) = adjacency_words_by_word.get(&word) else {
                break;
            };

            ladder_count += 1;

            if words.contains(&end_word) {
                break;
            }

            for word in words {
                ladder_words.push_back(word.clone());
            }
        }

        ladder_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step1_test() {
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

        // fail
        // let (begin_word, end_word, word_list) = (
        //     "a",
        //     "c",
        //     vec!["a", "b", "c"]
        //         .iter()
        //         .map(ToString::to_string)
        //         .collect::<Vec<String>>(),
        // );
        // assert_eq!(
        //     Solution::ladder_length(
        //         begin_word.into(),
        //         end_word.into(),
        //         word_list.into_iter().collect()
        //     ),
        //     2
        // );
    }

    #[test]
    fn step1_not_exists_end_word_test() {
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
}
