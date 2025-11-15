// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  時間計算量: O(n)
  空間計算量: O(1)
*/

/*
  1回目: 2分48秒
  2回目: 3分02秒
  3回目: 3分41秒
*/

/*
  所感
  - ほぼ暗記しているので数週間後には解けなさそう。
*/

pub struct Solution {}
impl Solution {
    pub fn num_ways(n: i32, k: i32) -> i32 {
        if n <= 0 || k <= 0 {
            return 0;
        }

        let mut tail_two_same = k;
        let mut tail_two_diff = k * (k - 1);

        match n {
            1 => return tail_two_same,
            2 => return tail_two_same + tail_two_diff,
            _ => (),
        }

        for _ in 3..=n {
            let prev_tail_two_same = tail_two_same;
            let prev_tail_two_diff = tail_two_diff;

            tail_two_same = prev_tail_two_diff;
            tail_two_diff = (prev_tail_two_same + prev_tail_two_diff) * (k - 1);
        }

        tail_two_same + tail_two_diff
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step3_test() {
        assert_eq!(Solution::num_ways(3, 2), 6);
        assert_eq!(Solution::num_ways(2, 2), 4);
        assert_eq!(Solution::num_ways(3, 1), 0);
        assert_eq!(Solution::num_ways(4, 2), 10);

        // --- basic cases ---
        assert_eq!(Solution::num_ways(1, 1), 1);
        assert_eq!(Solution::num_ways(1, 2), 2);
        assert_eq!(Solution::num_ways(1, 3), 3);
        assert_eq!(Solution::num_ways(2, 1), 1);
        assert_eq!(Solution::num_ways(2, 3), 9);

        // --- constraint cases ---
        assert_eq!(Solution::num_ways(3, 1), 0);
        assert_eq!(Solution::num_ways(4, 1), 0);
        assert_eq!(Solution::num_ways(5, 1), 0);
        assert_eq!(Solution::num_ways(3, 3), 24);

        // --- larger values ---
        assert_eq!(Solution::num_ways(4, 3), 66);
        assert_eq!(Solution::num_ways(5, 2), 16);
        assert_eq!(Solution::num_ways(5, 3), 180);

        // --- n = 0 をどう扱うか（学習用） ---
        assert_eq!(Solution::num_ways(0, 1), 0);
        assert_eq!(Solution::num_ways(0, 3), 0);
    }
}
