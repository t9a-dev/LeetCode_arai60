// Step2a_recursive.rs
// 目的: 動的計画法の練習をステップに分けて行う。再帰アプローチ(トップダウン)
// https://leetcode.com/problems/house-robber/solutions/156523/from-good-to-great-how-to-approach-most-of-dp-problems/

/*
  問題の理解
  - 自然数からなる配列numsを与えられる。（各家屋に置いてある金額を表している）
  家屋から金を強盗するとき、金額が最大になるように家屋を選び、最大の金額を返す。
  制約として隣接する家屋からは金を強盗できない。

  解法の考え方(理解のためにリンク先の解説を写経している)
  https://leetcode.com/problems/house-robber/solutions/156523/from-good-to-great-how-to-approach-most-of-dp-problems/
  - 強盗には2つの選択肢がある。
    - a)現在の家nums[i]を強盗する
      - 制約から隣接している前の家nums[i-1]は強盗できないが、前の前の家nums[i-2]は強盗できる
    - b)現在の家を強盗しない
      - nums[i-1]を含めたnums[i-n]を強盗するかを後で決める
  - a),b)の選択肢のどちらが合計金額が大きくなるかを計算する必要がある。
    - a) 現在の家nums[i]を強盗したときの金額 + 前の前の家を強盗したときの金額nums[i-2]
    - b) 現在の家を強盗しない時nums[i-1]
  - 再帰関数(run_robber())のケース
    - 基本ケース
      iが0以上
    - 再帰ケース
      - a) nums[i] + run_rob(i-2)
      - b) run_rob(i-1)
      a),b)のうち利益の大きい（金額が大きい）方を知りたい。(run_rob(i) + run_rob(i-2)).max(run_rob(i-1))

    計算量の見積もり
    - 他の人のコードで言及されていたのを少し覚えていて、再帰ケースで i-2,i-1 による再帰を行っているのでフィボナッチ数列に近いといった内容だった気がする
    しかし、具体的な計算量の求め方が分からない。ChatGPT(GPT-5.1)に聞く。
    再帰ケースが2つに別れているので、再帰深さnとすると時間計算量はO(2 ^ n)になる。
    入力制約は 1 <= nums.len() <= 100 なので 2 ^ 100 = 1.2676506002E30 (1 穣 2676 秭 5060 垓 0228 京 2294 兆 0149 億 6703 万 2053) 読み方すらわからないが計算量が爆発しているのは分かった。
    メモ化が必要なことが分かる。step2b_recursive_memo.rsで行う。
*/

// Solution::robの中で生成したrobberインスタンスのメソッドとして、Robber::run_robberが見えるのが気持ち悪いのでmodにしている。
/*
  let house_robber = Robber::new(nums);
  house_robber.run_robber(); // <- これができないように。
*/
mod house_robber {
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
            self.run_robber((self.targets.len() - 1) as isize)
        }

        fn run_robber(&self, i: isize) -> i32 {
            if i < 0 {
                return 0;
            }

            (self.targets[i as usize] + self.run_robber(i - 2)).max(self.run_robber(i - 1))
        }
    }
}

use house_robber::Robber;

pub struct Solution {}
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        /*
          このコードはLeetCode採点システムで Time Limit Exceeded となります。
          nums = [114,117,207,117,235,82,90,67,143,146,53,108,200,91,80,223,58,170,110,236,81,90,222,160,165,195,187,199,114,235,197,187,69,129,64,214,228,78,188,67,205,94,205,169,241,202,144,240]
        */
        let house_robber = Robber::new(nums);
        house_robber.collect_max_amount()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step2a_recursive_test() {
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
    fn step2a_recursive_panic_test() {
        Solution::rob(vec![]);
    }
}
