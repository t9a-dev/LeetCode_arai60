// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  n = grid.len()
  m = grid[n].len()
  時間計算量: O(n * m)
  空間計算量: O(n * m)
*/

/*
  1回目: 10分42秒
  2回目: 9分2秒
  3回目: 9分33秒
  4回目: 9分4秒
*/

/*
  所感
  - 繰り返し書いている中で気づいたがgridから行と列を指定して値を取得するメソッドは一度しか呼ばれておらず、やっていることは配列の境界チェックなので削った。
*/

use std::collections::BinaryHeap;

enum Location {
    Water,
    Land,
    Visited,
}

struct Solution;
impl Solution {
    /*
      メソッドのシグネチャーからgridは所有権を要求する。呼び出し側でこのメソッド呼び出し以降はgridを利用できない。
      つまり、このメソッド内でgridに破壊的変更を加えても呼び出し側に影響を与えない。
    */
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = Self::number_to_location(grid);
        let mut island_area_by_max: BinaryHeap<i32> = BinaryHeap::new();
        let dummy = 0;
        island_area_by_max.push(dummy);

        for row in 0..grid.len() {
            for col in 0..grid[row].len() {
                let mut island_area = 0;
                Self::explore_island(&mut grid, row, col, &mut island_area);

                if island_area_by_max
                    .peek()
                    .is_some_and(|peeked_area| *peeked_area < island_area)
                {
                    island_area_by_max.pop();
                    island_area_by_max.push(island_area);
                }
            }
        }

        match island_area_by_max.pop() {
            Some(v) => v,
            None => 0,
        }
    }

    fn number_to_location(grid: Vec<Vec<i32>>) -> Vec<Vec<Location>> {
        grid.into_iter()
            .map(|row| {
                row.into_iter()
                    .filter_map(|col| match col {
                        0 => Some(Location::Water),
                        1 => Some(Location::Land),
                        _ => None,
                    })
                    .collect()
            })
            .collect()
    }

    fn explore_island(
        grid: &mut Vec<Vec<Location>>,
        row: usize,
        col: usize,
        island_area: &mut i32,
    ) {
        if grid.get(row).is_none() || grid[row].get(col).is_none() {
            return;
        }

        let location = &mut grid[row][col];

        match location {
            Location::Visited | Location::Water => return,
            _ => (),
        }

        *location = Location::Visited;
        *island_area += 1;

        if let Some(row) = row.checked_sub(1) {
            Self::explore_island(grid, row, col, island_area);
        }
        if let Some(col) = col.checked_sub(1) {
            Self::explore_island(grid, row, col, island_area);
        }
        Self::explore_island(grid, row + 1, col, island_area);
        Self::explore_island(grid, row, col + 1, island_area);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn step3_test() {
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
    fn step3_empty_grid_test() {
        assert_eq!(Solution::max_area_of_island(vec![]), 0);
    }

    #[test]
    fn step3_full_grid_test() {
        let (m, n) = (50, 50);
        let full_grid = (0..m)
            .map(|_| (0..n).map(|_| 1).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        assert_eq!(Solution::max_area_of_island(full_grid), 2500);
    }
}
