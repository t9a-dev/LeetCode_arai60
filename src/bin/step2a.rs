// Step2a
// 目的: Erase-remove idiomの考え方で実装してみる。

// 方法
// Step1のコードを読みやすくしてみる
// 他の人のコードを2つは読んでみること
// 正解したら終わり

// 以下をメモに残すこと
// 講師陣はどのようなコメントを残すだろうか？
// 他の人のコードを読んで考えたこと
// 改善する時に考えたこと

/*
  https://github.com/Yoshiki-Iwasa/Arai60/pull/59/changes#diff-8201a9b64da970a353f0eb106023266ce67230b908a5213848bdfa1793d8574c
  - Erase-remove idiomのRust実装例。retain(), resize()メソッドどちらも使ったことがなく新しいメソッドを知れた。

  所感
  - シンプルな考え方で一番わかりやすい。
    - 配列の順序を維持したまま0を取り除く。
    - 変更前の配列のサイズに等しくなるように配列末尾を0で埋める。
  - retainの実装(retain_mutを呼び出している)を少し読んだが、想定以上に長い実装だった。
    - 標準ライブラリで要求される効率性と安全性を担保しながら、配列をin-placeで読み書きするのは大変なんだなと思った。
  https://doc.rust-lang.org/nightly/src/alloc/vec/mod.rs.html#2432
*/

pub struct Solution {}
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let original_len = nums.len();
        // predicateに一致する値のみにする。0でない値を残す。
        nums.retain(|v| *v != 0);
        // 元のサイズと同じになるように0を追加する。
        nums.resize(original_len, 0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step2a_test() {
        let mut before_nums = vec![0, 1, 0, 3, 12];
        let after_nums = vec![1, 3, 12, 0, 0];
        Solution::move_zeroes(&mut before_nums);
        assert_eq!(before_nums, after_nums);

        let mut before_nums = vec![0];
        let after_nums = vec![0];
        Solution::move_zeroes(&mut before_nums);
        assert_eq!(before_nums, after_nums);

        let mut before_nums = vec![0, 0, 1];
        let after_nums = vec![1, 0, 0];
        Solution::move_zeroes(&mut before_nums);
        assert_eq!(before_nums, after_nums);
    }
}
