// Step2a
// 目的: 別の解法を練習する

/*
  参考にした解法
  https://github.com/hayashi-ay/leetcode/pull/69/changes#diff-f2b395d63173ac2f2d3c547e8f9e8e07c9acfbeb6b5da935bb28d83d8bcc7a04R145
  https://github.com/Yoshiki-Iwasa/Arai60/pull/64/changes#diff-dbf5e75b50aa4257946026d6539860be248a58856e1b457b4d37445059b05857R21
  if num > i32::MAX / 10 || num == i32::MAX / 10 && digit > i32::MAX % 10
  この条件分岐でなぜオーバーフローを判定できているのかわからないので整理する。
    i32::MAX      = 2 147 483 647
    i32::MAX / 10 = 214 748 364
    i32::MAX % 10 = 7
    - num > i32::MAX / 10
      - for-loopの最後で num *= 10, num += digitしている
      - numが i32::MAX / 10 = 214 748 364 を超えるようであれば、オーバーフローすることが確定している
        - num = 214 748 365　の場合を考えると、桁を上げた時点(2 147 483 650)でオーバーフローとなる
    - num == i32::MAX / 10 && digit > i32::MAX % 10
      - num == i32::MAX / 10
        - 一桁分は余裕があるもののi32::MAXの10進数の最下位桁(1の位)は7なのでdigitが7を超えるとオーバーフローする
      - digit > i32::MAX % 10
        - i32::MAX % 10 = 7となる
        - digitが7を超えるようだとオーバーフローするので、丸める必要がある

  所感
  - オーバーフローの判定部分で何をしているのか最初分からなかったが、落ち着いて分解してみるとどのような気持ちで条件分岐が書かれているのか理解できたので良かった。
*/

pub struct Solution {}
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut index = 0usize;
        let chars = s.chars();

        // skip white space
        for c in chars {
            if c.is_ascii_whitespace() {
                index += 1;
                continue;
            }
            break;
        }
        if index == s.chars().count() {
            return 0;
        }

        // determine sign
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
        let mut num = 0i32;
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
    fn step2a_test() {
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
