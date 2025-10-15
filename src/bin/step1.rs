// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  問題の理解
  - 間違った理解をしていた -> 配列nums、整数kが与えられる。numsの中で合計したときにkとなるペアの数を返す。
  subarray(部分集合)という概念を知らず、そのままスルーしてしまったのが原因だと思った。

  - 配列numsが与えられるので、subarrayの合計がkとなるsubarrayの数を解として返す。
  nums=[1,1,1]の場合subarrayは[1],[1,1],[1,1]となる。index[0,1],[1,2]で同じ配列が並んでいる
  nums=[1,2,3]の場合subarrayは[1],[2],[3],[1,2],[2,3],[1,2,3]となる。
  subarrayは連続していることが条件なので、[1,3]とはならない。

  何がわからなかったか
  - 問題文を誤って理解していた。subarrayという概念を知らなかった。
  - 答えを見てもなぜ正解になるのか、腹落ちしなかったのでChatGPTの学習モードで聞いてみたところ「累積和」という概念が出てきた。
  「累積和」も初めて聞く単語だった。
  - 「累積和」について
  https://qiita.com/drken/items/56a6b68edef8fc605821

  - ある時点までの累積和でkと等しくなる部分集合が存在するかのカウントを数え上げる部分(count += freqency_count)が
  直感に反するので納得するのに時間がかかった。(過去に出現した数も重複してcountに加算しているように見える)
  nums=[1,-1,1,-1],k=0の場合に部分集合の和が0になるような集合は[0..2),[1..3)[2..4),[0..4)となる。
  解のロジックでは右端のインデックスが同じ場合([2..4),[0..4))にまとめてcountに加算するので重複して計上していないと理解した。

  何を考えて解いていたか
  -　配列を最初から足していきkと等しくなるか見る。kを超えたら合計値算出は無駄なのでやめる。
  num[i]がkと等しければ解のカウントをインクリメントする。
  num[i]のときに一つ先読みして合計値が等しければ解のカウントをインクリメントする
  - すでに出現した単一の値をHashMapでカウントする。キーをnums[i],出現数を値とする。
  - 結局自力で解けなかった。
  ↑失敗: nums=[1],k=0やnumsに負の数が含まれているケースに対応できないロジックが出来上がってしまった。

  「累積和」利用した解法の手順
  - nums=[1,2,3]のとき累積和はS=[0,1,3,6]となる。
  numsをループで回しながら、ある時点の累積和prefixを計算してkとの差need=prefx-kを求める。
  ある時点(区間)の累積和をハッシュマップのキー、値は出現回数をカウントする。
  ハッシュマップからneedで引いて存在すればcountに出現回数を足していく。

  正解してから気づいたこと
  - 問題を正しく理解できたらナイーブな実装を自分でできるか試す。(step1_naive.rs)
  - 変数名について以下の方が良いと思った。
  prefix -> prefix_sum　現在の累積和ということが分かりやすい
  frequency_count -> frequency 頻度という単語自体がカウントを含んでいるように感じられるためcountはなくて良いと思った。
*/

use std::{collections::HashMap, i32};

pub struct Solution {}
impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        let mut prefix = 0;
        let mut frequency_by_prefix: HashMap<i32, i32> = HashMap::new();
        // 累積和の配列で0が先頭に常に出現する。
        frequency_by_prefix.insert(0, 1);

        for num in nums.into_iter() {
            prefix += num;
            let need = prefix - k;

            if let Some(frequency_count) = frequency_by_prefix.get(&need) {
                count += frequency_count;
            }

            // 現時点の累積和を追加
            frequency_by_prefix
                .entry(prefix)
                .and_modify(|frequency_count| {
                    *frequency_count += 1;
                })
                .or_insert(1);
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step1_test() {
        assert_eq!(Solution::subarray_sum(vec![1, 1, 1], 2), 2);
        assert_eq!(Solution::subarray_sum(vec![1, 2, 3], 3), 2);
    }
}
