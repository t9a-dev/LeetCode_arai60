// Step2b_recursive_memo.rs
// 目的: 動的計画法の練習をステップに分けて行う。再帰+メモ化アプローチ(トップダウン)
// https://leetcode.com/problems/house-robber/solutions/156523/from-good-to-great-how-to-approach-most-of-dp-problems/

/*
  問題の理解
  - 自然数からなる配列numsを与えられる。（各家屋に置いてある金額を表している）
  家屋から金を強盗するとき、金額が最大になるように家屋を選び、最大の金額を返す。
  制約として隣接する家屋からは金を強盗できない。

  メモ化について
  - iが重複する箇所について計算結果をメモリ上に保存しておいてそれらを参照するようにする。

  メモ化後の計算量の見積もり
    n = nums.len()
    時間計算量: O(n) HashMapからiで計算済みの値を取得する操作の計算量はO(1)で行える。
    空間計算量: O(n) 0..n をkeyとするデータを保持する。

  所感
  - HashMapを利用せずVecでも行えるが、HashMapの方がVecに比べて配列の範囲外アクセスを気にすることなく、データ構造(HashMap)のAPI経由で安全かつ気軽に扱えて読み手にも優しいのでHashMapを採用した。
  HashMapはkeyのハッシュ化にコストが発生するが、keyにはisizeといった実行時に固定長になる型を利用しているので、殆どの場合においてここが問題になることは無いと考えた。
  - index_to_amountを構造体のフィールドにすると、インスタンスをmutにする必要があり(HashMapへのinsert操作)、呼び出し側から見た時インスタンスで操作を行うと内部状態が変更されることを示唆する。
  利用側から考えると内部的にメモ化のためのキャッシュを利用していることは知らなくて良いことなので、フィールドに値を持たせず、インスタンスをimmutableにした方が良いと考えた。
  - 内部状態が変化しようがしまいがスレッドセーフであることは変わらない。内部状態をスレッド間で共有しない（できない）ため。
*/

// Solution::robの中で生成したrobberインスタンスのメソッドとして、Robber::run_robberが見えるのが気持ち悪いので意図的に別モジュールとしている。
/*
  let house_robber = Robber::new(nums);
  robber.run_robber(); // <- これができないように。
*/
mod house_robber {
    use std::collections::HashMap;

    pub struct Robber {
        targets: Vec<i32>,
    }

    impl Robber {
        pub fn new(targets: Vec<i32>) -> Self {
            if targets.is_empty() {
                panic!("targets must not be empty.");
            }

            Self { targets }
        }

        pub fn collect_max_amount(&self) -> i32 {
            let mut index_to_amount: HashMap<isize, i32> = HashMap::new();
            self.run_robber((self.targets.len() - 1) as isize, &mut index_to_amount)
        }

        fn run_robber(&self, i: isize, index_to_amount: &mut HashMap<isize, i32>) -> i32 {
            if i < 0 {
                return 0;
            }

            if let Some(amount_cache) = index_to_amount.get(&i) {
                return *amount_cache;
            }

            let amount = (self.targets[i as usize] + self.run_robber(i - 2, index_to_amount))
                .max(self.run_robber(i - 1, index_to_amount));

            index_to_amount.insert(i, amount);
            amount
        }
    }
}

use house_robber::Robber;

pub struct Solution {}
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let house_robber = Robber::new(nums);
        house_robber.collect_max_amount()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step2b_recursive_memo_test() {
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
        assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1]), 12);
        assert_eq!(Solution::rob(vec![2, 1, 1, 2]), 4);
        assert_eq!(Solution::rob(vec![4, 0, 1]), 5);
        assert_eq!(Solution::rob(vec![4, 0]), 4);
        assert_eq!(Solution::rob(vec![2, 3]), 3);
        assert_eq!(Solution::rob(vec![2]), 2);
        assert_eq!(Solution::rob(vec![0]), 0);
    }

    #[test]
    #[should_panic]
    fn step2b_recursive_memo_panic_test() {
        Solution::rob(vec![]);
    }
}
