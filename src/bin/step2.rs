// Step2
// 目的: 自然な書き方を考えて整理する

// 方法
// Step1のコードを読みやすくしてみる
// 他の人のコードを2つは読んでみること
// 正解したら終わり

// 以下をメモに残すこと
// 講師陣はどのようなコメントを残すだろうか？
// 他の人のコードを読んで考えたこと
// 改善する時に考えたこと

/*
  他の人のコードを読んで考えたこと
  - prefixよりも以下で採用されている命名prefix_sumの方が累積和の途中の和であることがわかりやすくて良いと思った。
  https://github.com/nanae772/leetcode-arai60/pull/17/files#diff-a5a94a8ce8cfe38ec49396d3322dfd79c93117aebb1be79c61c7871a080cc2ffR7

  - c++にはexclusive_scan()なるものがあるそう。
  https://github.com/colorbox/leetcode/pull/30/files#r1867764633
  Rustだと似たようなメソッドでscan()メソッドがあったが動き方が若干違う（初期値が戻り値の先頭にこない）のと、
  自分が採用している解法だと累積和の配列をまとめて取得する場面が無いので使わない。

  改善する時に考えたこと
  -　変数名について。
  prefix -> prefix_sum　現在の累積和ということが分かりやすい
  frequency_count -> frequency 頻度(frequency)という単語自体がカウントを含んでいるように感じられるためcountはなくて良いと思った。
  need -> complement ある時点の累積和とkの差を埋めるような補数を探しているので、complementの方がより具体的だと思った。

  - numsはi32型の数値が入っているので、個々の数値がi32上限に近い数値が多い場合、累積和の型をi32にしておくと計算時に桁溢れすることが考えられる。
  桁溢れするケースを考慮してprefix_sum周りの型はi64にしておく。
  i64 / i32 = 4 294 967 298となり、numsにi32::MAXを4 294 967 298個詰めないとi64を桁溢れしないので十分だろうという見積もり。
  i32の累積和を求めるのにi64型で十分だろうという根拠がすぐに思いつかず調べたので、一つ上の型を使っておけば十分だろうという覚え方で良さそう。
  isizeなどのアーキテクチャー依存の型もあるが、これはポインタやメモリなどのアドレスを表すのに使う型であることがついでに分かった。
  今まで少し勘違いしていたが、32bitアーキテクチャのコンピューターでu64,i64が扱えないわけではなく、32bit幅のレジスタを2つ使えば計算できる。
  https://doc.rust-lang.org/reference/types/numeric.html#r-type.numeric.int.size

  - countもi64にしておかないとnums[1,1,1,...],k=1で大量の同じ値をカウントするケースに対応できないのでi64にしておく。
  戻り値に対してダウンキャストすることになるので、失敗したらi32::MINを返す。-1より例外であることが分かりやすそうなので。

  - 改善後のコードで`as`によるキャストを行っているが、常にアップキャスト(i32 -> i64)になるので安全だという認識
  ダウンキャスト(i64 -> i32など)の場合はasによるキャストだとビットトリムが行われて例外などが発生しないので注意する必要がある。
  ダウンキャストする場合Rustであれば、`as`の代わりに`a.try_into().unwrap_or(i64::MIN)`やResult型で処理するのが良さそう。

  - HashMapの型パラメータ指定はHashMap<_, _>のように型推論させられるので、こちらのほうがすっきり書けて良いと感じた。
  今回の場合だと、変数宣言のprefix_sum: i64は明示的に書く必要があるが、これを入れるHashMapの型は変数の型から推論できることが自明なので。
*/

use std::{collections::HashMap, i32};

pub struct Solution {}
impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        // ループの外で一度だけアップキャスト
        let k = k as i64;
        let mut count: i64 = 0;
        let mut prefix_sum: i64 = 0;
        let mut frequency_by_prefix_sum: HashMap<_, _> = HashMap::new();
        // 累積和の配列先頭に0が必ず存在する
        frequency_by_prefix_sum.insert(0, 1);

        for num in nums.into_iter() {
            prefix_sum += num as i64;
            let complement = prefix_sum - k;

            // これまでに合計値が補数と等しい累積和があるか確認
            if let Some(frequency) = frequency_by_prefix_sum.get(&complement) {
                count += frequency;
            }

            frequency_by_prefix_sum
                .entry(prefix_sum)
                .and_modify(|frequency| *frequency += 1)
                .or_insert(1);
        }

        count.try_into().unwrap_or(i32::MIN)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step2_test() {
        assert_eq!(Solution::subarray_sum(vec![1, 1, 1], 2), 2);
        assert_eq!(Solution::subarray_sum(vec![1, 2, 3], 3), 2);

        assert_eq!(Solution::subarray_sum(vec![1, 1, 1], 1), 3);
        assert_eq!(
            Solution::subarray_sum(vec![i32::MAX, i32::MAX, i32::MAX], i32::MAX),
            3
        );
        assert_eq!(
            Solution::subarray_sum(vec![i32::MIN, i32::MIN, i32::MIN], i32::MIN),
            3
        );
    }
}
