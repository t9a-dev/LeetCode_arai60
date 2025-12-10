// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  問題の理解
  - ソート済の重複のない整数からなる配列numsと整数targetが与えられる。
  numsにtargetが含まれる場合、numsのindexを返す。
  numsにtargetが含まれない場合、numsがソートされた状態でtargetをnumsのどの位置に挿入するべきかのindexを返す。
    - nums[1,3] target=2 output=1 nums[1]にtarget=2を挿入すればソートされた状態が維持できる
  時間計算量がO(log n)になるようなアルゴリズムで実装する必要がある。つまり、線形探索は使えない。

  何がわからなかったか
  - binary searchをしながら、元の配列に対応するインデックスをうまく取り回す方法

  何を考えて解いていたか
  - binary searchの時間計算量はO(log n)になるので、binary searchを実装する。
    - ちなみにRustでは似たような処理を行うiter::insert_positionといった感じで存在すると思うので後で実装を見てみる。
  - binary searchは実装したことが無いので何をしているのか整理する。
    - numsの真ん中のインデックス(middle)を取得する。
    - target <= nums[middle] のように比較してtargetがどちらの集合にありそうか判断して、該当しない方は捨てる。
    - binary searchするために2つに分ける。nums[0..middle],nums[middle..nums.len()]
    - nums.len() == 2 になるまで行う。

  - 再帰で解けそうな感じがする。
    - 基本ケース
      - nums.len() == 1 であれば target == nums[0] target <= nums[0]
      でインデックスまたは挿入位置を特定する。
    - 再帰ケース
      - target <= nums[middle] で targetが含まれていると思われるnumsのスライスを引数に再帰に入る。
  ここで手が止まったので答えを見る

  解法の写経と理解
  https://leetcode.com/problems/search-insert-position/solutions/7169834/search-insert-position-binary-search-for-0hjb/
  - start,endで見ている区間の開始位置、終了位置を管理している。
  - start,endの区間で見るものがなくなるまで繰り返しを行っている。
  - 区間の真ん中の値(middle_value)とtargetの値を比較して等しければindexを返している。
  - target と middle_valueを比較して、targetがmiddleで分けたときの区間の左側、右側どちらかにあるかを確認している。
    - 区間の開始、終了を表す変数start,endをmiddleを基準にずらすことで、log n の範囲に区間を絞り込んでいる。
  - 最後にstartをそのまま返しているのは直感的でないと感じた。
    - target と　middle_value の比較
      - targetの方が小さい場合はmiddle - 1 をendに更新している
      - targetの方が大きい場合はmiddle + 1 をstartに更新している
    - targetがnums[i]のいずれよりも小さい時、startは動かないので0となり、targetはnums[0]の位置に挿入される。
    - targetがnums[i]のいずれかより大きい時、startは常にnums[i] < target の位置を指し示す。
    - nums[middle]　== targetであれば早期リターンする。見つからない時はstartが指し示すインデックスを返せば良いというロジックになっている。

  正解してから気づいたこと
  - 解法のコードは自然に理解できた。解法自体は近いところまで考えれていた気がするが、コードで表現する所までは距離があった。
  - このコードは計算量や記述のシンプルさから無駄がないように見える。最適解だと思った。
  - 練習として再帰に書き換えてみる。step1a_recursive.rs

  n = nums.le()
  時間計算量: O(log n)
  空間計算量: O(1)

*/

pub struct Solution {}
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut start = 0;
        let mut end = nums.len() as i32 - 1;

        while start <= end {
            let middle = (start + end) / 2;
            let middle_value = nums[middle as usize];

            if middle_value == target {
                return middle;
            }
            if target < middle_value {
                end = middle - 1;
                continue;
            }

            start = middle + 1;
        }

        start
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn playground() {
        let nums = vec![1, 2];
        let empty: Vec<i32> = vec![];

        assert_eq!(empty, &nums[0..0]);
        assert_eq!(vec![1], &nums[0..1]);
        assert_eq!(vec![2], &nums[1..nums.len()]);
    }

    #[test]
    fn step1_test() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);

        assert_eq!(Solution::search_insert(vec![], 8), 0);
    }
}
