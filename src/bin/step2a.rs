// Step2a
// 目的: 別の実装方法を練習

/*
  累積和と二分探索を組み合わせた実装を練習する
  - 参考
    https://github.com/Yoshiki-Iwasa/Arai60/pull/43/changes#diff-705d13bf86ae090a0d8b47599b0a08e57431901d11cce1cdd7c2dea2f2568047R43
    - 二分探索も利用している
      https://github.com/naoto-iwase/leetcode/pull/50/changes#diff-d7328f247ea703c897516111c872b729bf96f2b2e54ab850566dd2ac46122d94R62
      https://github.com/hayashi-ay/leetcode/pull/51/changes#diff-6d4eb2707ed57d8037bfa2e5985424b237a407741a7602ba92b31e090d1cb096R79

  解法の理解
  - 入力numsに対応する累積和の配列を生成する。累積和は先頭が0からスタートすることに注意。
  - target以上となる部分配列のうち、最小の部分配列を探してそのサイズを返したい。つまり、部分配列の和（累積和）がtarget以上になるものを探す。
  - ある区間の合計値は [start,end)な半開区間のとき、Sum(start,end) = prefix_sums[end] - prefix_sums[start] で求められる。
           nums = [1,2,3,4,5]
    prefix_sums = [0,1,3,6,10,15]
    Sum(1,5) = prefix_sums[5] - prefix_sums[1] = 15 - 1 = 14
    区間（部分配列）の合計値がtarget以上になる最小の区間長さを探している。
    target <= prefix_sums[end] - prefix_sums[start] = target + prefix_sums[start] <= prefix_sums[end]
    partition_pointメソッドでは、predicateがfalseとなる最初のインデックスを返す。
    prefix_sums[end] < target + prefix_sums[start] とするとtarget以上となる最初のインデックスが返ってくる。(targetと等しい値もfalseにしている。)

  所感
  - lower_boundの条件で prefix_sums[end] < target + prefix_sums[start] としている箇所が難しく感じた。なぜこれが正しくなるのかすぐに理解できず、紙に書いてデバッグして正しいことが分かった感じ。
  ある区間の合計値は累積和を利用して求められるということが分かっている前提になっているからだと思った。(Sum(start,end) = prefix_sums[end] - prefix_sums[start])
*/

pub struct Solution {}
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        const NOT_FOUND: i32 = 0;

        if nums.is_empty() {
            return NOT_FOUND;
        }

        let mut nums_iter = nums.iter();
        let prefix_sums = std::iter::successors(Some(0), |x| nums_iter.next().map(|num| num + x))
            .collect::<Vec<_>>();
        let sentinel_subarray_length = nums.len() + 1;
        let mut min_subarray_length = sentinel_subarray_length;

        for start in 0..nums.len() {
            let end =
                prefix_sums.partition_point(|prefix_sum| *prefix_sum < target + prefix_sums[start]);

            if end == prefix_sums.len() {
                break;
            }

            min_subarray_length = min_subarray_length.min(end - start);
        }

        if min_subarray_length == sentinel_subarray_length {
            return NOT_FOUND;
        }

        min_subarray_length as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step2a_test() {
        assert_eq!(Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
        assert_eq!(Solution::min_sub_array_len(4, vec![1, 4, 4]), 1);
        assert_eq!(
            Solution::min_sub_array_len(11, vec![1, 1, 1, 1, 1, 1, 1, 1]),
            0
        );
        assert_eq!(Solution::min_sub_array_len(11, vec![1, 2, 3, 4, 5]), 3);
    }
}
