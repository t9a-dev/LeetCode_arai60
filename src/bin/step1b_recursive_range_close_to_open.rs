// step1b_recursive_range_close_to_open
// 目的: 左閉区間右開区間(left-close,right-open)として区間を扱う実装を練習する。

/*
  問題の理解
  - ソート済の重複のない整数からなる配列numsと整数targetが与えられる。
  numsにtargetが含まれる場合、numsのindexを返す。
  numsにtargetが含まれない場合、numsがソートされた状態でtargetをnumsのどの位置に挿入するべきかのindexを返す。
    - nums[1,3] target=2 output=1 nums[1]にtarget=2を挿入すればソートされた状態が維持できる
  時間計算量がO(log n)になるようなアルゴリズムで実装する必要がある。つまり、線形探索は使えない。

  何がわからなかったか

  何を考えて解いていたか
  - 区間を半開区間(left-close,right-open)として扱う時の再帰処理の設計
  呼び出し時は[0..nums.len())となる。right-openなので区間に自身(nums.len())を含まないため。
    - 基本ケース
      - start >= end return start 比較記号を数直線上の並びにするよりも、変数の並びを数直線上にしたほうが個人的に分かりやすい
        - start ~ end間に値があるかどうかで考える。left-close,right-openなので、start == end になると区間に値がなくなる。
        [0..0)の間に値はない。
      - target = nums[middle] return middle
    - 再帰ケース
     target < nums[middle] then [start..middle) else [middle+1..end) <-ここで target == nums[middle]ではないことが確定しているのでmiddle+1としてmiddle自体を除いている。

  正解してから気づいたこと
  - start,endをleft,rightに書き換えて見たがしっくりこなかったのでstart,endに戻した。start,endの方が単語の違いが視覚的に分かりやすい感じ。
  - 区間を扱う時特に理由が無ければ半開区間（right-close,left-open）が良さそう。
    - RustのRange記法[a..b]も半開区間(a..b]
      - https://doc.rust-lang.org/std/ops/struct.Range.html
    - Pythonのrange型も半開区間(a..b]
      - https://docs.python.org/ja/3/library/stdtypes.html#typesseq-range

*/

pub struct Solution {}
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        Self::search_insert_position(&nums, 0, nums.len() as i32, target)
    }

    fn search_insert_position(nums: &[i32], start: i32, end: i32, target: i32) -> i32 {
        if start >= end {
            return start;
        }

        let middle = (start + end) / 2;
        let middle_value = nums[middle as usize];
        if middle_value == target {
            return middle;
        }

        if target < middle_value {
            return Self::search_insert_position(nums, start, middle, target);
        }

        return Self::search_insert_position(nums, middle + 1, end, target);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step1b_test() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
        assert_eq!(Solution::search_insert(vec![1, 3], 2), 1);

        assert_eq!(Solution::search_insert(vec![], 8), 0);
    }
}
