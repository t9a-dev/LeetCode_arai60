// Step2a_dp
// 目的: DP実装を練習する。DP配列を先頭から末尾に書けて見るバージョン

/*
  所感
  - DP配列を先頭から末尾に向けて走査する方法だと手が止まって解けなかった。時間切れなのでGPT-5.1に聞く
  - 空文字をtrueとしてdp[0]をtrueとする。<- 直感に反するので腹落ちしない
  - dp[0]をtrueとした時、一致する単語の長さdp[i+word.len()] = true　をする前に、それまでに出現した文字列[i+word.len()]がtrueであることを確認する必要がある。
  dp[i+word.len()] がtrueでない時単語を分割しきれていない。 <- ここもよく分からない。
  時間切れなので写経のみとする
*/

pub struct Solution {}
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut segmentable_words = vec![false; s.len() + 1];
        segmentable_words[0] = true;

        for i in 0..s.len() {
            for word in &word_dict {
                let Some(peeked_word) = s.get(i..i + word.len()) else {
                    continue;
                };

                if peeked_word != word {
                    continue;
                }

                if !segmentable_words[i] {
                    break;
                }

                segmentable_words[i + word.len()] = true;
            }
        }

        *segmentable_words.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step2b_test() {
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
