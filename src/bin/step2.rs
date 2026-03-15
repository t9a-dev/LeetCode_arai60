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
  https://github.com/Ryotaro25/leetcode_first60/pull/64#discussion_r2014577471
    > 漢字などでも True になるのは Python の isdigit の話ですね。
  - isdigitでTrueになるのは罠すぎると思ったが、言語によっては数値文字であるという当たり前の前提が異なるから、日本語話者の自分の直感と違うことは当たり前でもあるかと思った。
    Stack Overflowで関連する質問があった。深追いはしないが、文字周りの判定はややこしいので入力文字種に制限が無いような環境では慎重になるべきポイントだと理解した。
    https://stackoverflow.com/questions/44891070/whats-the-difference-between-str-isdigit-isnumeric-and-isdecimal-in-pyth

  https://github.com/shining-ai/leetcode/pull/59#discussion_r1577212861
  - 符号付き32ビットから桁溢れするかどうかを計算して求めている。
  自分はparseできるかどうかで判定していたのでコーディング練習の観点では、オーバーフローするかを桁数に思いを馳せながら考えるのも良いなと思った。

  https://github.com/hayashi-ay/leetcode/pull/69#discussion_r1548091225
    > long はデータモデルによってサイズが異なります。
    > https://ja.wikipedia.org/wiki/64%E3%83%93%E3%83%83%E3%83%88
    > LP64 LLP64 等でお調べください。
  - プログラミング言語のデータ型・ビットサイズという文脈でデータモデルという言葉を始めて聞いた。
    https://www.ibm.com/docs/ja/zos/3.2.0?topic=dbile-ilp32-lp64-data-models-data-type-sizes
    Rustを書いている中ではあまり気にする場面が思いつかなかったのでGPT-5.3に聞いたところ、Rustの外側との境界で問題が表面化することが分かった。
      - 例としてC言語で書かれた既存ライブラリを活用(FFI)するときに、C言語側でlongと書かれているシグネチャの部分をRust側でi64と決め打ちすると問題が発生する場合がある。
        - LLP64データモデルにおいてlong型は長さが32bitだが、Rust側では符号付き64bitとして扱っているため。
    歴史的な経緯までは追いきれなかったが、64bitアーキテクチャにおいてWindowsカーネルはLLP64を採用していることが分かった。
      - Windowsカーネル(LLP64)ではlong型は32bitになる。
      - Linuxカーネル(LP64)ではlong型は64bitになる。
    https://learn.microsoft.com/ja-jp/windows/win32/winprog64/abstract-data-models
      > ほとんどのアプリケーションではサイズを増やす必要がないため、すべてのデータ型を 64 ビット長にすると、領域が無駄になります。 ただし、アプリケーションには 64 ビット データへのポインターが必要であり、選択したケースでは 64 ビットのデータ型を持つ機能が必要です。 これらの考慮事項により、LLP64 (または P64) と呼ばれる抽象データ モデルが選択されました。 LLP64 データ モデルでは、ポインターのみが 64 ビットに拡張されます。他のすべての基本データ型 (整数と long) は、長さが 32 ビットのままです。

  https://github.com/mamo3gr/arai60/pull/54/changes#r2882808316
    > 仮に自分がこの問題を面接で出題するとしたら、 int() の実装をするようお願いすると思います。おそらくここがこの問題のポイントの一つなのではないかと思います。
  - step1.rsの自分のコードにも当てはまる指摘だと思った。i32に収まるかどうかの判定を自分で実装した方が良さそう。step2a.rsでやる。

  改善する時に考えたこと
  - match chars.front()の+,-マッチ条件で実行しているコードが重複しているのを改善する
  - 関数冒頭の空文字チェックは必要なさそう
  - is_minusよりはis_negativeの方が英語として自然そう

  所感
  - parse::<i32>()でオーバーフローするのかどうか、先頭についている0の処理などを丸投げしている罪悪感（コーディング練習になっていない）があるのでstep2a.rsでこのあたりを実装する。
    - VecDequeに詰め込まずにin-placeでも実装できそうなのでやってみる
*/

use std::collections::VecDeque;

pub struct Solution {}
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut chars = s.trim_start().chars().collect::<VecDeque<_>>();
        let mut is_negative = false;

        match chars.front() {
            Some('-') => {
                is_negative = true;
                chars.pop_front();
            }
            Some('+') => {
                chars.pop_front();
            }
            Some('0'..='9') => (),
            _ => return 0,
        }

        let digits = chars
            .iter()
            .map_while(|c| c.is_ascii_digit().then_some(c))
            .collect::<String>();
        if digits.is_empty() {
            return 0;
        }

        match digits.parse::<i32>() {
            Ok(num) => {
                if is_negative {
                    return -num;
                }
                num
            }
            Err(_) => {
                if is_negative {
                    return i32::MIN;
                }
                i32::MAX
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn play_ground() {
        assert_eq!("00042".parse::<i32>().unwrap(), 42);
    }

    #[test]
    fn step2_test() {
        assert_eq!(Solution::my_atoi(" -042".to_string()), -42);
        assert_eq!(Solution::my_atoi("1337c0d3".to_string()), 1337);
        assert_eq!(Solution::my_atoi("0-1".to_string()), 0);
        assert_eq!(Solution::my_atoi("words and 987".to_string()), 0);

        assert_eq!(Solution::my_atoi("".to_string()), 0);
        assert_eq!(Solution::my_atoi("   -042".to_string()), -42);
        assert_eq!(
            Solution::my_atoi("20000000000000000000".to_string()),
            i32::MAX
        );
    }
}
