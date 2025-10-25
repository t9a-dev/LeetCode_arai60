// Step1
// 目的: step1.rsで思いついた最適化を試す。

/*
  思いついた最適化手段
  - 初期化時にソートして、k番目の値を把握しておく。
  - addで追加された値がk番目の値以下ならソートせずにk番目の値を返す。
  - ソートした場合は返却するk番目の値を更新する。

  正解してから気づいたこと
  - LeetCode上の実行速度が改善前後で150msくらい速くなったが、依然としてBeats6.45%と遅い。
  - ソートの部分をどうにかしたいが手段がわからないのでstep2で調べる。
*/

pub struct KthLargest {
    k: i32,
    kth_num: i32,
    nums: Vec<i32>,
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        // LeetCodeの定義上kをi32型としているが、配列中先頭からのある位置を表すのであればusizeとして、型レベルでkをバリデーションするべきであると考える。
        // kをusizeとした場合で0が渡されたときはpanic!意外に常に1として扱うという方法もあると考えた。
        if k <= 0 {
            panic!("k must be 1 or greater.")
        }

        let k = k - 1;
        let mut desc_sorted_nums = nums;
        desc_sorted_nums.sort_by(|a, b| b.cmp(a));

        Self {
            kth_num: *desc_sorted_nums.get(k as usize).unwrap_or(&i32::MIN),
            nums: desc_sorted_nums,
            k,
        }
    }

    fn add(&mut self, num: i32) -> i32 {
        if num <= self.kth_num {
            return self.kth_num;
        }

        self.nums.push(num);
        self.nums.sort_by(|a, b| b.cmp(a));

        match self.nums.get(self.k as usize) {
            Some(v) => {
                self.kth_num = *v;
                return *v;
            }
            None => i32::MIN,
        }
    }
}

pub struct Solution {}
impl Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step1_case1_test() {
        let mut score_manager = KthLargest::new(3, vec![4, 5, 8, 2]);
        assert_eq!(score_manager.add(3), 4);
        assert_eq!(score_manager.add(5), 5);
        assert_eq!(score_manager.add(10), 5);
        assert_eq!(score_manager.add(9), 8);
        assert_eq!(score_manager.add(4), 8);
    }

    #[test]
    fn step1_case2_test() {
        let mut score_manager = KthLargest::new(4, vec![7, 7, 7, 7, 8, 3]);
        assert_eq!(score_manager.add(2), 7);
        assert_eq!(score_manager.add(10), 7);
        assert_eq!(score_manager.add(9), 7);
        assert_eq!(score_manager.add(9), 8);
    }
}
