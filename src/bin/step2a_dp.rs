// Step2a_dp
// 目的: DP実装を練習する

/*
  DP実装の理解
  NeetCodeの解説動画は最終的にDP実装をしている。
  https://www.youtube.com/watch?v=Sx9NNgInc3A
  s="leetcode" word_list=["leet", "code"]
  s=文字列
  - 再帰の基本ケースである、sの最後に到達したら分割できたということをdp[s.len() + 1] = true で表現している。
  - sを後ろから先頭に向けて走査する。
    - 一致する単語を見つけた時
     - 今見ている文字位置iから一致した文字長さ分進めた時に余りの文字が無いかの情報を格納する。
       - 単語末尾(余り無し)がtrueなので、単語末尾までピッタリ分割できる(余りが無い)とtrueと判定される。
     - 入力のword_dictに含まれる単語を使って分割できるかを確認すれば良いので、ある文字位置で分割できる場合は、その文字位置で分割できないことを確認する必要はない。
       - dp[i]がtrueであれば、そのループを抜ける
  - dp[0]まで到達した時に全部分割できたかの情報が入る。
  - 自然言語で表すのがだいぶ難しいと感じた。
  - これはdpテーブルの末尾から先頭に向けて見ているのでトップダウンだろうか。
    GPT-5.1に聞いたところボトムアップアプローチとのこと。
    再帰+メモ化: トップダウンアプローチ
    dp配列: 先頭、末尾どちらから開始するかに関わらずボトムアップアプローチ

  所感
  - breakするときのif分をブロック節のように見せるためにループ先頭に持ってくるか、値のすぐ後にもってくるか迷った。
  ループ先頭に持って行くとsegmentable_words[i]の出現場所がばらばらになり、文脈が分断され読み手の負荷が増えると考えたので、先頭ではなく末尾に置いた。
  - コメント集からdiscordのやり取り見た時に逆順で書けますかみたいなコメント見かけたのでdpを逆順(head -> tail)で見るバージョンも書く。 step2b_dp_head_to_tail.rs
*/

pub struct Solution {}
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut segmentable_words = vec![false; s.len() + 1];
        segmentable_words[s.len()] = true;

        for i in (0..s.len()).rev() {
            for word in &word_dict {
                let Some(peeked_s) = s.get(i..i + word.len()) else {
                    continue;
                };

                if peeked_s != word {
                    continue;
                }

                segmentable_words[i] = segmentable_words[i + word.len()];
                if segmentable_words[i] {
                    break;
                }
            }
        }

        segmentable_words[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step2a_test() {
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
