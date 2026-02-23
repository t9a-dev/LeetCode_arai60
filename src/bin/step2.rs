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
  コメント集、他の人のコードを読んで考えたこと
  https://github.com/hroc135/leetcode/pull/50/changes#r2052246310
  - strings.Builderについて。C#で使ったことあるなと思った。
  C#ではString型はimmutableなので、文字列結合を行うと新たに結合後の文字列を保持するメモリ領域を毎回確保することになる。
  StringBuilderではmutableなので、一度確保したメモリ領域を伸長しながら追加を行えるので、毎回メモリ確保が走らなくなり、比較的高速に処理できると理解した。
  Go言語でもStringはimmutableなので、文字列結合を頻繁に行うのであればStrings.Builderを利用することで比較的高速になると理解した。
    https://stackoverflow.com/questions/36720693/immutable-strings-in-go
  Rustではデフォルトで変数の値はimmutableであり、mut キーワードによってmutableにすることが可能。
  上記のStack Overflowに貼られている以下のリンクの説明で、Go言語のStringはimmutableなので複数の文字列が同じストレージを共有しても安全ですとある。
   > A string is represented in memory as a 2-word structure containing a pointer to the string data and a length. Because the string is immutable, it is safe for multiple strings to share the same storage,
   https://research.swtch.com/godata
  不変性によりマルチスレッドプログラミングなどの場面で安全性の確保を行っていると理解した。
  Rust   | Go              | 可変性
  ------- ----------------- -----------
  String | strings.Builder | mutable
  &str   | string          | immutable
  マルチスレッドの文脈も関係あるかなと思ってRustドキュメントの並行性の箇所を眺めていたところGo言語の設計思想について言及されていた。
    https://doc.rust-jp.rs/book-ja/ch16-02-message-passing.html
  Go言語における並列プログラミングではメモリを直接共有するのではなく、通信(チャネル)を通じてメモリを共有するという設計思想であることが分かった。
    > Do not communicate by sharing memory; instead, share memory by communicating.
    https://go.dev/doc/effective_go#concurrency
  Rustではチャネル、ロックによる並行性を扱う方法が提供されていることが分かった。ロックによる状態共有しか頭になかったので、色々なモデルがあって面白いなと思った。
  書籍「プログラミングRust 第2版」にも並列性の章でチャネルについて言及されている。面白そうだと思うものの自分の経験してきた範囲では使い所があまり分からないという感じでもある。
  Rustでは所有権システムにより、String(mutable)を複数スレッドで共有できない（コンパイラが弾く）が、Go言語ではstrings.Builderをchannelで共有できてしまう点が面白いと思った。
  Go言語のstrings.Builderでは空でないstrings.Builderをコピーするなとコメントで警告している。
  copyCheck関数で実行時にアドレスの比較をし、コピーを検出してpanicさせている。無秩序にスレッド間でメモリを共有してレースコンディションなどの競合状態を発生させるくらいなら実行時エラーにするという意思を感じる。
    > // Do not copy a non-zero Builder.
    https://go.dev/src/strings/builder.go
  実行時エラーになるコードが書けた。
    https://go.dev/play/p/7bABd2P_uZ3
  Rustはコンパイル時に検出できるからGo言語より優れているという主張をしたわけではなくて、言語仕様や設計哲学による言語間の違いを確認できて面白かった。

  https://github.com/olsen-blue/Arai60/pull/54#discussion_r2020125121
  - 問題の入力と出力が分かっている状態で、出力をどのように網羅的に分類するかという考え方。
  入力に対応する出力のパターンを網羅できるパターンを見つけることが第一段階で、見つけたパターンをコードにできるかということだと理解した。
  step1.rsでは問題からパターンを見つけることができず、実装例からどのようなパターンに分類しているかを理解していた。
  繰り返し練習することで、様々なパターンを分類する能力を鍛えていくんだななどと考えた。

  https://github.com/olsen-blue/Arai60/pull/54#discussion_r2022014586
  - ここのやり取りで言われている「(A)B」と分ける考え方がよく分からないので例示されているコードを写経してみる。(step2a.rs)
    > https://github.com/olsen-blue/Arai60/pull/54#discussion_r2027288220

  https://github.com/olsen-blue/Arai60/pull/54#discussion_r2022554077
  - 0個以上の変数名は複数形にするとより良いのではないかというコメント。同意できるので取り入れる。

  https://github.com/skypenguins/coding-practice/pull/27#discussion_r2533462366
  - 計算量の話（カタラン数）とPythonにおいて文字列はimmutableなので素朴に結合すると計算量が悪化しそうだが、文字列結合はネイティブコードで実行されるのでPythonインタープリター比で十分に速く、気にするほどではないという話。

  - base_caseの条件を if open_brackets_count + close_brackets_count == n * 2 としているコードも見かけたが、2というマジックナンバーを避けたいと思った。

  改善する時に考えたこと
  - open_bracket_count -> open_brackets_countのように複数形にする

  所感
  - backtrackingアルゴリズムとは直接関係のないstrings.Builderについて調べ始めたらかなり横道にそれてしまった感があるが、コーディング練習を通じて知らなかったことを知れたので良しとする。
*/

pub struct Solution {}
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut all_parenthesis = Vec::new();
        let mut parenthesis = String::new();

        Self::make_parenthesis(n, 0, 0, &mut parenthesis, &mut all_parenthesis);

        all_parenthesis
    }

    fn make_parenthesis(
        n: i32,
        open_brackets_count: i32,
        close_brackets_count: i32,
        parenthesis: &mut String,
        all_parenthesis: &mut Vec<String>,
    ) {
        if open_brackets_count == n && close_brackets_count == n {
            all_parenthesis.push(parenthesis.clone());
            return;
        }

        if open_brackets_count < n {
            parenthesis.push('(');
            Self::make_parenthesis(
                n,
                open_brackets_count + 1,
                close_brackets_count,
                parenthesis,
                all_parenthesis,
            );
            parenthesis.pop();
        }

        if close_brackets_count < open_brackets_count {
            parenthesis.push(')');
            Self::make_parenthesis(
                n,
                open_brackets_count,
                close_brackets_count + 1,
                parenthesis,
                all_parenthesis,
            );
            parenthesis.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step2_test() {
        let mut expect = vec!["()"];
        let mut actual = Solution::generate_parenthesis(1);
        expect.sort();
        actual.sort();
        assert_eq!(expect, actual);

        let mut expect = vec!["(())", "()()"];
        let mut actual = Solution::generate_parenthesis(2);
        expect.sort();
        actual.sort();
        assert_eq!(expect, actual);

        let mut expect = vec!["((()))", "(()())", "(())()", "()(())", "()()()"];
        let mut actual = Solution::generate_parenthesis(3);
        expect.sort();
        actual.sort();
        assert_eq!(actual, expect);
    }
}
