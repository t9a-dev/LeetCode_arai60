// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  問題の理解
  - 整数を持つ配列numsが与えられるので、numsの部分配列のうち合計値の最大値を返す。
  部分配列は配列numsの連続したからでない配列。
  nums=[1,2,3] のとき [1,3]は連続していないので部分配列ではない

  何がわからなかったか
  - 手作業で解くならこう考えるという時点で自然言語でうまく表せず思考停止した。

  何を考えて解いていたか
  - 解法を思いついていない状態でコードを書き始めるとめちゃくちゃなコードが出来上がり混乱のもとになるので、手元で解法が固まるまでコードを書かないようにする。
  nums=[5,4,-1,7,8] max_sum_sub_arrayは長いのでとりあえずresultとする。
  手が止まったので解答を見る。

  解法の理解
  https://leetcode.com/problems/maximum-subarray/solutions/3315652/rust-dynamic-programming-kadanes-algorit-d9b5/
  この解法ではfold()メソッドを使っているが、最終的に値を取り出すときにタプルのn番目の要素という形で値を取り出しているのに違和感を感じたのでfold()メソッドを利用しない形に書き換えた。
  fold()は意味的に折りたたんで1つにまとめるというイメージがあるので、最終的に1つの値にまとめられないのであれば、無理に使うべきでは無いかなという感覚。
  nums=[-2, 1, -3, 4, -1, 2, 1, -5, 4]
  sum = 0 = 0.max(-2) max_sum = 0
  sum = 1 = 1.max(0 + 1) max_sum = 1
  sum = -2 = -3.max(1 + -3) max_sum = 1
  sum = 4 = 4.max(-2 + 4) max_sum = 4
  sum = 3 = -1.max(4 + -1) max_sum = 4
  sum = 5 = 2.max(3 + 2) max_sum = 5
  sum = 6 = 1.max(1 + 5) max_sum = 6
  sum = 1 = -5.max(-5 + 6) max_sum = 6
  sum = 5 = 4.max(4 + 1) max_sum = 6
  return 6

  自然言語で説明してみる
  - sumはnumsを先頭から見ているときのある時点の数列の合計値 sum(nums[0], nums[1], ...nums[i])
  - nums[i + 1] < sum + nums[i + 1] を満たさなければ、sumは合計が最大となる部分列を構成する数列としてふさわしくないので、数列(の合計値sum)を捨ててしまう。
  sum + nums[i + 1] < nums[i + 1] であれば、nums[i + 1]　の方が合計値が最大となる部分列を構成する数列としてさわしいので　sum = nums[i+1]となる。
  全ての時点での最大値(max_sum)を答えとして求めたいので、常に最大値を更新する。 max_sum = max_sum.max(sum);

  正解してから気づいたこと
  https://github.com/t9a-dev/LeetCode_arai60/pull/31/files/0951d7da3dfb99f4f7e076183f987f32ad468d47#r2533198397
  - 前回問題のレビューで教えてもらった動的計画法の適用条件に照らし合わせると、
    - 問題全体としては部分配列の合計が最大値(max_sum)になるような部分配列の合計値を求めている。
    - 部分問題としても、ある時点nums[0..i]の部分配列(sum)の合計値が最大となるような部分配列の合計値を求めている。
  といった感じで動的計画法の適用条件を満たしていると理解した。

  所感
  - 自分で解法を思いつけないし、解法を見ただけではその解法が正しいことを理解できず、処理の流れを追う必要がある。
    - ただ、解法の理解に要する時間は短くなっているので、このまま立ち止まらずに数をこなしていくのが良さそう
*/

pub struct Solution {}
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            panic!("nums must not be empty.");
        }

        let mut max_sum = i32::MIN;
        let mut sum = 0;

        for num in nums {
            sum = num.max(sum + num);
            max_sum = max_sum.max(sum);
        }

        max_sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step1_test() {
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
    fn step1_panic_test() {
        Solution::max_sub_array(vec![]);
    }
}
