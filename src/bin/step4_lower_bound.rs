// Step4
// 目的: lower_bound(下限)を探す二分探索アルゴリズムの実装

/*
  方法
  - lower_boundについて考えられる全ての区間を扱って実装を行う
    - target == nums[i] による早期リターンは行わない方針とする
  - 実装を行う際に考えるポイントなどをまとめる

  実装するときの考え方まとめ
  - lower_bound: target <= nums[i] となる最小のiを返す
    配列を nums[i] < target, target <= nums[i] に分ける境界の位置を探す
    - start側を nums[i] < target
    - end側を target <= nums[i]
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
      - lower_boundのとき nums[i] < target である nums[i] は答えの候補（未確定領域）に含めない。target未満であるので答えになりえない。
      - lower_boundのとき target <= nums[i] である nums[i] は答えの候補（未確定領域）に含める。target以上であるので答えになりえる。
    - ポインタ更新時に+-1するかどうか
      - 探索候補に含めないとき i が区間に含まれないようにする。
        - 端点が開いているか閉じているかに気をつける。
          - 端点から+-1が決まるわけではなく、端点を未確定領域に含めたいかどうかが先にあって、そのあとに端点の状態を見て+-1するか決める。
  - 無限ループが発生しないかを確認する
    - 未確定領域に含まれる要素が1つの時の最小ケースを用意して、無限ループにならないか(ループ毎に未確定領域が縮小するか)をチェックする

  所感
  - 最初にソートされた適当な配列[1,3,5,7]を考えて、欲しい答えを考えた時に境界をどこに置くのか考えると分かりやすく感じる。
  lower_boundにおいて、target <= num[i] を満たす最小のiを探す時の境界を | で表している。
  target = 7のとき [1,3,5|7]
  target = 4のとき [1,3|5,7]
  target = 5のとき [1,3|5,7]
  [ nums[i] < target | target <= nums[i] ]といった感じで境界で分けている。

  - left-open,right-closeな半開区間が難しく感じる。
    - 探索範囲が空になるまでループを回すと無限ループになる
    - 探査範囲に要素が1つ残った状態で探索を終了して、最後に値をチェックしてから返す必要がある。
  - upper_boundも練習して、境界が変わった時に実装できるかで理解度チェックする。
*/

pub struct Solution {}
impl Solution {
    pub fn search_insert_close_to_close_interval(nums: Vec<i32>, target: i32) -> i32 {
        // [start,end]
        // start <= i <= end
        let mut start = 0;
        let mut end = nums.len() as isize - 1;

        while start <= end {
            let middle = start + (end - start) / 2;

            if nums[middle as usize] < target {
                // middleの位置にある値は答えになりえないので探索候補から外す。
                // startは middle < target となる境界の1つ右側を常に指し示すので、lower_boundになっている。
                start = middle + 1;
            } else {
                /*
                  middleの位置にある値middle_valueは答えになりうるので探索候補に含めたい(end = middle)が、
                  探索範囲が縮小しなくなることを避けるためにend = middle - 1としている。
                */
                end = middle - 1;
            }
        }

        start as i32
    }

    pub fn search_insert_close_to_open_interval(nums: Vec<i32>, target: i32) -> i32 {
        // [start,end)
        // start <= i < end
        let mut start = 0;
        let mut end = nums.len() as isize;

        while start < end {
            let middle = start + (end - start) / 2;

            if nums[middle as usize] < target {
                start = middle + 1;
            } else {
                end = middle;
            }
        }

        end as i32
    }

    pub fn search_insert_open_to_close_interval(nums: Vec<i32>, target: i32) -> i32 {
        // (start,end]
        // start < i <= end
        if nums.is_empty() {
            return 0;
        }

        let mut start = -1;
        // endを閉じた端点として扱う練習をしているので、end = nums.len() as isize と番兵を使わずに実装している。
        let mut end = nums.len() as isize - 1;

        // 未確定領域に要素が1つ残った状態で探索が終了しないと無限ループする
        while start + 1 < end {
            let middle = start + (end - start) / 2;

            if nums[middle as usize] < target {
                start = middle;
            } else {
                end = middle;
            }
        }

        if nums[end as usize] < target {
            return (end + 1) as i32;
        }

        end as i32
    }

    pub fn search_insert_open_to_open_interval(nums: Vec<i32>, target: i32) -> i32 {
        // (start,end)
        // start < i < end
        let mut start = -1;
        let mut end = nums.len() as isize;

        while 1 < end - start {
            let middle = start + (end - start) / 2;

            if nums[middle as usize] < target {
                start = middle;
            } else {
                end = middle;
            }
        }

        end as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step4d_close_to_close_interval_test() {
        assert_eq!(
            Solution::search_insert_close_to_close_interval(vec![1, 3, 5, 6], 5),
            2
        );
        assert_eq!(
            Solution::search_insert_close_to_close_interval(vec![1, 3, 5, 6], 2),
            1
        );
        assert_eq!(
            Solution::search_insert_close_to_close_interval(vec![1, 3, 5, 6], 7),
            4
        );
        assert_eq!(
            Solution::search_insert_close_to_close_interval(vec![1, 3], 2),
            1
        );
        assert_eq!(
            Solution::search_insert_open_to_close_interval(vec![3, 5], 3),
            0
        );

        assert_eq!(
            Solution::search_insert_close_to_close_interval(vec![1, 3, 5, 6], 0),
            0
        );
        assert_eq!(
            Solution::search_insert_close_to_close_interval(vec![], 8),
            0
        );
    }

    #[test]
    fn step4d_close_to_open_interval_test() {
        assert_eq!(
            Solution::search_insert_close_to_open_interval(vec![1, 3, 5, 6], 5),
            2
        );
        assert_eq!(
            Solution::search_insert_close_to_open_interval(vec![1, 3, 5, 6], 2),
            1
        );
        assert_eq!(
            Solution::search_insert_close_to_open_interval(vec![1, 3, 5, 6], 7),
            4
        );
        assert_eq!(
            Solution::search_insert_close_to_open_interval(vec![1, 3], 2),
            1
        );
        assert_eq!(
            Solution::search_insert_open_to_close_interval(vec![3, 5], 3),
            0
        );

        assert_eq!(
            Solution::search_insert_close_to_open_interval(vec![1, 3, 5, 6], 0),
            0
        );
        assert_eq!(Solution::search_insert_close_to_open_interval(vec![], 8), 0);
    }

    #[test]
    fn step4d_open_to_close_interval_test() {
        assert_eq!(
            Solution::search_insert_open_to_close_interval(vec![1, 3, 5, 6], 5),
            2
        );
        assert_eq!(
            Solution::search_insert_open_to_close_interval(vec![1, 3, 5, 6], 2),
            1
        );
        assert_eq!(
            Solution::search_insert_open_to_close_interval(vec![1, 3, 5, 6], 7),
            4
        );
        assert_eq!(
            Solution::search_insert_open_to_close_interval(vec![1, 3], 2),
            1
        );
        assert_eq!(
            Solution::search_insert_open_to_close_interval(vec![3, 5], 3),
            0
        );

        assert_eq!(
            Solution::search_insert_open_to_close_interval(vec![1, 3, 5, 6], 0),
            0
        );
        assert_eq!(Solution::search_insert_open_to_close_interval(vec![], 8), 0);
    }

    #[test]
    fn step4d_open_to_open_interval_test() {
        assert_eq!(
            Solution::search_insert_open_to_open_interval(vec![1, 3, 5, 6], 5),
            2
        );
        assert_eq!(
            Solution::search_insert_open_to_open_interval(vec![1, 3, 5, 6], 2),
            1
        );
        assert_eq!(
            Solution::search_insert_open_to_open_interval(vec![1, 3, 5, 6], 7),
            4
        );
        assert_eq!(
            Solution::search_insert_open_to_open_interval(vec![1, 3], 2),
            1
        );
        assert_eq!(
            Solution::search_insert_open_to_close_interval(vec![3, 5], 3),
            0
        );

        assert_eq!(
            Solution::search_insert_open_to_open_interval(vec![1, 3, 5, 6], 0),
            0
        );
        assert_eq!(Solution::search_insert_open_to_open_interval(vec![], 8), 0);
    }
}
