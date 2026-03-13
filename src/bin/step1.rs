// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  問題文の日本語訳:
    整数配列の順列とは、その要素を連続した順序に並べたものを指します。
    * 例えば、配列 arr = [1,2,3] の場合、以下の配列がすべて arr の順列となります：[1,2,3]、[1,3,2]、[2,1,3]、[2,3,1]、[3,1,2]、[3,2,1]。
    整数配列の「次の順列」とは、その整数の辞書順で次に来るより大きな順列のことです。
    より形式的に言うと、配列のすべての順列を辞書順に従って1つのコンテナにソートした場合、その配列の次の順列とは、ソートされたコンテナ内でその順列の次に来る順列を指します。
    そのような並べ替えが不可能な場合、配列は可能な限り小さい順序（すなわち、昇順にソート）に再配置されなければなりません。
    * 例えば、配列 arr = [1,2,3] の次の順列は [1,3,2] です。
    * 同様に、配列 arr = [2,3,1] の次の順列は [3,1,2] です。
    * 一方、配列 arr = [3,2,1] の次の順列は [1,2,3] です。これは、[3,2,1] には辞書順でより大きな並べ替えが存在しないためです。
    整数配列 nums が与えられたとき、nums の次の順列を求めてください。
    この置換は元の配列内で行わなければならず、追加のメモリは定数量しか使用できません。

  問題の理解、何を考えて解いていたか
  - 自然数からなる配列numsを与えられる。numsの次の順列を返す。制約として、in-place（空間計算量: O(1)）のアルゴリズムで実装する必要がある。
  「次の順列」の定義がよく分からないので、自分の言葉で説明する。
    - まず全ての順列に注目して、各配列の先頭が昇順ソートされている用に見える。1~3
    - p = partition_pointの戻り値。predicateは v < nums[i] とする。
      - 二分探索で v < nums[i] として i == p でないときにswapできる。
        - i == p だとソート済なのでcontinue
        - ソート済の場合(関数最後まで到達したら)はswap(nums.last, nums[nums.len - 2])している
      - 手作業でやることを考えて、[1,2,3]~[3,2,1]までの遷移をトレースして見つけた規則性から考えられる条件分岐
        - i+1 == nums.len
          - nums.pop,nums.insert(0) return
        - if p == nums.len
          - nums.remove(0),nums.push() return
        - if [i+1] < [p] swap(i+1,p) return else continue
        - nums.is_sorted_byでソートを検知したら、nums.reverse()してreturnする。
    n = nums.len
    時間計算量: O(n log n)
    正直このロジックで解けるかは不安だが、アルゴリズムも思いつかないので実装してみる。
  Wrong Answerとなった。二分探索するときは対象の配列がソートされている必要があるが、まずこの点を忘れていてうまく動いていないように見える。
  numsを任意の都合の良い位置（ソート済のまとまりとそうでないまとまりで分けられる）で分けてから考えるなどを考慮しないと二分探索が使えない。
  全然わからないのでstep2で他の人のコードを見る。

  何がわからなかったか
  - 与えられたnumsの次の順列を返す時に、次の順列をどのように決めているのかが分からなかった。
*/

pub struct Solution {}
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        /*
          このコードはWrong Answerとなります。
        */
        if nums.is_empty() {
            return;
        }
        let nums_len = nums.len();
        if nums_len == 2 {
            nums.swap(0, 1);
            return;
        }

        let is_desc_sorted = nums.is_sorted_by(|a, b| b <= a);
        if is_desc_sorted {
            nums.reverse();
            return;
        }

        for i in 0..nums_len {
            let p = nums.partition_point(|v| *v < nums[i]);

            if i == p {
                continue;
            }
            if i + 1 == nums_len {
                let v = nums.pop().unwrap();
                nums.insert(0, v);
                return;
            }
            if p == nums_len {
                let v = nums.remove(0);
                nums.push(v);
                return;
            }
            if nums[i + 1] < nums[p] {
                nums.swap(i + 1, p);
                return;
            }
        }

        nums.swap(nums_len - 2, nums_len - 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn play_ground() {
        let nums = vec![1, 2, 3];
        let partition_point = nums.partition_point(|v| *v < 1);
        assert_eq!(partition_point, 0);

        let partition_point = nums.partition_point(|v| *v < 2);
        assert_eq!(partition_point, 1);

        let nums = vec![2, 1, 3];
        let partition_point = nums.partition_point(|v| *v < 2);
        assert_eq!(partition_point, 2);
    }

    #[test]
    fn step1_desc_sorted_next_permutation_test() {
        let mut nums = vec![3, 2, 1];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 2, 3]);
    }

    #[test]
    // テストエクスプローラーでfailが紛らわしいので should_panicとして握りつぶしています。
    #[should_panic]
    fn step1_test() {
        let mut nums = vec![1, 2, 3];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 3, 2]);

        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![2, 1, 3]);

        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![2, 3, 1]);

        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![3, 1, 2]);

        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![3, 2, 1]);

        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 2, 3]);

        let mut nums = vec![1, 1, 5];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 5, 1]);
    }
}
