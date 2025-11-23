// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  問題の理解
  - m * nのグリッドを表すobstacle_gridが与えられる。ロボットがgridの左上(0,0)に配置される。ロボットが右下の座標に行く一意の経路の総数を返す。
  ロボットは下または右方向にしか移動できない。
  obstacle_gridではロボットが移動できるスペースが0、ロボットが移動できない障害物が1として表される。
  障害部を含む経路は移動できないので、経路として数えない。

  何がわからなかったか
  - テストケースobstacle_grid = [[1]] のケースを見逃しており、1回Wrong Answerとなった。
  基本ケースの順番を入れ替えて対応。グリッドに障害物(1)があるかを確認した後に、ゴール到達判定を行うように修正。
  先にテストコードを書いていたもので、このテストケースは思いつきたかったなと思った。

  何を考えて解いていたか
  - 前の問題(62.Unique Paths)で利用した再帰DFSの解法に少し変更を加えれば解けそう。
  具体的には今見ているobstacle_grig[row][col]が1であれば、基本ケースでreturn 0としてしまう。

  正解してから気づいたこと
  - explore_unique_pathsの呼び出しで0-based indexingな値に調整しているところと、explore_unique_pathsの引数が4つなのが気になった。
  しかし、外部に公開するAPIとしてはunique_paths_with_obstaclesなので、利用側からは0-based indexingかどうかは気にしなくて良い。
  explore_unique_pathsの引数が4つであることも上記の理由から構造体にまとめなくても良いかと考えた。
  - obstacle_gridの異常値について、4行あるうちの2行目に列を持たない空の配列が含まれるケースを考えたときに、グリッドが断絶しているのでゴールに到達しないということで0を返すので良いかなどと考えた。
  - 解ける！と思ったのですぐに実装に取り掛かってしまったが、再帰を利用するのであればスタックオーバーフローを入力のサイズと実装するロジックから見積もるべきだった。
  一度通ったパスはキャッシュしているので
  n = obstacle_grid.len()
  m = obstacle_grid[0].len()
  時間計算量: O(n * m)
  空間計算量: O(n * m)

  再帰呼び出しの回数は n + mとなる。nとmは再帰呼び出し中に-1ずつ減少するので。
  スタックサイズは (n + m) * スタックフレームのサイズ　となる。入力の制約は 1 <= n, m <= 100。
  1スタックフレームあたりのサイズは、8byteのメソッド引数4つと再帰関数内の変数などから多めに見積もって70byteとする。64bitプラットフォームにおいて、参照は8byteとして見積もる。usizeもプラットフォームに依存するので8byteと見積もる。
  (100 + 100) * 70byte = 14,000byte となりスタックオーバーフローを心配するようなサイズにならないことが分かる
*/

use std::collections::HashMap;

pub struct Solution {}
impl Solution {
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
        row_num: usize,
        col_num: usize,
        obstacle_grid: &Vec<Vec<i32>>,
        paths_cache: &mut HashMap<(usize, usize), i32>,
    ) -> i32 {
        let Some(grid_value) = obstacle_grid
            .get(row_num)
            .and_then(|cols| cols.get(col_num))
        else {
            return 0;
        };

        if *grid_value == 1 {
            return 0;
        }

        if row_num == 0 && col_num == 0 {
            return 1;
        }

        let to_left_path = match col_num.checked_sub(1) {
            Some(col_num) => match paths_cache.get(&(row_num, col_num)) {
                Some(to_left_cache) => *to_left_cache,
                None => {
                    let to_left_path =
                        Self::explore_unique_paths(row_num, col_num, obstacle_grid, paths_cache);
                    paths_cache.insert((row_num, col_num), to_left_path);
                    to_left_path
                }
            },
            None => 0,
        };

        let to_up_path = match row_num.checked_sub(1) {
            Some(row_num) => match paths_cache.get(&(row_num, col_num)) {
                Some(to_up_cache) => *to_up_cache,
                None => {
                    let to_up_path =
                        Self::explore_unique_paths(row_num, col_num, obstacle_grid, paths_cache);
                    paths_cache.insert((row_num, col_num), to_up_path);
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
    fn step1_test() {
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
    fn step1_broken_grid_test() {
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
    fn step1_panic_test() {
        let obstacle_grid = vec![vec![]];
        Solution::unique_paths_with_obstacles(obstacle_grid);

        let obstacle_grid = vec![vec![], vec![0]];
        Solution::unique_paths_with_obstacles(obstacle_grid);
    }
}
