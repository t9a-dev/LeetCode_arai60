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
  時間計算量: O(n log n) for文の中のpartition_pointでO(log n) これをfor文でn回繰り返している
  空間計算量: O(n) numsそれ自体が増加部分列の場合にnums.len()とサイズが等しくなる
*/

/*
  1回目: 1分57秒
  2回目: 2分07秒
  3回目: 2分00秒
*/

pub struct Solution {}
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        // lis(largest_increasing_subsequence)
        let mut lis = Vec::new();

        for num in nums {
            let insert_position = lis.partition_point(|num_in_lis| *num_in_lis < num);

            if lis.len() <= insert_position {
                lis.push(num);
                continue;
            }

            lis[insert_position] = num;
        }

        lis.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn play_ground() {
        let insert_position = vec![0].partition_point(|v| *v < 0);
        assert_eq!(insert_position, 0);

        let insert_position = vec![0].partition_point(|v| *v < 1);
        assert_eq!(insert_position, 1);

        let insert_position = vec![0].partition_point(|v| *v < -1);
        assert_eq!(insert_position, 0);
    }

    #[test]
    fn step3_test() {
        assert_eq!(Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
        assert_eq!(Solution::length_of_lis(vec![0, 1, 0, 3, 2, 3]), 4);
        assert_eq!(Solution::length_of_lis(vec![7, 7, 7, 7, 7, 7, 7]), 1);
        assert_eq!(
            Solution::length_of_lis(vec![5, 7, -24, 12, 13, 2, 3, 12, 5, 6, 35]),
            6
        );
        assert_eq!(Solution::length_of_lis(vec![1, 98, 99, 100, 2, 4]), 4);
        assert_eq!(Solution::length_of_lis(vec![7]), 1);
        assert_eq!(Solution::length_of_lis(vec![2, 1]), 1);
        assert_eq!(Solution::length_of_lis(vec![]), 0);
    }
}
