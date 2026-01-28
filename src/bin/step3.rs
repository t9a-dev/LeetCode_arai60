// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  n = n
  時間計算量: O(n)
  空間計算量: O(n) <- 最初1スタックフレームあたりがO(1)なのでO(1)になると思ったが違った。再帰処理でn回呼び出すのでスタックフレームをn個積む。
*/

/*
  1回目: 1分55秒
  2回目: 2分09秒
  3回目: 2分21秒
*/

/*
n=3,k=4
      0
     / \
    0   1 <- parent_value=1
   / \ / \
  0  1 1  0 <- k=4
*/

pub struct Solution {}
impl Solution {
    const BINARY_TREE_ROUTES: [[i32; 2]; 2] = [[0, 1], [1, 0]];

    pub fn kth_grammar(n: i32, k: i32) -> i32 {
        if n <= 0 || k <= 0 {
            panic!("n and k must be greater than 0");
        }

        if n == 1 {
            return 0;
        }
        let parent_value = Self::kth_grammar(n - 1, k - (k / 2));

        Self::BINARY_TREE_ROUTES[parent_value as usize][((k + 1) % 2) as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step3_test() {
        assert_eq!(Solution::kth_grammar(1, 1), 0);
        assert_eq!(Solution::kth_grammar(2, 1), 0);
        assert_eq!(Solution::kth_grammar(2, 2), 1);
    }

    #[test]
    #[should_panic]
    fn step3_kth_grammar_n1_k0_panics() {
        Solution::kth_grammar(1, 0);
    }

    #[test]
    #[should_panic]
    fn step3_kth_grammar_n0_k1_panics() {
        Solution::kth_grammar(0, 1);
    }

    #[test]
    #[should_panic]
    fn step3_kth_grammar_n0_k0_panics() {
        Solution::kth_grammar(0, 0);
    }

    #[test]
    #[should_panic]
    fn step3_kth_grammar_negative_n_panics() {
        Solution::kth_grammar(-1, 0);
    }

    #[test]
    #[should_panic]
    fn step3_kth_grammar_negative_k_panics() {
        Solution::kth_grammar(0, -1);
    }
}
