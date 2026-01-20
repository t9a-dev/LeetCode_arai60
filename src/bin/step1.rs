// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  問題の理解
  - 英数記号と空白で構成される文字列sが与えられる。
  重複する文字の存在しない最長部分文字列の長さを返す。
  部分文字列の定義として空白でない文字が連続していること。

  何を考えて解いていたか
  - 手作業でやることを考えた時に思いついた実装方法としてはs[i]からスタートして重複する文字または空白を見つけるまで出現した文字を数え上げる方法。
  s.len()としたときに、時間計算量O(n ^ 2)になる。入力の制約から文字列の最大長さは(5 * 10 ^ 4) ^ 2 = 2,500,000,000となり計算量が爆発する。
  ナイーブな実装ではTime Limit Exceededとなりそう。
  他の方法が思いつかないのでナイーブな実装だけしてみる。
  空白の扱いでWrong Answerとなった。
  「A substring is a contiguous non-empty sequence of characters within a string.」 のnon-emptyが空白文字に言及していると思っていた。
  修正してAcceptedとなった。
  事前に頭の中で思い描いていた実装とは異なり、s[i..j].contains[&s[j]]により時間計算量がO(N^3)となったものの採点システム側では時間計算量として許容範囲内だった理由が分からなかった。

  何がわからなかったか
  - 時間計算量の見積もり自体は行えたが、求めた時間計算量から組み合わせ爆発が起きると予想したがそうではなかった。
  - LeetCodeの採点システムがどの程度の実行時間を許容するかが事前に分からないという点は置いておいて、どの程度の実行時間がかかるかを事前に見積もれれば組み合わせ爆発ではないことが判断できたかどうか。

  正解してから気づいたこと
  - Big-O記法における時間計算量の見積もりはできていたが、実際に値を当てはめるときにミスをしていた。GPT-5.2に聞きながらまとめた。
  入力の制約からsの長さは 0 <= s.length < 5 * 10 ^ 4となるが、sは英数記号と空白文字なので多く見積もってprintable ASCIIの文字種類数95と見積もれる。
    https://www.ascii-code.com/characters/printable-characters
  n = s.len(), m = 95 として、自分の実装の時間計算量を計算すると、O(n * (m ^ 2) / 2) となる。
  二重ループの内側のcontainsによる計算は等差数列の和なので、95(95 - 1) / 2 となり、大体 (95 ^ 2) / 2と考えられる。
  ループがbreakされるまでの重複のない文字が連続するのが95であるため、jは最長で95になる。
  時間計算量の概算の数式に当てはめると、(5 * (10 ^ 4)) * ((95 ^ 2) / 2) / 10 ^ 8 = 約2.3秒
  実行時間としては長いものの、2.3秒であれば現実的な実行時間ではあるかと思った。
  LeetCode採点システムでTime Limit Exceededしそうな実行時間ではあると思った。
  ただし、かなり大雑把な見積もりで2.3秒という値が出てきた時にどう判断するのかがまだ良くわからない。
  今回は秒あたり1億ステップ(10 ^ 8)として見積もったが、秒あたり10億ステップ(10 ^ 9)とすると0.23秒 = 230msとなるので、Time Limit Exceededとはならないように見える。

  10 ^ 8 は秒あたり1億ステップ命令を実行できるという概算。(RustなのでC++に近い速度がでるという想定)
    https://github.com/Yuto729/LeetCode_arai60/pull/16#discussion_r2602118324

  実行時間の概算見積もりについて、コメント集などから参考にしたもの
  - https://github.com/Yuto729/LeetCode_arai60/pull/16#discussion_r2602118324
  - https://discord.com/channels/1084280443945353267/1200089668901937312/1235490680592273410

  簡易的なベンチマークとの比較
    手元の環境(Apple M4(10 cores))での簡易的なベンチマーク(デバッグビルド)は平均59.874msとなった。
    秒あたり10億ステップの概算では230msとなったので、ベンチマーク結果と4倍程度の差となった。
    見積もろうとしている計算時間のスケールを考えると、概算にしては良いところまで見積もれているように思った。

  所感
  - 文字種類のような値が時間計算量に関わってくると難しく感じる。
  - Time Limit ExceededによるWrong Answerだと思っていたが、たまたまAcceptedになったという感じだった。
  - 通らないと思っていたコードが採点システムを通ったので横道にそれてしまったがSliding window自体を知らないのでこの解法による実装を写経した方が良さそう。
*/

use std::time::Instant;

pub struct Solution {}
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.len() <= 1 {
            return s.len() as i32;
        }

        let s = s.as_bytes();
        let mut max_length_of_substring = 1;

        for i in 0..s.len() {
            let mut unique_characters_substring_length = 1;

            for j in i + 1..s.len() {
                if s[i..j].contains(&s[j]) {
                    break;
                }

                unique_characters_substring_length += 1;
                max_length_of_substring =
                    max_length_of_substring.max(unique_characters_substring_length);
            }
        }

        max_length_of_substring
    }
}

// GPT-5.2によるベンチマークコード
// “最悪寄せ”入力（表示可能ASCIIのみ）
fn gen_adversarial_printable(n: usize) -> String {
    let sigma = 128usize; // ASCII 0..127 は1バイトUTF-8として合法
    let mut bytes = Vec::<u8>::with_capacity(n);

    while bytes.len() < n {
        for k in 0..sigma {
            // 巡回シフトしたユニーク列（長さ sigma）
            for t in 0..sigma {
                bytes.push(((k + t) % sigma) as u8);
                if bytes.len() == n {
                    break;
                }
            }
            if bytes.len() == n {
                break;
            }

            // “最後の1バイト”をもう一回（末尾に重複を置く）
            bytes.push(((k + sigma - 1) % sigma) as u8);
            if bytes.len() == n {
                break;
            }
        }
    }

    String::from_utf8(bytes).unwrap()
}

// GPT-5.2によるベンチマークコード
fn main() {
    // 入力サイズ（LeetCode制約上限）
    let n = 50_000usize;

    // ベンチ回数：1回だけだとブレるので数回回して平均を見る
    let iters = 5usize;

    // 入力生成（ここは計測に入れない）
    let s = gen_adversarial_printable(n);
    println!("input length = {}", s.len());

    // ウォームアップ（最適化・キャッシュ等のため）
    let warm = Solution::length_of_longest_substring(s.clone());
    println!("warmup answer = {}", warm);

    // 計測開始
    let start = Instant::now();

    // 同じ入力で複数回（clone は計測に入るが、関数シグネチャが String なので仕方ない）
    // clone のコストも含めた「実際の提出に近い」時間になる
    let mut sum = 0i64;
    for _ in 0..iters {
        let ans = Solution::length_of_longest_substring(s.clone());
        sum += ans as i64; // 最適化で消されないように使う
    }

    let elapsed = start.elapsed();

    // 結果表示
    println!("iters = {}", iters);
    println!("sum(ans) = {}", sum);
    println!("elapsed = {:.3?}", elapsed);
    println!("avg/iter = {:.3?}", elapsed / (iters as u32));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step1_test() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring("bbbbb".to_string()),
            1
        );
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring("ab abc ab ".to_string()),
            4
        );
        assert_eq!(Solution::length_of_longest_substring("a".to_string()), 1);
        assert_eq!(Solution::length_of_longest_substring("".to_string()), 0);
    }
}
