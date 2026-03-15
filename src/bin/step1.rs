// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  問題の理解
  - 文字列sが与えられる。文字列sを符号付き32ビット整数に変換して返す。
  変換アルゴリズムの内容は以下
    - 先頭の空白文字は無視する
    - 符号の判定「-」「+」を行う。符号なしの場合は正の値として扱う
    - 先頭の0はスキップしながら、非数字文字又は文字列末尾に到達するまで正数を読み取る。
    - 文字列から数字文字を読み取れなかったとき、0を返す。
    - 整数が符号付き32ビット整数を超える時は丸め処理を行って返す。

  何を考えて解いていたか
  - 問題の制約に空間計算量の制約はなく、入力の文字列長は最大200文字なので入力の文字列をVecDeque<char>に変換して扱う。
  - 文字列先頭から見ていく
    - if 空白 or or - or + or 0-9 then 整数パース処理に入る
    else return 0
  - 整数バース処理
    - 文字が数字文字列の場合そのままかえす。数字文字でなければbreak
  - 戻り値のチェックを行いi32::MAX,i32::MINの境界を超えるようならそれぞれ丸めて返す
    - 文字列から生成した数値はi64で扱う必要がある

  n = s.len
  時間計算量: O(n)
  空間計算量: O(n)
  この内容で実装する。
  s="  -042"でWrong Answerとなった。先頭の空白が連続した時に全て取り除いて良いことに気づかなかった。入力の先頭空白全てをtrimすることで対応。
  s="20000000000000000000"でWrong Answerとなった。文字列に数字以外が含まれていることに着目してパースするだけだと思っていたが、そのまま数値型にパースするとオーバーフローするような数字文字列を正しく扱えていないことに気付いた。
  そのまま数字文字列をi64型にパースするとオーバーフローするので、パースが失敗した時にそのままi32::MAXまたはi32::MINでreturnすれば良い。
  この修正でAcceptedとなった。

  何がわからなかったか
  - 先頭の空白はスキップするというルールで空白1文字だと思いこんでいた。普通に考えて空白1文字より先頭の空白は全て無視する方が自然なので、問題を解くことに囚われすぎていて視野が狭くなっていたなと思った。
  - 入力の文字列長が200ということで、時間計算量O(200)なら問題ないと考えていた。この問題では数字文字列から符号付き32ビット整数に変換する必要があるので、ここでオーバーフローする可能性がある文字列が入力として与えられる可能性を見落としていた。

  正解してから気づいたこと
  - 問題を見た時に自力で解けるか解けないかギリギリそうだなという感覚があった。余裕がなく視野が狭くなってエッジケースでWrong Answerとなるようなコードを提出する結果になったなと思った。
  - match chars.front()で符号を判定するためだけに重複するコードが発生しているのが気になる。
  - parseで文字列先頭にある0は無視してくれるのでmatchで'0'を見る必要がない。
*/

use std::{collections::VecDeque, i32};

pub struct Solution {}
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        };

        let mut chars = s.trim_start().chars().collect::<VecDeque<_>>();
        let mut is_minus = false;
        let mut num_chars = Vec::new();

        match chars.front() {
            Some('-') => {
                is_minus = true;
                let _ = chars.pop_front();
                Self::collect_num_chars(&mut chars, &mut num_chars);
            }
            Some('+' | '0') => {
                let _ = chars.pop_front();
                Self::collect_num_chars(&mut chars, &mut num_chars);
            }
            Some('1'..='9') => Self::collect_num_chars(&mut chars, &mut num_chars),
            _ => return 0,
        }

        if num_chars.is_empty() {
            return 0;
        }

        match num_chars.iter().collect::<String>().parse::<i32>() {
            Ok(num) => {
                if is_minus {
                    return -num;
                }
                num
            }
            Err(_) => {
                if is_minus {
                    return i32::MIN;
                }
                i32::MAX
            }
        }
    }

    fn collect_num_chars(chars: &mut VecDeque<char>, num_chars: &mut Vec<char>) {
        let Some(c) = chars.pop_front() else {
            return;
        };

        if c.is_ascii_digit() {
            num_chars.push(c);
            Self::collect_num_chars(chars, num_chars);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step1_test() {
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
