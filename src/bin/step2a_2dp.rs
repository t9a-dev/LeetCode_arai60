// Step2
// 目的: 別の解法を実装する。
// https://github.com/ryosuketc/leetcode_arai60/pull/47/files#diff-1939e3b7a98f60d1d539a0c3835b0aedea5a43e1dc18e5255a438f5162608ea1

/*
    解法の理解
    - 計算した経路を保存する変数を用意する。入力のグリッドと同じサイズで全てのセルを0で初期化しておく。
    - 1行目,1列目を1で初期化していく。この際、対応する座標に障害(1)を見つけたら初期化作業を止める。右と下方向に直進する途中に障害物がありロボットが進めないので。
    - 1行目,1列目以外のセルを計算結果で更新していく。
      - 入力の制約上あり得ないが、入力グリッドのうち途中で断絶している箇所は無視して早期リターンする。
    - 行ごとに列を移動しながら経路を計算している。途中で障害物を見つけたらそれ以上その方向に移動できないのでcontinueで次の行に進む。
    - 右下のセルに結果が書き込まれるので、結果として返す。

    所感
    - 自分で作った異常値テストケースを通すために配列のインデックスを安全な方に倒した実装にした。
    メソッド先頭で入力のobstacle_gridが完全なグリッドであることを検証すれば、インデックスアクセス周りをもう少し簡易的にできるなと思った。
    その場合は、完全なグリッド以外は例外やエラーを返すことになる。そもそも制約外の入力をテストケースにしているので、あまり深追いする必要はない。
    インデックスアクセスを伴う処理は
      - 処理の冒頭で範囲外インデックスアクセスが起こり得ないように入力を厳密にバリデーションする
      - 範囲外インデックスアクセスが発生してもパニックしないようにエラーハンドリングする
    のどちらかになりそうだが、個人的にはインデックスアクセス自体を安全側に倒したいなという感覚がある。配列の添字で直接インデックスアクセスしていると少し不安になる感覚がある。
    - 前の問題(62.Unique Paths)で利用した1次元配列を更新していく方法を変形させて解けそうだが時間切れなのでスキップ
*/

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

        let (num_rows, num_cols) = (obstacle_grid.len(), obstacle_grid[0].len());
        let mut num_paths: Vec<Vec<i32>> = vec![vec![0; num_cols]; num_rows];

        // First Row
        for num_col in 0..num_cols {
            let Some(grid_value) = obstacle_grid.get(0).and_then(|cols| cols.get(num_col)) else {
                continue;
            };

            if *grid_value == Self::OBSTACLE {
                break;
            }

            num_paths[0][num_col] = 1;
        }

        // First Colmn
        for num_row in 0..num_rows {
            let Some(grid_value) = obstacle_grid.get(num_row).and_then(|cols| cols.get(0)) else {
                continue;
            };

            if *grid_value == Self::OBSTACLE {
                break;
            }

            num_paths[num_row][0] = 1;
        }

        for num_row in 1..num_rows {
            for num_col in 1..num_cols {
                let Some(grid_value) = obstacle_grid
                    .get(num_row)
                    .and_then(|cols| cols.get(num_col))
                else {
                    return num_paths[num_rows - 1][num_cols - 1];
                };

                if *grid_value == Self::OBSTACLE {
                    continue;
                }

                num_paths[num_row][num_col] =
                    num_paths[num_row - 1][num_col] + num_paths[num_row][num_col - 1];
            }
        }

        num_paths[num_rows - 1][num_cols - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step2a_2dp_test() {
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
    fn step2a_2dp_broken_grid_test() {
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
    fn step2a_2dp_panic_test() {
        let obstacle_grid = vec![vec![]];
        Solution::unique_paths_with_obstacles(obstacle_grid);

        let obstacle_grid = vec![vec![], vec![0]];
        Solution::unique_paths_with_obstacles(obstacle_grid);
    }
}
