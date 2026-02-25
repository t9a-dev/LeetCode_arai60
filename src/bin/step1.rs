// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  問題の理解
  - 整数からなる配列numsが与えられる。numsに含まれる0を配列の末尾に移動する。このとき、0以外の数値は並び順を変更しない。
  新たに配列を作らずに(numsをコピーせず)処理を行う必要がある。

  何を考えて解いていたか
  - 配列を走査しながら0を見つけたらremove()して、push()すればin-placeで解けそう。
  n = nums.len()
  時間計算量: O(n)
  空間計算量: O(1)

  テストケース nums=[0, 0, 1] out=[0, 1, 0]となり,wrong Answerとなった。
  ループ中に配列を変更するので、2つ目の0をループ中で飛ばしてしまうのが原因だと思った。
  for-loopではなく、while-loopにして0を見つけて移動させたときはインデックスをインクリメントしない方針で対応できそう。
  これまでに見た要素数を別でインクリメントしておいて、nums.len() - 1になったらループを抜けないと無限ループになる。

  何がわからなかったか
  - そのままループで解けると思い込んでしまい、エッジケースでWrong Answerとなった。

  正解してから気づいたこと
  - iter.remove(i)は時間計算量がO(n)なので、全体の時間計算量はO(n ^ 2)になる。
*/

pub struct Solution {}
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut i = 0;
        let mut processed_nums_count = 0;

        while processed_nums_count < nums.len() {
            processed_nums_count += 1;

            if nums[i] != 0 {
                i += 1;
                continue;
            }

            let zero = nums.remove(i);
            nums.push(zero);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step1_test() {
        let mut before_nums = vec![0, 1, 0, 3, 12];
        let after_nums = vec![1, 3, 12, 0, 0];
        Solution::move_zeroes(&mut before_nums);
        assert_eq!(before_nums, after_nums);

        let mut before_nums = vec![0];
        let after_nums = vec![0];
        Solution::move_zeroes(&mut before_nums);
        assert_eq!(before_nums, after_nums);

        let mut before_nums = vec![0, 0, 1];
        let after_nums = vec![1, 0, 0];
        Solution::move_zeroes(&mut before_nums);
        assert_eq!(before_nums, after_nums);
    }
}
