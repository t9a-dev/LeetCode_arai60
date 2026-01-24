// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  問題の理解
  - 自然数からなる配列numsと自然数targetが与えられる。合計値がtarget以上となる部分配列の最小サイズを求めて返す。
  合計値がtarget以上になるような部分配列が存在しない場合は0を返す。

  何を考えて解いていたか
  - 累積和の規則とSliding windowの関係を考える。
  target=4
  [1,1,2,1]
  [3,1,1]
  [1,1,3]
  - 部分配列の長さは2ポインタの差から求められる。
  - 部分配列の長さを保持する変数を用意しておき、累積和がtarget以上となった時に部分配列の最小サイズを更新できるか試行する。
  - 部分配列の最小長さを更新したら、
    - 累積和を0にリセット
    - start = endにしてこれまで見た値を捨てて次のループへ
  この解法で解けるなら時間計算量O(n),空間計算量O(1)になると考える。
  フォローアップで時間計算量がO(n log n)となるようなアルゴリズムも考えようとあるが、ソートしてtargetからピボットを探して配列の値を半分捨てるとかだろうか。
  部分配列は配列内の連続した配列であるので、元の配列を並べ替えるのはダメそう。
  よく分からないのでまずは、思いついた解法を実装する。
  この解法ではWrong Answerとなったので解答を見る

  何がわからなかったか
  - target=11, nums=[1,2,3,4,5]のように列の後半にかけて解となる部分配列が出現するケースの対応方法。
  もう少し具体的にはstartポインタをどのような場合にどこまで動かすか。

  解法の理解
  https://leetcode.com/problems/minimum-size-subarray-sum/solutions/7266487/rust-efficient-two-pointer-approach-by-h-dg4q/
  - start,endによる閉区間の2ポインタで区間の端点を管理している。
  - sumで区間（部分配列）の合計値を管理している。
  - ポインタが不変条件(start <= end && end < nums.len())を満たす間以下を繰り返す。
    - 区間の合計値がtarget以上となったら、最小部分配列長の更新を行う。
      - この時点でstart側を縮小してさらに小さい部分配列が作れないかを試行している。
    - 区間の合計値がtarget未満のとき、条件を満たす部分配列を作るためにend側を伸ばして合計値を更新。

  正解してから気づいたこと
  - 区間の始点、終点どちらかを固定するのではなく、不変条件(start <= end && end < nums.len())を満たす間、両側の端点を動かしている。

  所感
  - 問題の制約上、入力は自然数であることが分かっているのでメソッドのシグネチャはi32ではなく、u32で統一してほしいと思った。
*/

pub struct Solution {}
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        };

        let n = nums.len() + 1;
        let mut min_subarray_length = n;
        let mut sum = nums[0];
        let (mut start, mut end) = (0, 0);

        while start <= end && end < nums.len() {
            if target <= sum {
                min_subarray_length = min_subarray_length.min((end - start) + 1);
                sum -= nums[start];
                start += 1;
            } else {
                end += 1;
                if end < nums.len() {
                    sum += nums[end];
                }
            }
        }

        if min_subarray_length == n {
            return 0;
        }

        min_subarray_length as i32
    }

    #[allow(dead_code)]
    pub fn min_sub_array_len_wa(target: i32, nums: Vec<i32>) -> i32 {
        /*
          この実装はWrong Answerとなります。
        */
        let mut start = -1isize;
        let mut accumulated_sum = 0;
        let mut min_subarray_length = (nums.len() + 1) as isize;

        for end in 0..nums.len() {
            if target <= accumulated_sum + nums[end] {
                min_subarray_length = min_subarray_length.min(end as isize - start);
                accumulated_sum = 0;
                start = end as isize;
            }

            accumulated_sum += nums[end];
        }

        if min_subarray_length == (nums.len() + 1) as isize {
            return 0;
        }

        min_subarray_length as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step1_test() {
        assert_eq!(Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
        assert_eq!(Solution::min_sub_array_len(4, vec![1, 4, 4]), 1);
        assert_eq!(
            Solution::min_sub_array_len(11, vec![1, 1, 1, 1, 1, 1, 1, 1]),
            0
        );
        assert_eq!(Solution::min_sub_array_len(11, vec![1, 2, 3, 4, 5]), 3);
    }
}
