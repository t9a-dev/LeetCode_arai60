// Step1
// 目的: 解法の理解

// 方法
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  問題の理解
  - 整数を持つ配列numsが与えられる。numsの並び順を変えずに厳密に増加する部分列を作ったときに最長となる部分列の長さを返す。
  case1: nums=[1,0,2,3] subsequence=[1,2,3] return=3
  case2: nums=[0,1,0,3,2,3] subsequence=[0,1,2,3] return=4
  case3: nums=[3,3,3,3] subsequence=[3] return=1

  解法の理解
  DPテーブルによる状態の遷移もあり理解の助けになりそう
  https://leetcode.com/problems/longest-increasing-subsequence/solutions/4511397/c-dynamic-programming-clean-code-by-dany-5mk0/
  このコードだけでは理解しきれず、ChatGPT(GPT-5.1)の学習モードを使いながら理解を進めた。
  - 動的計画法では「何かを少しづつ積み重ねる」、「前に計算したものを使い回す」という点が重要
  - 増加部分列はある値自身を含むので最低1になる。最初に配列の数と等しい数だけ増加部分列を1で初期化して用意する。
  - 先頭からnums[i]を見ていった時に、左側の値の中(nums[0..i])にnums[i]よりも小さい値があるかを確認する。
    - 増加部分列は[1,3,5]のように右に行くにつれて大きくなる数列である。部分数列では右側にある数値の方が左側の数値よりも大きくなる。
  - nums=[0,1,0,3,2,3]の時 nums[4](2)はnumsの4番目に値2が入っていることを表している。
    - nums[0](0)は左側に数値がないのでスキップ lengths[0] = 1
    - nums[1](1)は左側のnums[0](0)より値が大きいので、lengths[1]を更新する。 lengths[1] = max(lengths[0..1], lengths[1] + 1) = lengths[0](1) + 1 = 2
      - 自分より長い増加部分列をもつ場合に、自分自身の長さ(1)を加えて、増加部分列を更新するので+1している。
      - lengthsは増加部分列の長さを保持している。具体的な数列で表すと lengths[1]の増加部分列は [0, 1] になる。
    - nums[2](0)は左側に自身よりも小さい値が無いのでスキップ lengths[2] = 1
    - nums[3](3)は左側[0..3)により小さい値が存在する。 length[3] = max(lengths[3], lengths[0..3] + 1) = lengths[1](2) + 1 = 3
    - nums[4](2)は左側[0..4)により小さい値が存在する。 length[4] = max(lengths[4], lengths[0..4] + 1) = lengths[1](2) + 1 = 3
      - length[0](1), lenght[1](2), lengths[2](1)部分配列の長さが一番大きいのはlengths[1](2)となる。lengths[3](3) <-値がnums[4](2)より小さくないので増加部分列に加えられずスキップ
    - nums[5](3)は左側[0..5)により小さい値が存在する。 lengths[5] = max(lengths[5], lengths[0..5] + 1) = lengths[4](3) + 1 = 4
      - length[0](1), lenght[1](2), lengths[2](1), lengths[4](3)部分配列の長さが一番大きいのはlengths[4](3)となる。lengths[3](3) <-値がnums[5](3)より小さくないので増加部分列に加えられずスキップ
  - ある値nums[i]の時に、この値よりも左側の値nums[0..i]をループで全て見ながら比較する。
  - lengthsの中で一番大きい増加部分列の長さを返す lengths[5]の4が答えとなる。

  正解してから気づいたこと
  - 時間計算量O(n ^ 2)となるようなナイーブな実装については、何をしているのか、なぜこうするのかを理解できた。
    - 時間計算量O(n log n)となる実装もあるようなので、この実装も理解したい。(step1b.rsで実装しておく)
  - if nums[left_index] < nums[i]は条件を反転してcontinueすることでネストを浅くできるが、条件通りの流れで素直に読めなくなるデメリットの方が大きいのでやらない。

  所感
  - 動的計画法(DP)カテゴリの問題は解法自体の理解をするのに要する時間と労力が大きいのできつい。
    - どういう気持でこうしているのか、なぜこうなるのかの部分を理解するのが難しく感じる。
    - きつく感じるのはコンフォートゾーンの外にいるということなので良いことではあるかなどと思った。
*/

pub struct Solution {}
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        };

        let n = nums.len();
        let mut lengths = vec![1; n];

        for i in 0..n {
            for left_index in 0..i {
                if nums[left_index] < nums[i] {
                    lengths[i] = lengths[i].max(lengths[left_index] + 1);
                }
            }
        }

        *lengths.iter().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step1a_test() {
        assert_eq!(Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
        assert_eq!(Solution::length_of_lis(vec![0, 1, 0, 3, 2, 3]), 4);
        assert_eq!(Solution::length_of_lis(vec![7, 7, 7, 7, 7, 7, 7]), 1);
        assert_eq!(
            Solution::length_of_lis(vec![5, 7, -24, 12, 13, 2, 3, 12, 5, 6, 35]),
            6
        );
        assert_eq!(Solution::length_of_lis(vec![7]), 1);
        assert_eq!(Solution::length_of_lis(vec![2, 1]), 1);
        assert_eq!(Solution::length_of_lis(vec![]), 0);
    }
}
