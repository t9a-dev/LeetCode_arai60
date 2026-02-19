// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  問題の理解
  - 整数からなる配列candidates,整数targetが与えられる。
  合計値がtargetとなるような重複のないcandidates[i]の組み合わせを全て返す。
  candidates[i]は同じ値を何度使っても良い。
  candidates=[2,3,5], target=8のとき、
  out=[[2,2,2,2],[2,3,3],[3,5]]

  何を考えて解いていたか
  - 何がしたいのかはわかるので、手作業でやることを考える。
  補数(complement) = target - candidates[i] を求める。
  同じ値candidates[i]は何度でも使えるので、候補から除くことはできない。
  決定木として考えると、complementが2以上のときは更に決定木を辿る必要がある。
  candidates=[2,3,5], target=8を決定木の図として書いて見ると、3 -> 5のパスと 5 -> 3 のパスでノードが同じなので重複する結果が得られる。
  ナイーブな対応方法は、sort + setによる重複管理だがこのあたりでbacktrackingが使えそうな感じがする。
  backtrackingではcadidates[i]をpopした残りを次に引き継いで、すぐ元に戻すイメージだがここでpopしてしまうと、何度でも同じ値を使えるという制約に反しているのでおかしなことになる。
  他の方法は思いつかないのでナイーブな実装方針とする。
    - complement = target - candidates[i] を求めながら再帰処理を行う。
      - base_case
      complement < 2 then return
      complement == 0 then combination.push(candidates[i])

      - recursive_case
      complement = target - candidates[i]
      f(candidates, complement, combination)
      combinations.push(combination)

      combinations.foreach(combination => combination.sort())
      HashSet::from_iter(combinations.into_iter())

  コードを書いてからになってしまったがLeet Code採点システム提出前に時間計算量と実行時間の概算見積もりを行う。
  n = candidates.len()
  m = target
  f(n, m) = cost(combination.clone()) + n * f(n ,m - 2)　のような計算式になると思うが、ここからBig-O記法の時間計算量にする方法がよく分からない。
  f(n) = n * f(n-1) のときは n - 1, n - 2, n - 3となり階乗になるのが感覚として分かるが、n * (m - 2) のような場合はよく分からない。
  実際に値を当てはめると答えを最後まで求めるまでもなく、n * mにはならないのでn ^ mとして考える。
  f(m - 2) としたのは問題の制約からcandidates[i]の最小値が2であるため、最悪ケースとして2を利用した。
  時間計算量: O(n ^ m)
  空間計算量: O(n ^ m)

  問題の制約:
  1 <= candidates.len <= 30
  2 <= candidates[i] <= 40
  1 <= target <= 40
  candidatesの要素は重複していない

  時間計算量だけ見ると 40 ^ 30 の時点で計算量が爆発しているので、LeetCode採点システムでTime Limit Exceededになりそう。
  Acceptedとなった。Acceptedになったということは明らかに時間計算量の見積もりが間違っていることは分かる。
  LeetCode採点システムでは以下のように表示された。
  時間計算量: O(N(T / M+1))
  空間計算量: O(T / M)

  GPT-5.2に聞いてみる。
    n = candidates.len()
    t = target
    m = min(candidates) 入力の制約では2。
    d = t / m 木の深さ。 ここが分かっていなかった。target < 2で底を枝刈りしているが計算量としては無視している。
  candidates=[2,3,5] target=8のとき、最悪計算量ではcandidates[0]である2で考える。一番深い位置まで木の枝が伸びるイメージ。
  d = t / m = 8 / 2 = 4となる。緒間的にもtarget=8のとき、candidates[i]との補数を求めながら単調減少していくと4回処理を行うことが分かる。（target < 2の枝刈りは無視）
  f(n, t) = cost(combination.clone()) + n * f(n, t - m) のような感じになり、 f(n, t - m)の再帰がどこまで深くなるかの見積もり方が分かっていなかった。
  O(n ^ (t / m))になると思うので、LeetCode採点システムが表示している計算量(O(N(T / M+1)))が間違ってそう。
  補助空間計算量も自分のコードでは重複を含んだ配列をcombinationsで保持しているので、LeetCode採点システムが表示している O(T / M)にはならないと思った。
  重複排除前にcombinationsにパスを保持する部分が支配的なので、O(n ^ (t / m) * (t / m))が正しいと思った。
  時間計算量 O(n ^ (t / m))による実行時間の概算見積を行う。
  30 ^ (40 / 2) / 10 ^ 8 = 3.486784401E21 となり計算量が爆発している。
  改めてLeetCode問題文を見るとテストケースの制約として、答えとなるような一意の組み合わせの数が150以下になるとある。ただ、この制限を適用してどう実行時間の概算見積をすればよいか分からない。
  GPT-5.2に聞いてみたが、重複したパスも全て見る実装なので組み合わせの数が150になる制限を適用できないとのことだった。
  折角なので概算見積を行いたかったが、LeetCode採点システムのテストケースはAcceptedしたときのものは見られなさそうなので諦める。

  何がわからなかったか
  - backtrackingによる重複を発生させないアルゴリズム

  所感
  - backtrackingアルゴリズムを思い通りに扱えないので、少し問題設定が変わると対応できない。慣れの問題ではあるので気にせず進める。
*/

use std::collections::HashSet;

pub struct Solution {}
impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut combination = Vec::new();
        let mut combinations = Vec::new();

        Self::make_combination_sum(&candidates, target, &mut combination, &mut combinations);
        combinations.iter_mut().for_each(|c| c.sort());
        let unique_combinations: HashSet<_> = HashSet::from_iter(combinations.into_iter());

        unique_combinations.into_iter().collect()
    }

    fn make_combination_sum(
        candidates: &[i32],
        target: i32,
        combination: &mut Vec<i32>,
        combinations: &mut Vec<Vec<i32>>,
    ) {
        if target == 0 {
            combinations.push(combination.to_vec());
            return;
        }
        if target < 2 {
            return;
        }

        for i in 0..candidates.len() {
            let complement = target - candidates[i];

            combination.push(candidates[i]);
            Self::make_combination_sum(candidates, complement, combination, combinations);
            combination.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step1_test() {
        let mut expect = vec![vec![2, 2, 3], vec![7]];
        let mut actual = Solution::combination_sum(vec![2, 3, 6, 7], 7);
        expect.iter_mut().for_each(|x| x.sort());
        expect.sort();
        actual.iter_mut().for_each(|x| x.sort());
        actual.sort();
        assert_eq!(expect, actual);

        let mut expect = vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]];
        let mut actual = Solution::combination_sum(vec![2, 3, 5], 8);
        expect.iter_mut().for_each(|x| x.sort());
        expect.sort();
        actual.iter_mut().for_each(|x| x.sort());
        actual.sort();
        assert_eq!(expect, actual);

        let expect = Vec::<Vec<i32>>::new();
        let actual = Solution::combination_sum(vec![2], 1);
        assert_eq!(expect, actual);
    }
}
