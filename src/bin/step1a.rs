// Step1a
// 目的: step1のリファクタリング

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

#[derive(PartialEq, Eq)]
pub struct FrequencyEntry {
    value: i32,
    count: i32,
}

impl Ord for FrequencyEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.count
            .cmp(&other.count)
            .then_with(|| self.value.cmp(&other.value))
    }
}

impl PartialOrd for FrequencyEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.count.partial_cmp(&other.count)
    }
}

pub struct Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        if k <= 0 {
            return vec![i32::MIN];
        }

        let k = k as usize;
        let mut frequency_by_value: HashMap<_, _> = HashMap::new();
        let mut top_k_heap: BinaryHeap<Reverse<FrequencyEntry>> = BinaryHeap::new();

        for num in nums {
            frequency_by_value
                .entry(num)
                .and_modify(|frequency| *frequency += 1)
                .or_insert(1);
        }

        for (value, frequency) in frequency_by_value {
            let entry = Reverse(FrequencyEntry {
                value,
                count: frequency,
            });

            if top_k_heap.len() < k {
                top_k_heap.push(entry);
                continue;
            }

            if let Some(Reverse(peeked_entry)) = top_k_heap.peek() {
                if peeked_entry.count < entry.0.count {
                    top_k_heap.pop();
                    top_k_heap.push(entry);
                }
            }
        }

        top_k_heap
            .into_iter()
            .map(|Reverse(entry)| entry.value)
            .collect()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn step1a_test() {
        let mut result = Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2);
        result.sort();
        assert_eq!(result, vec![1, 2]);

        let mut result = Solution::top_k_frequent(vec![1], 1);
        result.sort();
        assert_eq!(result, vec![1]);

        let mut result = Solution::top_k_frequent(vec![1, 2, 1, 2, 1, 2, 3, 1, 3, 2], 2);
        result.sort();
        assert_eq!(result, vec![1, 2]);
    }
}
