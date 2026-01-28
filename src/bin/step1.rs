// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  問題の理解
  - 整数からなる配列numsと整数targetが与えられる。numsにtargetが含まれる場合はその位置を返す。見つからなければ-1を返す。
  numsはソートされた配列をk回転させたものが与えられる。[0,1,2,4,5,6,7] を3回転すると[4,5,6,7,0,1,2]となる。kの位置で配列を分割して左右を入れ替えるイメージ。
  numsに含まれる値は一意である。
  制約として時間計算量O(log n)を満たす必要がある。

  何を考えて解いていたか
  - lower_boundを探して、探索終了後にtargetと等しいか一度確認して答えを返す方針で解く。
  この実装が難しそうなら、探索中にtarget == nums[i] を見つけたら早期リターンする。
  [start,end)半開区間で扱う。
  この考え方だと、middleの左右にmiddleよりも大きい値が存在するケースを処理できない。不変条件が壊れる。
  [start,end]な閉区間にしてtargetがありそうな範囲に絞り込んでいって、探索終了時にnums[end] == targetとなるかを確認するほうが良さそう
  ここまで考えて手が止まってので答えを見る。

  何がわからなかったか
  - 回転後の配列でmiddleの左右どちら側にtargetがあるのかどう判断すればよいか分からなかった。

  解答の理解
  https://leetcode.com/problems/search-in-rotated-sorted-array/solutions/3879263/100-binary-search-easy-video-ologn-optim-yp2k/
  - start ~ middle , middle ~ end どちらがソートされているかを判定する。
    - 元々ソートされていた配列をk回転させた後の配列は中央位置から見て左右どちらかの区間は必ずソートされている性質を持つ。
  - ソートされている方の範囲で値を探す。
  - targetがソートされている範囲に含まれているかを判定する。
    - ソートされている範囲にtargetが含まれていれば、探索範囲をソートされている範囲に縮小する。
    - ソートされている範囲にtargetが含まれていなければ、ソートされている範囲を探索範囲外にする。

  正解してから気づいたこと
  - この解法で解くにしても、if-elseのネストはもう少しなんとかしたい。
    - continueにしても良いが、場合分けなのでelseの方がより意図に対応している感じがする。
  - 早期リターンしない解法はstep2で他の人のコードを見つつ練習する。
*/

pub struct Solution {}
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return -1;
        }

        let mut start = 0;
        let mut end = nums.len() - 1;

        while start <= end {
            let middle = start + (end - start) / 2;

            if nums[middle] == target {
                return middle as i32;
            }

            let is_start_side_sorted = nums[start] <= nums[middle];
            let is_target_in_start_side = nums[start] <= target && target <= nums[middle];
            let is_target_in_end_side = nums[middle] < target && target <= nums[end];

            if is_start_side_sorted {
                if is_target_in_start_side {
                    end = middle - 1;
                } else {
                    start = middle + 1;
                }
            } else {
                if is_target_in_end_side {
                    start = middle + 1;
                } else {
                    end = middle - 1;
                }
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step1_test() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
        assert_eq!(Solution::search(vec![1], 0), -1);
        assert_eq!(Solution::search(vec![], 0), -1);

        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 2), 6);
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 4), 0);
        assert_eq!(Solution::search(vec![5, 1, 3], 1), 1);
        assert_eq!(Solution::search(vec![5, 1, 3], 5), 0);
        assert_eq!(Solution::search(vec![5, 1, 3], 3), 2);
    }
}
