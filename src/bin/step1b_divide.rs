// Step1b_divide
// 目的: 分割統治法による解法の写経。

/*
  問題の理解
  - 整数を持つ配列numsが与えられるので、numsの部分配列のうち合計値の最大値を返す。
  部分配列は配列numsの連続したからでない配列。
  nums=[1,2,3] のとき [1,3]は連続していないので部分配列ではない

  何がわからなかったか
  - 分割統治法は使ったことがないので、何をどういう気持ち（動機）でやっているのか分からなかった。

  - (left..=middle).rev()としている箇所。なぜ逆順にしているのかがよく分からなかった。
  ある地点から端(左、右)に向かって計算したいのでこうしている。
  left=0 middle=4 のとき　4番目の位置から0番目の端に向かって計算したい。[-2, 1, -3, 4, -1] は [-1, 4, -3, 1, -2] で見たい。

  - middleは半分ずつになっていくので、left=0 ~ middle=3のときの累積和が求められていないのでは？
  勘違いしていた。left=0 ~ middle=2, right=middle+1=3 ~ right=4 のように計算される。

  所感
  - 正直これがO(n log n)になるのは腹落ちしていないが時間切れなので次に進む
  - とりあえず答えを消して書くことはできた。暗記している感じ。
    - 再帰に入るときは区間全部を指定する。
    - 再帰の中では区間の真ん中を見つけて左側、右側に分けて累積和のを計算しながらそれぞれ一番大きい累積和を探している。
    - 区間が重複しないように気をつける必要がある。
      - 最初の再帰呼び出し時にどのように区間を渡しているか見れば、とりあえず間違えずに書けた。left..=middle　と middle + 1..=rightのところ。
  - 分割統治法のコードを書いてみて、以前解いた問題のレビューコメントにあった「左端と右端のインデックスだけ持たせる...」の意味が少し理解できた気がした。
  calculate_max_sub_arrayメソッド引数のleft,rightがこれに相当するという理解。
  分割した配列のコピーまたは配列の参照(スライス)を渡しても同じことをできるが、左端(left)と右端(right)のインデックスを渡すだけのほうが、データのサイズがコンパクトになるので効率が良いという理解。
  https://github.com/t9a-dev/LeetCode_arai60/pull/24#discussion_r2502450568
*/

pub struct Solution {}
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            panic!("nums must not be empty.");
        }

        Self::calculate_max_sub_array(&nums, 0, nums.len() - 1)
    }

    fn calculate_max_sub_array(nums: &[i32], left: usize, right: usize) -> i32 {
        if left == right {
            return nums[left];
        }

        let middle = (left + right) / 2;
        let left_max = Self::calculate_max_sub_array(nums, left, middle);
        let right_max = Self::calculate_max_sub_array(nums, middle + 1, right);

        let mut sum = 0;
        let mut max_in_left = i32::MIN;
        for i in (left..=middle).rev() {
            sum += nums[i];
            if max_in_left < sum {
                max_in_left = sum;
            }
        }

        sum = 0;
        let mut max_in_right = i32::MIN;
        for i in middle + 1..=right {
            sum += nums[i];
            if max_in_right < sum {
                max_in_right = sum;
            }
        }

        let merged_max = max_in_left + max_in_right;

        left_max.max(right_max).max(merged_max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step1b_divide_test() {
        assert_eq!(Solution::max_sub_array(vec![-1, -2]), -1);
        assert_eq!(Solution::max_sub_array(vec![1, -1, 1]), 1);
        assert_eq!(
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
        assert_eq!(Solution::max_sub_array(vec![5, 4, -1, 7, 8]), 23);
        assert_eq!(Solution::max_sub_array(vec![1]), 1);
        assert_eq!(Solution::max_sub_array(vec![0]), 0);
    }

    #[test]
    #[should_panic(expected = "nums must not be empty.")]
    fn step1b_divide_panic_test() {
        Solution::max_sub_array(vec![]);
    }
}
