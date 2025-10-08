// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  問題の概要: 2つの整数を持つ配列が与えられる。2つのどちらにも含まれる整数列を戻り値として返す。
  例:
  input: [1,2,2,1],[2,2], output: [2]
  input: [4,9,5],[9,4,9,8,4], output: [4,9]又は[9,4]

  何がわからなかったか
  - 特になし

  何を考えて解いていたか
  -　愚直に配列を全て見ていって自身と同じ値がもう片方の配列にあれば結果の配列に追加して最後に返す。
  配列を全て見るので時間計算量: O(N^2),空間計算量: O(N)
  - HashSetに片方の配列の値を追加する。もう片方の配列を全走査しながらHashSetに値が含まれているか探索して、含まれていれば
  結果の配列に追加していく。
  - HashSetのメソッドを確認していたらそのままのintersectionメソッドを見つけてしまい、
  アルゴリズムについて考える必要がなくなってしまった。とりあえずHashSet::intersection()メソッドを利用しない形で書く。

  正解してから気づいたこと
  - Vec<i32>型であるnums2.contains()は線形探索になるのでO(N)になるが、HashSetであれば探索に平均O(1)となるのでHashSetにしたほうが良さそう。
  入力の型がi32で固定長なのでHashSetにするときに伴うハッシュ化はO(N)になるがループの外で一度だけ行うので問題ない。
  - ループで値を取り出す配列はサイズが小さい方にしたほうが効率が良い。内側のHashSetの探索時間はO(1)とすると計算量Nが支配的になる。
  Nは配列のサイズなので全走査する配列はより小さい方が時間計算量が少なくなる。
  - 変数名intersectionがメソッド名と被っていて違和感を感じた。

*/

use std::collections::HashSet;

pub struct Solution {}
impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut intersection = HashSet::with_capacity(nums1.len().min(nums2.len()));
        for num1 in nums1 {
            if nums2.contains(&num1) {
                intersection.insert(num1);
            }
        }

        intersection.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step1_test() {
        let mut result = Solution::intersection(vec![1, 2, 2, 1], vec![2, 2]);
        result.sort();
        assert_eq!(result, vec![2]);

        let mut result = Solution::intersection(vec![4, 9, 5], vec![9, 4, 9, 8, 4]);
        result.sort();
        assert_eq!(result, vec![4, 9]);
    }
}
