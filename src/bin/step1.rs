// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  問題の理解
  - 整数からなる配列numsが与えられる。numsから作成することが可能なサブセットを生成して返す。
  nums=[1,2,3]
  subsets=[[],[1],[2],[3],[1,2],[1,3],[2,3],[1,2,3]]
  サブセットとは値に重複のない全ての可能な組み合わせだと理解した。

  何を考えて解いていたか
  - HashSetにおいて並び順が異なり同じ値を持つ配列が重複として扱われないので、HashSetに重複管理させるのは無理。
    - 毎回ソートするのも筋が悪そう。
  - ループを回しながら重複しない配列のを作っていく。
    - 空の配列とnums,nums[i]を解の配列に詰める。O(n)
    再帰処理
     - base_case
       nums.len() == 1でreturn
     - recursive_case
       num = nums.pop_front()
       numsを解の配列に詰める
       nums.push_back(num)

  時間計算量 O(n!)
  空間計算量 O(n!)
  問題の制約から nums.length <= 10 となるので階乗の計算量でも問題ないと判断。
  そもそも、要求される解が可能な組み合わせを列挙するものなので、時間計算量はあまり改善できなず定数因数のみの改善しか行えないと思った。
  途中でswapの方法でも書けると思ったが、まずは自然に思いついたVecDequeのpop_front(),push_back()を利用した解法で実装する。
  重複する配列が出力され、Wrong Answerとなった。
  重複する配列を含まずに実装する方法がすぐに思いつかないので、配列をsortしてHashSetにinsertすることで重複排除を行う。
  sortによって時間計算量が O(n! * n log n)になると考える。
  10! * 10 log 10 = 36,288,000 となる。秒あたり10億ステップだと見積もると 36,288,000 / 10 ^ 8 = 0.36288 = 約362msとなり、現実的な実行時間ではある。
  Acceptedになった(Runtime 288ms)。しかし、他の解法ではRuntime 0msのようなので明らかにソートせずに実装する解法があることが分かる。
  step2で解法を見る。

  何がわからなかったか
  - 重複する配列を結果に含めずに処理するアルゴリズム

  正解してから気づいたこと
  - 時間計算量の見積もりが不安だったのでGPT-5.2に聞いたところ正確にはsortの時間計算量よりも、make_subsets内のforループでcloneしているコストの方が大きくO(n! * n ^ 2)になるとのことだった。
  なので、実行時間の概算がかなり近い数値で見積もれたのはたまたまかと思った。

  所感
  - Wrong Answerになった時点で解答を見ようかと思ったが、時間計算量は悪化することが分かったうえでsortとHashSetによる重複排除による解法を実装してみることにしたのは良かったと思った。
  Acceptedになったから良かったということではなくて、最適解のアルゴリズムではないからナイーブな実装を試すことすらしないのは良くない癖だというコメントを思い出してこれを実践できたため。
  答えがわからないので腕を組んで時間を浪費するのは良くないが、ナイーブな実装がわかっているならまずは実装してみて、そこを起点にアルゴリズムを改善すれば良いという考え方。
*/

use std::collections::{HashSet, VecDeque};

pub struct Solution {}
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.is_empty() {
            return vec![vec![]];
        }

        let mut subsets = HashSet::new();

        subsets.insert(vec![]);
        for i in 0..nums.len() {
            subsets.insert(vec![nums[i]]);
        }

        Self::make_subsets(nums, &mut subsets);

        subsets.into_iter().fold(vec![], |mut result, subset| {
            result.push(subset);
            result
        })
    }

    fn make_subsets(nums: Vec<i32>, subsets: &mut HashSet<Vec<i32>>) {
        if nums.len() == 1 {
            return;
        }

        let mut subset = nums.clone();
        subset.sort();
        subsets.insert(subset);

        let mut nums = VecDeque::from_iter(nums.into_iter());
        for _ in 0..nums.len() {
            let num = nums.pop_front().unwrap();
            Self::make_subsets(nums.clone().into(), subsets);
            nums.push_back(num);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn playground() {
        let mut set = HashSet::new();
        set.insert(&[1, 2]);
        set.insert(&[2, 1]);
        assert_eq!(set.len(), 2);

        let mut set = HashSet::new();
        set.insert(&[2]);
        set.insert(&[2]);
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn step1_test() {
        let mut subsets = Solution::subsets(vec![1, 2, 3]);
        let mut expect = vec![
            vec![],
            vec![1],
            vec![2],
            vec![1, 2],
            vec![3],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3],
        ];
        subsets.iter_mut().for_each(|x| x.sort());
        subsets.sort();
        expect.iter_mut().for_each(|x| x.sort());
        expect.sort();
        assert_eq!(subsets, expect);

        let mut subsets = Solution::subsets(vec![3, 2, 4, 1]);
        let mut expect = vec![
            vec![],
            vec![3],
            vec![2],
            vec![2, 3],
            vec![4],
            vec![3, 4],
            vec![2, 4],
            vec![2, 3, 4],
            vec![1],
            vec![1, 3],
            vec![1, 2],
            vec![1, 2, 3],
            vec![1, 4],
            vec![1, 3, 4],
            vec![1, 2, 4],
            vec![1, 2, 3, 4],
        ];
        subsets.iter_mut().for_each(|x| x.sort());
        subsets.sort();
        expect.iter_mut().for_each(|x| x.sort());
        expect.sort();
        assert_eq!(subsets, expect);

        let mut subsets = Solution::subsets(vec![0]);
        let mut expect = vec![vec![], vec![0]];
        subsets.iter_mut().for_each(|x| x.sort());
        subsets.sort();
        expect.iter_mut().for_each(|x| x.sort());
        expect.sort();
        assert_eq!(subsets, expect);

        assert_eq!(Solution::subsets(vec![]), vec![vec![]]);
    }
}
