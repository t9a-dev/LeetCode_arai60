// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  Nは入力のサイズとする。
  時間計算量: O(log N) BinaryHeap(優先度付きキュー)のpop操作が支配的。
  空間計算量: O(N) BinaryHeapに入力を全て積んでいる。
*/

/*
  1回目: 3分50秒
  2回目: 4分14秒
  3回目: 3分38秒
*/

use std::{cmp::Reverse, collections::BinaryHeap};

pub struct KthLargest {
    min_heap: BinaryHeap<Reverse<i32>>,
    k: usize,
}

impl KthLargest {
    pub fn new(k: i32, nums: Vec<i32>) -> Self {
        if k <= 0 {
            panic!("k must be 1 or greater");
        }

        Self {
            min_heap: BinaryHeap::from_iter(nums.into_iter().map(Reverse)),
            k: k as usize,
        }
    }

    pub fn add(&mut self, num: i32) -> i32 {
        self.min_heap.push(Reverse(num));

        while self.min_heap.len() > self.k {
            self.min_heap.pop();
        }

        self.min_heap
            .peek()
            .map(|Reverse(v)| *v)
            .unwrap_or(i32::MAX)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step3_case1_test() {
        let mut kth_largest_finder = KthLargest::new(3, vec![4, 5, 8, 2]);
        assert_eq!(kth_largest_finder.add(3), 4);
        assert_eq!(kth_largest_finder.add(5), 5);
        assert_eq!(kth_largest_finder.add(10), 5);
        assert_eq!(kth_largest_finder.add(9), 8);
        assert_eq!(kth_largest_finder.add(4), 8);
    }

    #[test]
    fn step3_case_2_test() {
        let mut kth_largest_finder = KthLargest::new(4, vec![7, 7, 7, 7, 8, 3]);
        assert_eq!(kth_largest_finder.add(2), 7);
        assert_eq!(kth_largest_finder.add(10), 7);
        assert_eq!(kth_largest_finder.add(9), 7);
        assert_eq!(kth_largest_finder.add(9), 8);
    }
}
