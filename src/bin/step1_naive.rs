// Step1_naive
// 目的: ナイーブな実装を自分でできるのか試す

/*
    時間計算量: O(N^2)
    空間計算量: O(1)
*/

/*
    Leet Codeの実行時間は当てにならないとしても1000ms程度の実行時間がかかるので遅いなと思った。
    空間計算量はO(1)と省メモリだが、実行時間が遅すぎるので安易にトレードオフとも言えないなと思った。
*/

pub struct Solution {}
impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;

        for (i, num_i) in nums.iter().enumerate() {
            let mut prefix_sum = 0;
            prefix_sum += num_i;

            if prefix_sum == k {
                count += 1;
            }

            for num_j in nums.iter().skip(i + 1) {
                prefix_sum += num_j;

                if prefix_sum == k {
                    count += 1;
                }
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step1_naive_test() {
        assert_eq!(Solution::subarray_sum(vec![1, 1, 1], 2), 2);
        assert_eq!(Solution::subarray_sum(vec![1, 2, 3], 3), 2);
        assert_eq!(Solution::subarray_sum(vec![1, -1, 0], 0), 3);
        assert_eq!(Solution::subarray_sum(vec![1], 0), 0);
        assert_eq!(Solution::subarray_sum(vec![1], 1), 1);
        assert_eq!(Solution::subarray_sum(vec![100, 1, 2, 3, 4], 6), 1);
    }
}
