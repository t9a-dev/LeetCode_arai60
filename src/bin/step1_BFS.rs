// Step1_BFS
// 目的: 練習のためBFSでも実装する

use std::collections::{BinaryHeap, VecDeque};

pub struct Solution {}
impl Solution {
    /*
      メソッドのシグネチャーからgridは所有権を要求する。呼び出し側でこのメソッド呼び出し以降はgridを利用できない。
      つまり、このメソッド内でgridに破壊的変更を加えても呼び出し側に影響を与えない。
    */
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        // メソッドのシグネチャーを変更してmut gridにすれば次の行は不要になるが、LeetCodeで用意されているテンプレートのシグネチャーに従っている。
        let mut grid = grid;
        let mut island_area_by_max: BinaryHeap<i32> = BinaryHeap::new();
        let dummy = 0;

        island_area_by_max.push(dummy);

        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                let mut area_count = 0;
                Self::explore_islands_area(&mut grid, x, y, &mut area_count);

                if island_area_by_max.peek().is_some_and(|v| *v < area_count) {
                    island_area_by_max.pop();
                    island_area_by_max.push(area_count);
                }
            }
        }

        match island_area_by_max.pop() {
            Some(v) => v,
            None => 0,
        }
    }

    fn get_grid_value(grid: &mut Vec<Vec<i32>>, x: usize, y: usize) -> Option<&mut i32> {
        if let Some(rows) = grid.get_mut(y) {
            return rows.get_mut(x);
        }

        None
    }

    fn explore_islands_area(
        grid: &mut Vec<Vec<i32>>,
        start_x: usize,
        start_y: usize,
        area_count: &mut i32,
    ) {
        let mut visited_positions = VecDeque::new();
        visited_positions.push_back((start_x, start_y));

        while let Some((x, y)) = visited_positions.pop_front() {
            if let Some(v) = Self::get_grid_value(grid, x, y) {
                if *v == 0 {
                    continue;
                }

                *v = 0;
                *area_count += 1;

                if let Some(x) = x.checked_sub(1) {
                    visited_positions.push_back((x, y));
                }
                if let Some(y) = y.checked_sub(1) {
                    visited_positions.push_back((x, y));
                }
                visited_positions.push_back((x + 1, y));
                visited_positions.push_back((x, y + 1));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step1_bfs_test() {
        let grid = vec![
            vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
        ];
        assert_eq!(Solution::max_area_of_island(grid), 6);

        let grid = vec![vec![1], vec![0], vec![0, 1, 1]];
        assert_eq!(Solution::max_area_of_island(grid), 2);

        assert_eq!(Solution::max_area_of_island(vec![vec![1]]), 1);
    }

    #[test]
    fn step1_bfs_empty_grid_test() {
        assert_eq!(Solution::max_area_of_island(vec![]), 0);
    }

    #[test]
    fn step1_bfs_full_grid_test() {
        let (m, n) = (50, 50);
        let full_grid = (0..m)
            .map(|_| (0..n).map(|_| 1).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        assert_eq!(Solution::max_area_of_island(full_grid), 2500);
    }
}
