// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  n = nums.len()
  時間計算量: O(log n)
  空間計算量: O(1)
*/

/*
  1回目: 4分36秒
  2回目: 3分21秒
  3回目: 4分16秒
*/

/*
  所感
  - コード量に対して時間がかかっている気がするが、暗記せず毎回ロジックを考える部分に時間がかかっているのでむしろ良い傾向だと思った。
*/

pub struct Solution {}
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return -1;
        }

        let mut start = 0;
        let mut end = nums.len();
        let last_num = *nums.last().unwrap();

        while start < end {
            let middle = start + (end - start) / 2;

            if last_num < target && nums[middle] <= last_num {
                end = middle;
                continue;
            }

            if target <= last_num && last_num < nums[middle] {
                start = middle + 1;
                continue;
            }

            if target <= nums[middle] {
                end = middle;
            } else {
                start = middle + 1;
            }
        }

        if end < nums.len() && nums[end] == target {
            return end as i32;
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step3_test() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
        assert_eq!(Solution::search(vec![1], 0), -1);
        assert_eq!(Solution::search(vec![], 0), -1);

        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 2), 6);
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 4), 0);
        assert_eq!(Solution::search(vec![5, 1, 3], 5), 0);
        assert_eq!(Solution::search(vec![5, 1, 3], 1), 1);
        assert_eq!(Solution::search(vec![5, 1, 3], 3), 2);
    }
}
