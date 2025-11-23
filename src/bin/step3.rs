// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  n = obstacle_grid.len()
  m = obstacle_grid[0].len()
  時間計算量: O(n * m)
  空間計算量: O(n * m)
*/

/*
  1回目: 8分41秒
  2回目: 8分19秒
  3回目: 分秒 <- 長いのでスキップ
*/

/*
  所感
  - 一定のルールに沿って自動的に探索していくところがアルゴリズムという感じで好きな解法
*/

use std::collections::HashMap;

pub struct Solution {}
impl Solution {
    const OBSTACLE: i32 = 1;

    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        if obstacle_grid.len() == 0 || obstacle_grid[0].len() == 0 {
            panic!(
                "obstacle_grid must be grid. obstacle_grid: {:#?}",
                obstacle_grid
            );
        }

        let mut paths_cache: HashMap<(usize, usize), i32> = HashMap::new();
        Self::explore_unique_paths(
            obstacle_grid.len() - 1,
            obstacle_grid[0].len() - 1,
            &obstacle_grid,
            &mut paths_cache,
        )
    }

    fn explore_unique_paths(
        num_row: usize,
        num_col: usize,
        obstacle_grid: &Vec<Vec<i32>>,
        paths_cache: &mut HashMap<(usize, usize), i32>,
    ) -> i32 {
        let Some(grid_value) = obstacle_grid
            .get(num_row)
            .and_then(|cols| cols.get(num_col))
        else {
            return 0;
        };

        if *grid_value == Self::OBSTACLE {
            return 0;
        }

        if num_row == 0 && num_col == 0 {
            return 1;
        }

        let to_left_path = match num_col.checked_sub(1) {
            Some(num_col) => match paths_cache.get(&(num_row, num_col)) {
                Some(to_left_cache) => *to_left_cache,
                None => {
                    let to_left_path =
                        Self::explore_unique_paths(num_row, num_col, obstacle_grid, paths_cache);
                    paths_cache.insert((num_row, num_col), to_left_path);
                    to_left_path
                }
            },
            None => 0,
        };
        let to_up_path = match num_row.checked_sub(1) {
            Some(num_row) => match paths_cache.get(&(num_row, num_col)) {
                Some(to_up_cache) => *to_up_cache,
                None => {
                    let to_up_path =
                        Self::explore_unique_paths(num_row, num_col, obstacle_grid, paths_cache);
                    paths_cache.insert((num_row, num_col), to_up_path);
                    to_up_path
                }
            },
            None => 0,
        };

        to_left_path + to_up_path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step3_test() {
        let obstacle_grid = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
        let unique_paths = Solution::unique_paths_with_obstacles(obstacle_grid);
        assert_eq!(unique_paths, 2);

        let obstacle_grid = vec![vec![0, 1], vec![0, 0]];
        let unique_paths = Solution::unique_paths_with_obstacles(obstacle_grid);
        assert_eq!(unique_paths, 1);

        let obstacle_grid = vec![vec![0, 1], vec![1, 0]];
        let unique_paths = Solution::unique_paths_with_obstacles(obstacle_grid);
        assert_eq!(unique_paths, 0);

        let obstacle_grid = vec![vec![0, 0, 1], vec![0, 0, 0], vec![0, 0, 1]];
        let unique_paths = Solution::unique_paths_with_obstacles(obstacle_grid);
        assert_eq!(unique_paths, 0);

        let obstacle_grid = vec![vec![1]];
        let unique_paths = Solution::unique_paths_with_obstacles(obstacle_grid);
        assert_eq!(unique_paths, 0);
    }

    #[test]
    fn step3_broken_grid_test() {
        let obstacle_grid = vec![vec![0, 0, 0], vec![], vec![0, 0, 0], vec![0, 0, 0]];
        let unique_paths = Solution::unique_paths_with_obstacles(obstacle_grid);
        assert_eq!(unique_paths, 0);

        let obstacle_grid = vec![vec![0, 0, 0], vec![0, 0, 0], vec![], vec![0, 0, 0]];
        let unique_paths = Solution::unique_paths_with_obstacles(obstacle_grid);
        assert_eq!(unique_paths, 0);

        let obstacle_grid = vec![vec![0], vec![0], vec![0, 0, 0]];
        let unique_paths = Solution::unique_paths_with_obstacles(obstacle_grid);
        assert_eq!(unique_paths, 1);
    }

    #[test]
    #[should_panic]
    fn step3_panic_test() {
        let obstacle_grid = vec![vec![]];
        Solution::unique_paths_with_obstacles(obstacle_grid);

        let obstacle_grid = vec![vec![], vec![0]];
        Solution::unique_paths_with_obstacles(obstacle_grid);
    }
}
