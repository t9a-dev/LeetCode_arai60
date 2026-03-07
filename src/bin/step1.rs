// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  問題の理解
  - 文字列s,tが与えられるのでsがtの部分文字列であればtrueを返す。そうでなければfalseを返す。
  s="ace",t="abcde" out=true
  s="aec",t="abcde" out=false

  何を考えて解いていたか
  - 文字sのi番目の文字を表すポインタと、文字tのj番目の文字を表すポインタをそれぞれ用意する。
  - tの方が文字列長が長いので、tをループで回す。
    - 問題の制約上はあり得ないが、s.len < t.lenを関数最初に確認すると良さそう。
    - s.len, t.len = 0のときはtrueになるのかが分からない。問題文からも読み取れない。部分列ではないように見えるのでfalseとして扱う。
  - s[i] == t[j]を見つけるたびにiをインクリメントする。
  - 関数最後で i == s.len を返す。
  このロジックで解けるなら計算量は以下になりそう。
  n = t.len
  時間計算量: O(n)
  空間計算量: O(1)
  入力の制約上は t.len < 10 ^ 4
  10 ^ 4 / 10 ^ 8 = 0.0001 = 0.1msとなり問題ない
  実装しようとして気付いたが与えられる文字列の型がStringなのでVec<char>にすると空間計算量: O(n)になった。
  s="",t=""のテストケースでWrong Answerとなった。ここはtrueだった。
  入力が空文字のときにfalseとするエッジケースの条件分岐を消してAcceptedとなった。

  何がわからなかったか
  - 空文字は空文字のsubsequenceであるということが分からなかった。

  正解してから気づいたこと
  - 今回の問題では入力の制約からアルファベット小文字のみなので、s.as_bytes()[i]のようにアクセスすれば空間計算量はO(1)になると思った。
    - ただし、マルチバイト文字が渡されるとこの実装は破綻する。1文字の単位を正しく扱えないため。
    - is_subsequenceがString型であることを考えると、s.as_bytes()[i]による文字参照はやっかいなバグを埋め込むことになりそうなので忌避感がある。
  - t.len() < s.len() は文字列のバイト数を比較する。自分が考えていた用途では文字数を数えたかったので正しくは if t.chars().count() < s.chars().count() とするべき。
  - Leet CodeのFollow Upについて。
    - 入力として与えられるsが複数ある場合にコードをどう変えるかという内容。sの個数の上限は 10 ^ 9
    - そのまま外側にループを足すと、n = t.len, m = s_list.len として、O(n * m)
      - 10 ^ 4 * 10 ^ 9 / 10 ^ 8 = 100,000秒(約27時間)となり、バッチ処理とかでなければ現実的ではない。
    - s_listの文字列s_list[i]の文字s[i]と対応するindexでHashMapを作る。
      - sのi文字目をkey,i文字目に出現する文字を HashMap<usize,HashSet<char>> に詰めていくイメージ
      - HashMapのkeyはkey数100(sの最大文字列長さ),valueはアルファベット小文字26種類が入る。
    ここまで考えて、最終的にsubsequenceかどうかをどのように判定すればよいか分からなくて手が止まったのでスキップ
*/

pub struct Solution {}
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if t.len() < s.len() {
            return false;
        }

        let s_chars = s.chars().collect::<Vec<_>>();
        let mut s_char_index = 0;
        for t_char in t.chars() {
            let Some(s_char) = s_chars.get(s_char_index) else {
                break;
            };

            if *s_char == t_char {
                s_char_index += 1;
            }
        }

        s_char_index == s_chars.iter().count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step1_subsequence_test() {
        assert_eq!(
            Solution::is_subsequence("ace".to_string(), "abcde".to_string()),
            true
        );
        assert_eq!(
            Solution::is_subsequence("a".to_string(), "abcde".to_string()),
            true
        );
        assert_eq!(
            Solution::is_subsequence("".to_string(), "".to_string()),
            true
        );
    }

    #[test]
    fn step1_not_subsequence_test() {
        assert_eq!(
            Solution::is_subsequence("aec".to_string(), "abcde".to_string()),
            false
        );
        assert_eq!(
            Solution::is_subsequence("abcde".to_string(), "ace".to_string()),
            false
        );
    }
}
