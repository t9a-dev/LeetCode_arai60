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
  https://github.com/olsen-blue/Arai60/pull/61/changes#diff-ccfa5b3e70552f7a1aea1f6a719cd00d74a1035c89cfab6426fddc4696ed3e21R56
  - 行を進める方向(direction)を1・-1で管理するのかboolによって管理するのかで結構別れている様子。boolとdirectionをenumで定義する方向で実装してみるのも良さそうだと思った。
  書いていて思ったが、direction自体は関数の外側に露出しないのでenumは過剰な気がしてきた。boolによるフラグ管理で十分そう。

  https://github.com/saagchicken/coding_practice/pull/22/changes#r2009508424
    > この問題、出題意図は、お手玉できるか、な気もします。
  - 同じようなことを思った。何をしようとしているかを理解して、これを素直にプログラムに落とし込めるかという感じ。

  https://github.com/naoto-iwase/leetcode/pull/61#discussion_r2529679629
    > row_index など、行番号のニュアンスがあっても良いかなと思いました。row の場合、内容を読む前だと rows との対応関係がありそうにも見えます。
  - 確かにrowsに対してrowだと row = rows[i]のようにも見えるなと思った。

  https://leetcode.com/problems/zigzag-conversion/solutions/333761/rust-0ms-4ms-by-obliquemotion-ceg6/
  LeetCode Solutionのトップにあった解法。読みづらいので練習としては良さそう。

  改善する時に考えたこと
  - for-loopの中で row = ((row as i32) + direction) as usize; の部分がキャストでごちゃごちゃしているのをなんとかしたい。
    - rowをi32として扱えば少し良くなりそう。

  所感
  - step1.rsと比べて少し行数は増えたものの読み手の認知負荷が下がったように感じる。
*/

pub struct Solution {}
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let num_rows: usize = match num_rows.try_into() {
            Ok(v) => v,
            Err(_) => panic!("num_rows must be positive value: num_rows: {}", num_rows),
        };

        if num_rows == 0 {
            return "".to_string();
        }
        if num_rows == 1 || s.chars().count() <= num_rows {
            return s;
        }

        let mut rows = vec![String::new(); num_rows];
        let mut i = 0;
        let mut is_down_direction = true;

        for c in s.chars() {
            rows[i as usize].push(c);

            if is_down_direction {
                i += 1;
            } else {
                i -= 1;
            }

            if i == 0 || i == num_rows - 1 {
                is_down_direction = !is_down_direction;
            }
        }

        rows.concat()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn step2_num_rows_negative_value_test() {
        Solution::convert("PAYPALISHIRING".to_string(), -3);
    }

    #[test]
    fn step2_test() {
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 3),
            "PAHNAPLSIIGYIR"
        );

        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 4),
            "PINALSIGYAHRPI"
        );

        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 1),
            "PAYPALISHIRING"
        );

        assert_eq!(Solution::convert("P".to_string(), 3), "P");
    }
}
