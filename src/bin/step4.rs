// Step4
// 目的: より良い書き方を試す

/*
  n = s.len
  時間計算量: O(n)
  空間計算量: O(1)
*/

/*
  改善点
  - step3.rsでs.chars()が何度も出てくるのでまとめる。(let mut s_iter = s.chars().peekable())
  - is_negative は sign = -1 or 1で持っておくと値を返す時の条件分岐が不要になる。(num * sign)

  所感
  - peekableなイテレータは便利だなと思った。次の要素を確認だけしたい(peek)ときにイテレータを進めずに見れるので。
*/

pub struct Solution {}
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        // skip white space
        let mut s_iter = s.trim_start().chars().peekable();

        // set sign
        let mut sign = 1;
        match s_iter.peek() {
            Some('-') => {
                sign = -1;
                s_iter.next();
            }
            Some('+') => {
                s_iter.next();
            }
            _ => (),
        }

        // to i32
        let mut num = 0i32;
        for c in s_iter {
            let Some(digit) = c.to_digit(10).and_then(|v| Some(v as i32)) else {
                break;
            };

            if num > i32::MAX / 10 || num == i32::MAX / 10 && digit > i32::MAX % 10 {
                if sign == -1 {
                    return i32::MIN;
                }
                return i32::MAX;
            }

            num *= 10;
            num += digit;
        }

        num * sign
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step4_test() {
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
