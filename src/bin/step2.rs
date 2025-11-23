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
  - 障害(obstacle)は定数にして明示的になっているのが良いと思った。自分のコードでも定数にする。
  https://github.com/ryosuketc/leetcode_arai60/pull/47/files#diff-5385b925d3a18eebe31d8db86b5496947f1e51242dc517aaa483a3278c7c6c34R20

  - 二次元DPと呼ばれる解法だと思うが、自分がこの解法を実装したことがないのでぱっと見て何をしているのか理解できない。
  写経のみしておく。(step2a_2dp.rs)
  https://github.com/ryosuketc/leetcode_arai60/pull/47/files#diff-1939e3b7a98f60d1d539a0c3835b0aedea5a43e1dc18e5255a438f5162608ea1

  - Rust実装を見ていたらControlFlowというのを始めて見た。
    - breakしたときに値を返せる。ControlFlow::break("breaked.")
    - あまり具体的な用途はすぐに思いつかないが、早期リターンするときの値に意味を持たせられると理解した。
      - 最後まで到達した ControlFlow::Continue()
      - 途中で早期リターンした ControlFlow::Break()
  https://github.com/Yoshiki-Iwasa/Arai60/pull/49/files#diff-2ca67c6aed5a2bebfbd6e13c83b9336f0a682bbed7f61a2daaa7b00bb2b2d276R20
  https://doc.rust-lang.org/std/ops/enum.ControlFlow.html

  - RustでBFS実装があり、実装のバリエーションとして参考になる。
  https://github.com/Yoshiki-Iwasa/Arai60/pull/49/files

  - ミクロな視点での最適化の必要性について。実行速度が常にコードの良さの指標になるわけではないよねという指摘。
  https://github.com/olsen-blue/Arai60/pull/34/files#r1967207778

  - コーディング練習会参加した初期にHashSet型の変数名に*_setみたいな命名をしてハンガリアン記法なのでアンチパターンだよみたいな指摘を受けたが、num_*はハンガリアン記法では無いので大丈夫だななどと考えた。
  ハンガリアン記法にするなら i_paths になると思った。
  https://github.com/ryosuketc/leetcode_arai60/pull/47/files#r2212812620

  - C++において、単に変数を行く裏面とするときは i++ ではなく　++i がGoogle Style Guideで推奨されているという指摘。
  C++は書けないが面白い豆知識だなと思った。いつかC++を書く時に思い出しそう。
  https://github.com/potrue/leetcode/pull/34/files#r2252440982
  https://google.github.io/styleguide/cppguide.html#Preincrement_and_Predecrement
  Google Style GuideにRustはなかったが、GoogleによるRustの学習コースはあった。
  https://google.github.io/comprehensive-rust/

  改善する時に考えたこと
  - 障害(1)をobstacle定数にする
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
    fn step2_test() {
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
    fn step2_broken_grid_test() {
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
    fn step2_panic_test() {
        let obstacle_grid = vec![vec![]];
        Solution::unique_paths_with_obstacles(obstacle_grid);

        let obstacle_grid = vec![vec![], vec![0]];
        Solution::unique_paths_with_obstacles(obstacle_grid);
    }
}
