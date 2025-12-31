// Step2a
// 目的: 別の解法を練習する

/*
  https://github.com/Yoshiki-Iwasa/Arai60/pull/36#discussion_r1712955053
  ペア？にするという別の解法を実装してみる。

  解法の理解
  - 結局よくわからなかったのでGPT-5.2に聞いて写経した。
  - binary_search_by_key(b, f)を引数に取る。
    https://doc.rust-lang.org/std/primitive.slice.html#method.binary_search_by_key
    - bは検索キー。検索したいものそのもの。
    - fはキー抽出関数。bと比較できるように、比較に利用するキーを生成する。
  - (bool,i32)のタプルをb(検索キー)としている。
    - boolでどちらの区間かが分かるので、探索範囲を半分にできる。
      - target=0,last=2
      - [4, 5, 6, 7, 0, 1, 2]
      - [F, F, F, F, T, T, T] (nums[i] <= last)
        F=false,T=true
      - target <= last は T となるので、右区間に含まれることが分かる。一度に半分に探索範囲を絞れる。
  - fが生成するキーとbの検索キーを比較して等しいものが見つかるかどうか二分探索する。
  https://github.com/t9a-dev/LeetCode_arai60/pull/42/changes/BASE..ba5b009c3fd1c49d89c5d2e1c2c09e7302f3b7d4#r2650867711
  ここでもらったコメントの理解が進んだ気がする。

  所感
  https://doc.rust-lang.org/src/core/slice/mod.rs.html#2932-2934
    - binary_search_byの実装を見てみたら配列の境界にかなり気を使っている様子がコメントから伺えて面白かった。
    - 戻り値のインデックスを返す前にインデックスが範囲内であるかどうかを判定しているコードがある。
      https://doc.rust-lang.org/src/core/slice/mod.rs.html#2975
    - コンパイラに対してインデックスが範囲内であるという仮定を伝えている。
      https://doc.rust-lang.org/stable/std/hint/fn.assert_unchecked.html
        - 最適化ヒントと呼ばれるものらしい。(C++日本語リファレンスより)
        https://cpprefjp.github.io/lang/cpp23/portable_assumptions.html
*/

pub struct Solution {}
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return -1;
        }

        let last = nums.last().unwrap();
        //(bool ,i32) -> (targetが左右どちらの区間に属するか, 探している値)
        let target_key = (target <= *last, target);

        match nums.binary_search_by_key(&target_key, |num| (num <= last, *num)) {
            Ok(i) => i as i32,
            Err(_) => -1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step2a_test() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
        assert_eq!(Solution::search(vec![1], 0), -1);
        assert_eq!(Solution::search(vec![], 0), -1);

        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 2), 6);
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 4), 0);
        assert_eq!(Solution::search(vec![5, 1, 3], 1), 1);
        assert_eq!(Solution::search(vec![5, 1, 3], 5), 0);
        assert_eq!(Solution::search(vec![5, 1, 3], 3), 2);
    }
}
