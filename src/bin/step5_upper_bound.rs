// Step5
// 目的: 二分探索アルゴリズムの理解を深めるためにupper_bound(上限)を探す実装を行う

/*
  方法
  - upper_boundについて考えられる全ての区間を扱って実装を行う
    - target == nums[i] による早期リターンは行わない方針とする
  - 実装を行う際に考えるポイントなどをまとめる

  実装するときの考え方まとめ
  - upper_bound: target < nums[i] となる最小のiを返す
    配列を nums[i] <= target,target < nums[i] に分ける境界の位置を探す
    - start側を nums[i] <= target
    - end側を  target < nums[i]
  - ループ条件は「未確定領域が空でない」という考え方をする。（見るべき未確定領域が残っているか）
    - ループ条件は区間から決められる。
    - 閉区間 [start,end] start <= i <= end
      while start <= end
    - 開区間 (start,end) start < i < end
      while 1 < end - start
    - 半開区間
      - [start,end) start <= i < end
        while start < end
      - (start,end] start < i <= end
        while start + 1 < end : start側が開区間でstart自体を含まないので+1
  - 2ポインタ（startとend）の更新
    - nums[i]を未確定領域に含めるかどうかを考える。
      - upper_boundのとき target < nums[i] である nums[i] は答えの候補（未確定領域）に含める。targetを超えるので答えになりえる。
      - upper_boundのとき nums[i] <= target である nums[i] は答えの候補（未確定領域）に含めない。target以下であるので答えになりえない。
    - ポインタ更新時に+-1するかどうか
      - 探索候補に含めないとき i が区間に含まれないようにする。
        - 端点が開いているか閉じているかに気をつける。
          - 端点から+-1が決まるわけではなく、端点を未確定領域に含めたいかどうかが先にあって、そのあとに端点の状態を見て+-1するか決める。
  - 無限ループが発生しないかを確認する
    - 未確定領域に含まれる要素が1つの時の最小ケースを用意して、無限ループにならないか(ループ毎に未確定領域が縮小するか)をチェックする

  所感
  - 最初にソートされた適当な配列[1,3,5,7]を考えて、欲しい答えを考えた時に境界をどこに置くのか考えると分かりやすく感じる。
  upper_boundにおいて、target < num[i] を満たす最小のiを探す時の境界を | で表している。
  target = 7のとき [1,3,5,7|]
  target = 4のとき [1,3|5,7]
  target = 5のとき [1,3,5|7]
  [ nums[i] <= target | target < nums[i] ]といった感じで境界で分けている。

  - left-open,right-closeな半開区間は考えることが多く、バグを埋め込みやすいと思った。
  この区間を扱わざるをえない状況がすぐに思いつかないが、可能であればleft-close,right-openな区間として扱うように番兵を利用するなどして区間を調整するのが良さそう。
  - ロジックを実装した後にコメントアウトで小さいテストケース // [1,3,5] target=5) を書いて、無限ループしないかどうか頭の中で状態遷移を追いながらデバッグして動作確認をしていた。
  ホワイトボードを用いたコーディングテストを考えると、この方法ができれば大丈夫そうだと思った。
  - left-close,right-openな区間として扱うと、探しているupper_boudの条件とポインタの更新が対になっていて分かりやすいと感じる。
  具体的には target < nums[i] を満たす最小のiを探していて、これを満たすiを見つけるたびにendポインタに入れているので、探索が終わったときにendが答え(upper_bound)になるのが分かりやすいという感覚。
  Rustで範囲を表すときにnums[0..i]とするとleft-close,right-openな範囲[0,i)になるが、扱いやすいからこうしているのかなどと思った。
*/

pub struct Solution {}
impl Solution {
    /*
      以下のコードは二分探索アルゴリズム学習のために実装しており、LeetCodeの問題(35.Search Insert Position)とは関係ありません。
    */
    pub fn upper_bound_close_to_close_interval(nums: Vec<i32>, target: i32) -> i32 {
        // [start,end]
        // start <= i <= end
        let mut start = 0;
        let mut end = nums.len() as isize - 1;

        while start <= end {
            let middle = start + (end - start) / 2;

            if target < nums[middle as usize] {
                // targetを超えるnums[middle]は答え候補に残したいので、end = middle としたいが、探索範囲を減少させるために end = middle - 1としている。
                // startが常にtarget以下となる境界の次を指すので、target < nums[middle] を満たす最小のmiddleとなり、uppper_boundとなる。
                end = middle - 1;
            } else {
                // nums[middle] <= target
                start = middle + 1;
            }
        }

        start as i32
    }

    pub fn upper_bound_close_to_open_interval(nums: Vec<i32>, target: i32) -> i32 {
        // [start,end)
        // start <= i < end
        let mut start = 0;
        let mut end = nums.len() as isize;

        while start < end {
            let middle = start + (end - start) / 2;

            if target < nums[middle as usize] {
                end = middle;
            } else {
                // nums[middle] <= target
                start = middle + 1;
            }
        }

        end as i32
    }

    pub fn upper_bound_open_to_close_interval(nums: Vec<i32>, target: i32) -> i32 {
        // (start,end]
        // start < i <= end
        if nums.is_empty() {
            return 0;
        }

        let mut start = -1;
        let mut end = nums.len() as isize - 1;

        // 未確定領域に要素が1つ残った状態で探索を終了しないと無限ループする
        while start + 1 < end {
            let middle = start + (end - start) / 2;

            if target < nums[middle as usize] {
                end = middle;
            } else {
                // nums[middle] <= target
                start = middle + 1;
            }
        }

        // target < nums[end] を満たす最小のendであればそのまま帰す。
        if target < nums[end as usize] {
            return end as i32;
        }

        // nums[end] <= target の範囲にあるので、境界を超えたところがupper_boundになる
        (end + 1) as i32
    }

    pub fn upper_bound_open_to_open_interval(nums: Vec<i32>, target: i32) -> i32 {
        // (start,end)
        // start < i < end
        let mut start = -1;
        let mut end = nums.len() as isize;

        while 1 < end - start {
            let middle = start + (end - start) / 2;

            if target < nums[middle as usize] {
                end = middle;
            } else {
                start = middle;
            }
        }

        end as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step5_close_to_close_interval_test() {
        assert_eq!(
            Solution::upper_bound_close_to_close_interval(vec![1, 3, 5, 6], 5),
            3
        );
        assert_eq!(
            Solution::upper_bound_close_to_close_interval(vec![1, 3, 5, 6], 2),
            1
        );
        assert_eq!(
            Solution::upper_bound_close_to_close_interval(vec![1, 3, 5, 6], 7),
            4
        );
        assert_eq!(
            Solution::upper_bound_close_to_close_interval(vec![1, 3], 2),
            1
        );
        assert_eq!(
            Solution::upper_bound_close_to_close_interval(vec![3, 5], 3),
            1
        );

        assert_eq!(
            Solution::upper_bound_close_to_close_interval(vec![1, 3, 5, 6], 0),
            0
        );
        assert_eq!(Solution::upper_bound_close_to_close_interval(vec![], 8), 0);
    }

    #[test]
    fn step5_close_to_open_interval_test() {
        assert_eq!(
            Solution::upper_bound_close_to_open_interval(vec![1, 3, 5, 6], 5),
            3
        );
        assert_eq!(
            Solution::upper_bound_close_to_open_interval(vec![1, 3, 5, 6], 2),
            1
        );
        assert_eq!(
            Solution::upper_bound_close_to_open_interval(vec![1, 3, 5, 6], 7),
            4
        );
        assert_eq!(
            Solution::upper_bound_close_to_open_interval(vec![1, 3], 2),
            1
        );
        assert_eq!(
            Solution::upper_bound_close_to_open_interval(vec![3, 5], 3),
            1
        );

        assert_eq!(
            Solution::upper_bound_close_to_open_interval(vec![1, 3, 5, 6], 0),
            0
        );
        assert_eq!(Solution::upper_bound_close_to_open_interval(vec![], 8), 0);
    }

    #[test]
    fn step5_open_to_close_interval_test() {
        assert_eq!(
            Solution::upper_bound_open_to_close_interval(vec![1, 3, 5, 6], 5),
            3
        );
        assert_eq!(
            Solution::upper_bound_open_to_close_interval(vec![1, 3, 5, 6], 2),
            1
        );
        assert_eq!(
            Solution::upper_bound_open_to_close_interval(vec![1, 3, 5, 6], 7),
            4
        );
        assert_eq!(
            Solution::upper_bound_open_to_close_interval(vec![1, 3], 2),
            1
        );
        assert_eq!(
            Solution::upper_bound_open_to_close_interval(vec![3, 5], 3),
            1
        );

        assert_eq!(
            Solution::upper_bound_open_to_close_interval(vec![1, 3, 5, 6], 0),
            0
        );
        assert_eq!(Solution::upper_bound_open_to_close_interval(vec![], 8), 0);
    }

    #[test]
    fn step5_open_to_open_interval_test() {
        assert_eq!(
            Solution::upper_bound_open_to_open_interval(vec![1, 3, 5, 6], 5),
            3
        );
        assert_eq!(
            Solution::upper_bound_open_to_open_interval(vec![1, 3, 5, 6], 2),
            1
        );
        assert_eq!(
            Solution::upper_bound_open_to_open_interval(vec![1, 3, 5, 6], 7),
            4
        );
        assert_eq!(
            Solution::upper_bound_open_to_open_interval(vec![1, 3], 2),
            1
        );
        assert_eq!(
            Solution::upper_bound_open_to_open_interval(vec![3, 5], 3),
            1
        );

        assert_eq!(
            Solution::upper_bound_open_to_open_interval(vec![1, 3, 5, 6], 0),
            0
        );
        assert_eq!(Solution::upper_bound_open_to_open_interval(vec![], 8), 0);
    }
}
