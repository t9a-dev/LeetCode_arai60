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
  https://github.com/h1rosaka/arai60/pull/45#discussion_r2529537136
  - 探索中に早期リターンせずに、探索終了時のポインタ位置の値が等しいかどうかで結果を返す方式。練習しておきたい解法。

  https://github.com/t9a-dev/LeetCode_arai60/pull/42/changes/BASE..ba5b009c3fd1c49d89c5d2e1c2c09e7302f3b7d4#r2650867711
  - 自分がもらったレビューコメント。
  回転済みの配列において、nums.last()は右区間に含まれる値になる。
  nums[i] <= nums.last()と比較することで、nums[i]が右区間に属するかを判定している。
  完璧に理解できているとは言えない（自分の道具として思い通りに扱えない）ものの、何を言っているのかは分かる感じ。

  https://github.com/Yoshiki-Iwasa/Arai60/pull/36#discussion_r1712955053
  ペア？にするという別の解法。
  ぱっと見よく分からないが二分探索の理解が危ういという自覚があるのでなるべく別の解法も試したい。step2a.rsでやってみる。

  改善する時に考えたこと
  - 早期リターンせずに探索終了時のポインタから答えを求める解法を実装して理解を深める。
  - 回転範囲の判定をフラグ変数にすると文脈が分断される気がしたので今回は止めた。一度しか利用しないのでその場で条件を読んだ方が良いという感覚。

  所感
  - step1の解法と比べて、回転している区間の処理とソートされている区間の単純な二分探索処理に分けられている。
  問題を細分化して、個々に対応できているので良い解法だと思った。
  - 回転の処理でendの範囲外アクセスが起きないようになっている。
  必要ないが、戻り値のチェックで配列アクセスするときに、end < nums.len() としておくことで自分を含めた読み手の不安を取り除きたいと思った。
  ここに不安を感じること自体が二分探索を理解しきれていないという感じもする。(コードを見て自信を持って範囲外アクセスは起きないと言えないところ)
  https://github.com/hayashi-ay/leetcode/pull/49/changes#r1527147960
  同じようなことを考えている人がいた。人が読むことを考えると絶対に削るべきというほどでも無いという感じだと思った。
*/

pub struct Solution {}
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return -1;
        }

        let mut start = 0;
        let mut end = nums.len();

        while start < end {
            let middle = start + (end - start) / 2;
            let last_num = *nums.last().unwrap();

            // middle ~ last_num はソートされているが、targetがlast_numを超えるので、この範囲にtargetは無い。
            // end側を捨てる。
            if nums[middle] <= last_num && target > last_num {
                end = middle;
                continue;
            }

            // middle が last_numを超えているので回転している。targetはlast_num以下なので、end側にtargetがある。
            // start側を捨てる。
            if nums[middle] > last_num && target <= last_num {
                start = middle + 1;
                continue;
            }

            // ここまで到達すると、探索範囲が回転していない（ソートされている）のでそのまま二分探索を行う。
            // endがlower_boundになる。
            if target <= nums[middle] {
                end = middle;
            } else {
                start = middle + 1;
            }
        }

        if end < nums.len() && nums[end] == target {
            return end as i32;
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step2_test() {
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
