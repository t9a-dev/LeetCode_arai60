// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  Nは入力文字列の長さとする。
  時間計算量: O(N) 入力文字列を全走査する。
  空間計算量: O(N) スタックで利用。
*/

/*
  1回目: 3分12秒
  2回目: 1分41秒
  3回目: 1分26秒
*/

/*
  step2の再帰下降型の解法は練習として書いた。
  以下の理由からstep1の解を採用した。
  - 再帰処理を利用するのでスタックオーバーフローを気にする必要がある
    - そもそも複雑な処理を構造的にわかりやすくするという目的とのトレードオフで再帰処理を採用するという認識
  - step2の再帰下降法の記述量が多いので何をしているのか理解するのに少し時間を要する
  - 採用したスタックでの処理はトリッキーなことをしていないのにもかかわらず記述がシンプルである
    - 視線の移動が少なく全体を把握できる。
*/

pub struct Solution {}
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];

        for c in s.chars() {
            match c {
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                ')' | ']' | '}' if Some(c) != stack.pop() => return false,
                _ => (),
            }
        }

        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step3_test() {
        assert_eq!(Solution::is_valid("[]".to_string()), true);
        assert_eq!(Solution::is_valid("[()]".to_string()), true);
        assert_eq!(Solution::is_valid("[](){}".to_string()), true);
        assert_eq!(Solution::is_valid("[a]a(a)a{a}a".to_string()), true);
        assert_eq!(Solution::is_valid("([)]".to_string()), false);
    }
}
