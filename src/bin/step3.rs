// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  時間計算量: O(m * n)
  空間計算量: O(max(m, n)) colsは入力の内大きい方を使うので。
*/

/*
  1回目: 3分10秒
  2回目: 2分46秒
  3回目: 2分59秒
*/

pub struct Solution {}
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        if m <= 0 || n <= 0 {
            panic!("m and n grater than 0 required.");
        }

        let (mut rows, mut cols) = (m as usize, n as usize);
        if rows > cols {
            std::mem::swap(&mut rows, &mut cols);
        }

        let mut tail_row = vec![1; cols];

        for _ in 0..rows - 1 {
            let mut current_row = vec![1; cols];

            for col in (0..cols - 1).rev() {
                current_row[col] = current_row[col + 1] + tail_row[col];
            }

            tail_row = current_row;
        }

        tail_row[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step3_test() {
        assert_eq!(Solution::unique_paths(4, 3), 10);
        assert_eq!(Solution::unique_paths(3, 7), 28);
        assert_eq!(Solution::unique_paths(3, 2), 3);
        assert_eq!(Solution::unique_paths(1, 1), 1);
    }

    #[test]
    #[should_panic]
    fn step3_panic_test() {
        Solution::unique_paths(0, 1);
        Solution::unique_paths(1, 0);
        Solution::unique_paths(0, 0);
    }
}
