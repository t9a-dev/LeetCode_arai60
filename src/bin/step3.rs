// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  n = s.len
  時間計算量: O(n)
  空間計算量: O(1)
*/

/*
  1回目: 6分28秒 overflowチェックの条件分岐をミスしてWrong Answerとなった
  2回目: 4分39秒 Accepted
  3回目: 5分19秒 Accepted
  4回目: 5分06秒 Accepted
*/

/*
  所感
  - s.chars()が何度も出現しているのが気になるものの特に代替案が思い浮かばないので、GPT-5.3に聞いてみる。
    - let mut iter = s.chars().peekable()が使えそうなことが分かったのでstep4.rsで書いてみる。
    - is_negativeはsignにして num * signとすると条件分岐を減らせるという指摘もあった。
    - as_bytes()のコードを提案された。確かに入力の制約上はマルチバイト文字は考慮しなくてよいが、文字列を文字に分解するという目的ではchars()の方が適切な気がするのでこのままとする。
*/

pub struct Solution {}
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut index = 0;

        // skip white space
        for c in s.chars() {
            if !c.is_ascii_whitespace() {
                break;
            }
            index += 1;
        }
        if index == s.chars().count() {
            return 0;
        }

        // parse sign
        let mut is_negative = false;
        match s.chars().nth(index) {
            Some('-') => {
                is_negative = true;
                index += 1;
            }
            Some('+') => index += 1,
            _ => (),
        }
        if index == s.chars().count() {
            return 0;
        }

        // to i32
        let mut num = 0;
        for i in index..s.chars().count() {
            let c = s.chars().nth(i).unwrap();
            let Some(digit) = c.to_digit(10).and_then(|v| Some(v as i32)) else {
                break;
            };

            if num > i32::MAX / 10 || num == i32::MAX / 10 && digit > i32::MAX % 10 {
                if is_negative {
                    return i32::MIN;
                }
                return i32::MAX;
            }

            num *= 10;
            num += digit;
        }

        if is_negative {
            return -num;
        }
        num
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step3_test() {
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
