// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  問題の理解
  - 文字列sと文字列を含む配列word_dictが与えられる。sを1つ以上のword_dictの単語のスペース区切りのシーケンスに分割できる場合にtrueを返す。

  何がわからなかったか
  - まず問題の内容（何が求められているのか）が理解できなかったのでChatGPT(GPT-5.1)に聞いて理解を進めた。

  何を考えて解いていたか
  - s="leetcode" word_dict=["leet", "code"] output=true
  - s="applepenapple" word_dict=["apple", "pen"] output=true
  - s="catsandog" word_dict=["cats", "dog", "sand", "and", "cat"] output=false
  文字列sからword_dictの単語に一致するワードを全て取り出せたらtrueという感じだろうか。
  文字列sからword_dictの単語に一致するワードを取り出していって、文字列sに取り出せない文字が残ったらfalseという方向で考える。
  文字列のメソッドに疎いのでRustの公式ドキュメントを読む。指定した文字列から指定した文字列を除いて残りを返すようなメソッドがあると良さそう。
  str::replaceメソッドでword_dictに含まれる単語をsで見つけたら空文字で置換して、最後にs.is_empty()を返せば実現できそう。
  https://doc.rust-lang.org/std/primitive.str.html#method.replace
  str::replaceメソッドは文字を全部見る必要があるので、線形探索O(s.len())だと見積もる。
  word_dictも線形探索するので、二重ループになり全体の時間計算量はO(s.len() * word_list.len())になる。
  入力の制約からs.len() <= 300, wordDict.len() <= 1000 なので300_000 ナイーブな実装でも大丈夫そう。
  手元のテストケースは通ったので、別のテストケースを考えてからLeetCode採点システムに提出する。
  エッジケースのようなものは、catsandogのケースでカバーされているように見えるので特に別のケースを思いつかないのでLeetCode採点システムに提出する。
  ```rust
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
      let mut s = s;
      for word in word_dict {
          s = s.replace(&word, "");
      }
      s.is_empty()
    }
  ```
  s="cars" word_dict=["car", "ca", "rs"] output=true　このテストケースでWrong Answerとなった。
  s="catsandog" word_dict=["cats", "dog", "sand", "and", "cat"] output=false このテストケースもtrueになるべきでは？と思いよく分からない。
  問題を理解できていないのでChatGPT(GPT-5.1)に聞いてみる。

  問題の理解(Chat-GPT(GPT-5.1)に問題の解説を聞いた後)
  - 文字列sと文字列から構成される配列word_listが与えられる。
    - word_listに含まれる単語を使って、文字列sを分割したときに、文字列sに余計な文字列が残らないように分割できればtrueを返す。
      - s="cars" word_dict=["car", "ca", "rs"]
        - 単語"car"で区切ると文字列sに"s"が残り分割しきれない。単語"ca"で区切ると文字列sに"rs"が残り単語"rs"と一致するので最終的に文字列sに余分な文字列が残らないのでtrue
      - s="catsandog" word_dict=["cats", "dog", "sand", "and", "cat"]
        - 単語"cats"で分割すると残りの文字列は"andog"となり、word_listに含まれる単語で分割できるものはない。どの単語で分割しても文字列sが空になるように分割しきれないのでfalse

  何を考えて解いていたか(Chat-GPT(GPT-5.1)に問題の解説を聞いた後)
  - 手が止まったので解答を見て理解する

  想定ユースケース
  - 英語のスペルチェックとかで使えそうだなと思った。存在する英単語をword_listとして、文章をsとする。文章sを単語で分割しきれるかでスペルミスを検出する。

  解答の理解
  - LeetCodeのSolutionsのRust実装を見たが、一次元DP配列に値を突っ込んでいくコードばかりでどういう気持ちでこのコードを書いているのか理解できなかった。
  NeetCodeの解説動画を見て、再帰による実装を行ってみる
  https://www.youtube.com/watch?v=Sx9NNgInc3A
  再帰処理
  - 基本ケース
    - s.len() <= i であればtrueを返す
  - 再帰ケース
    - word_listのword.len()と等しい長さ分の文字列をs[i..i+word.len()]から取り出した時に一致するかをword_listのword全てで確認する。
      - 一致した時 i を i+word.len() に更新して再帰処理に入る。
      - 一致しない時 は　falseとして扱う　再帰処理に入らない
    or で結果をまとめる。１つでもtrueになるパスがあれば、word_listのwordでsが分割できるため。

  この考え方で再帰+memo化のコードを実装してAccept

  正解してから気づいたこと
  - メソッド命名 is_splitable_word はもう少し良い命名がある気がするものの思いつかない
  - 文字列の区間を表すs_start_index,s_end_indexは少し冗長な気がするが、許容範囲かなという感覚。短くしたところで得られるメリットもあまりなさそう。
  - 再帰+メモ化をする前に計算量見積もりをするべきだった。
*/

use std::collections::HashMap;

pub struct Solution {}
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut splitable_word_cache: HashMap<usize, bool> = HashMap::new();
        Self::is_splitable_word(&s, 0, &word_dict, &mut splitable_word_cache)
    }

    fn is_splitable_word(
        s: &str,
        s_start_index: usize,
        word_list: &[String],
        splitable_word_cache: &mut HashMap<usize, bool>,
    ) -> bool {
        if s.len() <= s_start_index {
            return true;
        }

        if let Some(cache) = splitable_word_cache.get(&s_start_index) {
            return *cache;
        }

        word_list.iter().any(|word| {
            let s_end_index = s_start_index + word.len();
            let Some(splited_s) = s.get(s_start_index..s_end_index) else {
                return false;
            };

            if splited_s != word {
                return false;
            }

            let is_splitable =
                Self::is_splitable_word(s, s_end_index, word_list, splitable_word_cache);
            splitable_word_cache.insert(s_start_index, is_splitable);

            return is_splitable;
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn playground() {
        let s = "leetcode";
        assert_eq!(&s[4..8], "code");
    }

    #[test]
    fn step1_test() {
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
    }
}
