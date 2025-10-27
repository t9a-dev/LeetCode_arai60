// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  問題の理解
  - 符号付き整数を含む配列nums、自然数kが与えられる。
  numsに出現する頻度が高いk個の値(nums[i])を配列で返す。戻り値の配列内の順序は任意。

  何がわからなかったか
  - 値と値の頻度を関連付けた状態を維持するために構造体で表現したが、構造体を最大ヒープに入れたときにソートに利用するキーを指定する方法がわからず調べた。
  具体的には自分で定義した構造体にOrd,Partialトレイトの実装をするのが初めてだったので調べながら行った。

  何を考えて解いていたか
  - 値の頻度はHashMapを利用して表現できると考えた。keyにnum,valueに出現回数（頻度）。
  - 値と頻度の関係を保ったまま頻度でソートしたい。BinaryHeapで構造体を扱うためにはOrdトレイトを実装すれば良さそう。
  - 最大ヒープに構造体を入れてk回popする。
  - top_k_frequentの引数kは配列のインデックスを表す自然数なのでusizeにしたい。(LeetCodeの採点システム通らなくなるので無理だが)

  想定ユースケース
  - アイテムや書類のIdによって表される利用・閲覧回数の履歴の集合から出現回数の多い（または少ない）上位k件のIdを取得するなど。

  正解してから気づいたこと
  - トレイトの実装をする必要があるといった知識はあったが、トレイトの実装をドキュメントを見ながら実際に行ったのが初めてだったので良い経験になった。
  - 実装の引き出しが増えているのを実感した。
  - Ordトレイトのcmpでの比較時に、frequencyが同じだった場合の結果を安定させるために、値自体(num)による比較も行ったほうが良さそう。
  - frequency_by_numでループしている箇所は、タプルでkey,valueを取り出せる。
  - 変数の命名が雑になっているなと思った。
    - FrequencyCounter -> FrequencyEntry
    - FrequencyCounterのフィールドnum -> value
    - FrequencyCounterのフィールドfrequency -> count
    - frequency_sorted_nums -> entries_by_frequency
    - result -> top_k_values
*/

use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Eq, PartialEq)]
pub struct FrequencyCounter {
    num: i32,
    frequency: i32,
}

impl Ord for FrequencyCounter {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.frequency.cmp(&other.frequency)
    }
}

impl PartialOrd for FrequencyCounter {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.frequency.cmp(&other.frequency))
    }
}

pub struct Solution {}
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        if k <= 0 {
            return vec![i32::MIN];
        }

        let k = k as usize;
        let mut frequency_by_num: HashMap<_, _> = HashMap::new();
        let mut frequency_sorted_nums: BinaryHeap<FrequencyCounter> = BinaryHeap::new();
        let mut result = vec![];

        for num in nums {
            frequency_by_num
                .entry(num)
                .and_modify(|frequency| *frequency += 1)
                .or_insert(1);
        }

        for num in frequency_by_num.keys() {
            if let Some(frequency) = frequency_by_num.get(num) {
                frequency_sorted_nums.push(FrequencyCounter {
                    num: *num,
                    frequency: *frequency,
                });
            }
        }

        for _ in 0..k {
            if let Some(frequency_counter) = frequency_sorted_nums.pop() {
                result.push(frequency_counter.num);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn step1_test() {
        let mut result = Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2);
        result.sort();
        assert_eq!(result, vec![1, 2]);

        let mut result = Solution::top_k_frequent(vec![1], 1);
        result.sort();
        assert_eq!(result, vec![1]);

        let mut result = Solution::top_k_frequent(vec![1, 2, 1, 2, 1, 2, 3, 1, 3, 2], 2);
        result.sort();
        assert_eq!(result, vec![1, 2]);
    }
}
