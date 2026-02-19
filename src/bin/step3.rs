// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  n = candidates.len()
  m = candidates.min()
  t = target
  時間計算量: O(n ^ (t / m))
  空間計算量: O((t / m))
*/

/*
  1回目: 7分44秒
  2回目: 5分50秒
  3回目: 5分05秒
*/

/*
  所感:
  - target ~ 0 までの補数を計算する方向で一度目は書いたが、0 ~ targetとしたほうが素直な感じがしたので、この書き方に落ち着いた。
  具体的には、make_combinationsの引数sum,target_sumの変数名で値の関係を表せるのでより良いと思った。
  - 入力の制約としてあり得ないが、candidates[i]にマイナスの値があると、target_sumに到達せず（sumが単調増加しなくなる）に無限ループになる。
  LeetCode採点システムに通らなくなるが、関数のシグネチャを変更して自然数のみを扱うようにすることで、型レベルでこれを防ぐことができると思った。step4.rsで書いておく。
*/

pub struct Solution {}
impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut all_combinations = Vec::new();
        let mut combination = Vec::new();

        Self::make_combinations(
            &candidates,
            0,
            target,
            &mut combination,
            &mut all_combinations,
        );

        all_combinations
    }

    fn make_combinations(
        candidates: &[i32],
        sum: i32,
        target_sum: i32,
        combination: &mut Vec<i32>,
        all_combinations: &mut Vec<Vec<i32>>,
    ) {
        if sum == target_sum {
            all_combinations.push(combination.to_vec());
            return;
        }

        let Some((&candidate, rest_candidates)) = candidates.split_first() else {
            return;
        };
        let candidate_sum = candidate + sum;

        if candidate_sum <= target_sum {
            combination.push(candidate);
            Self::make_combinations(
                candidates,
                candidate_sum,
                target_sum,
                combination,
                all_combinations,
            );
            combination.pop();
        }

        Self::make_combinations(
            rest_candidates,
            sum,
            target_sum,
            combination,
            all_combinations,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step3_test() {
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
