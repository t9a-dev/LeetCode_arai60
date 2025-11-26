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
  - memoを利用した再帰処理。コードがシンプルで分かりやすい。
  https://github.com/ryosuketc/leetcode_arai60/pull/48/files#diff-c2f713990cb59b667e7df8442fafd04660344b310f479b9050e98b47f3ef0c2b

  - 一次元DPテーブルによる解法。コード自体はシンプルで分かりやすいが、何をしているのかすぐに理解できないので一度実装しておくと良さそう。step2c_iterative_memo.rs
  https://github.com/ryosuketc/leetcode_arai60/pull/48/files#diff-73577784a5f16e20a041c84517c4526dda887edd563c682e9194b0b2bc7c0b94

  - 動的計画法についてまとまっている資料。別のステップでこの手順で一通り実装するのが良さそう。step2<a|b>_*.rs
  https://github.com/olsen-blue/Arai60/pull/35/files#diff-3cad6c6001234a922d7e9a0b5da82cd9db4dc34edcb80f9dc139f5e9bc09ced4R69-R70
  https://leetcode.com/problems/house-robber/solutions/156523/from-good-to-great-how-to-approach-most-of-dp-problems/

  - int型(4byte)のような値は64bitアーキテクチャにおいて(8byte)の参照渡しするよりも、値渡しをしたほうが参照を介するよりも軽量になるというコメント。
  https://github.com/potrue/leetcode/pull/35#discussion_r2254099056
  https://github.com/maeken4/Arai60/pull/9#discussion_r2160364187
  https://github.com/potrue/leetcode/pull/36#discussion_r2253883669

  C++,Cにおけるint型は4byte
  https://learn.microsoft.com/ja-jp/cpp/cpp/data-type-ranges?view=msvc-170

  Rustのケースで考えると、
    - usizeはプラットフォーム依存なので、現代(2025年)の一般的なCPUアーキテクチャ(64bit)では8byteとなる。
    - 参照渡しはメモリアドレスを渡すのでusizeとなる。
    - Rustにおいて、型指定のない整数はi32として扱われる。
      https://github.com/rust-lang/rfcs/blob/master/text/0212-restore-int-fallback.md
  i32(4byte)のような整数型は参照渡し(8byte)よりも、値渡し(4byte)した方がよいと理解した。
  https://doc.rust-lang.org/std/primitive.usize.html#:~:text=The%20size%20of%20this%20primitive%20is%20how%20many%20bytes%20it%20takes%20to%20reference%20any%20location%20in%20memory%2E%20For%20example%2C%20on%20a%2032%20bit%20target%2C%20this%20is%204%20bytes%20and%20on%20a%2064%20bit%20target%2C%20this%20is%208%20bytes
  こういうミクロな視点での最適化は一瞬ためらうが、最適化前後におけるコードの書き方がほぼ変わらず読み手に負荷を与えないので大丈夫だという感覚。
  ```rust
  fn main() {
    assert_eq!(4, std::mem::size_of::<i32>());
    assert_eq!(8, std::mem::size_of::<&i32>());
  }
  ```
  https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=a82e3b6d4de55605cd374030bf67202e


  改善する時に考えたこと
  - 変数名についてprevious_previous_robbed -> two_previous_robbed_moneyの方が良さそう
  - 書きながらmatchによる変数への代入のコードを思いついた。この程度であればパズルになっておらずすっきりと書けているので大丈夫だという感覚。
*/

pub struct Solution {}
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            panic!("nums must not be empty.");
        }

        let (mut two_previous_robbed_money, mut previous_robbed_money) = match nums.len() {
            1 => return nums[0],
            _ => (nums[0], nums[0].max(nums[1])),
        };

        for i in 2..nums.len() {
            let robbed_money = nums[i] + two_previous_robbed_money;
            two_previous_robbed_money = previous_robbed_money;
            previous_robbed_money = previous_robbed_money.max(robbed_money);
        }

        previous_robbed_money
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step2_test() {
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
    fn step2_panic_test() {
        Solution::rob(vec![]);
    }
}
