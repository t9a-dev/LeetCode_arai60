// Step1a_naive
// 目的: 手作業でやることをコードにする練習。総当りの実装をしてみる

/*
  問題の理解
  - 整数を持つ配列numsが与えられるので、numsの部分配列のうち合計値の最大値を返す。
  部分配列は配列numsの連続したからでない配列。
  nums=[1,2,3] のとき [1,3]は連続していないので部分配列ではない

  何を考えて解いていたか
  - 配列の数値自体もそれ1つとして部分配列になる。
  とにかく全通りのペアの合計値を作って、配列に全部入れて最後に一番大きい値を取り出す。
  ここで Time Limit Exceeded するコードが出来上がった。
  この実装方針のまま改善する方法が思いつかず、ChatGPT(GPT-5.1)に聞いたところ、殆どカダネのアルゴリズムの実装がでてきた。
  分割統治の実装を別で写経する。(step1b_divide.rs)

  所感
  - 元々ナイーブな実装自体を思いつかなかったので実装に苦労した。組み合わせ問題は難しいなと思った。
*/

pub struct Solution {}
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        /*
          このコードはLeetCode採点システム上で Time Limit Exceeded となります。
        */
        if nums.is_empty() {
            panic!("nums must not be empty.");
        }
        if nums.len() == 1 {
            return nums[0];
        }

        let mut sub_array_sum: Vec<i32> = Vec::new();

        for i in 0..nums.len() {
            let mut sum = nums[i];
            sub_array_sum.push(sum);

            for j in i + 1..nums.len() {
                sum += nums[j];
                sub_array_sum.push(sum);
            }
        }

        *sub_array_sum.iter().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step1a_naive_test() {
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
    fn step1a_naive_panic_test() {
        Solution::max_sub_array(vec![]);
    }
}
