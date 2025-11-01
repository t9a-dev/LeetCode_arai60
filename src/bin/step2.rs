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
  - UnionFindによる解法。Arai60完走を優先したいのでスキップ。
  https://github.com/h1rosaka/arai60/pull/22/files#diff-8a19c9138ddde283b493e7deebbaacc7836f60327850295f22bcec1f0de00b28R107

  - 再帰関数での戻り値で島の面積を返す例。
  この方法は実装しようと思った記憶があるが、引数でインクリメントを管理する変数を渡す方式にしたので参考になると思った。
  試そうかと思ったがこの方法だとrow,colをisizeにして、事前に境界値チェックする必要があるので、変更に見合う効果は得られないと思ったので止めた。
  https://github.com/Kazuryu0907/LeetCode_Arai60/pull/12#discussion_r2347162514

  - 上下左右の移動をoffsetsとして定義しておく実装例。宣言的でわかりやすく良いと思った。
  境界値チェックはどうしてるかなと思ったら、先頭のガード節で行われていた。
  自分の実装は座標を減産するときにインデックスの境界値チェックしているが、メソッド先頭でやるのもありだなと思った。
  少し考えたが、個人的な好みとしては境界値チェックはどこでやってるかなと思ってコードを見に行く必要があるのと、[row][col]のようなアクセスを見ると範囲外アクセスの不安を感じるので、インデックスを減算する箇所の近くでチェックするかなと思った。
  https://github.com/nanae772/leetcode-arai60/pull/19/files/4ed0458b866226b7a6f83afb955864d46edba225#diff-e58eef20e7d713ad587cc2cffec4efb58927cb748118efa673b09a01ce79c8b3R20

  - Pythonには二重ループを浅くするitertools.productなるものがあるそう。
  https://github.com/docto-rin/leetcode/pull/18/files#r2425068670
    - Rustだと外部クレートitertoolsのiproductで実現できる。

  改善する時に考えたこと
  - 変数名x,yではなくrow,colでやってみる。
  https://github.com/t9a-dev/LeetCode_arai60/pull/17#discussion_r2477040013

  - enumで0->Water,1->Land,Visitedを表す。
*/

use std::collections::BinaryHeap;

enum Location {
    Water,
    Land,
    Visited,
}

pub struct Solution {}
impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = Self::num_to_location(grid);
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

    fn num_to_location(grid: Vec<Vec<i32>>) -> Vec<Vec<Location>> {
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

    fn get_location(
        grid: &mut Vec<Vec<Location>>,
        row: usize,
        col: usize,
    ) -> Option<&mut Location> {
        if let Some(row) = grid.get_mut(row) {
            return row.get_mut(col);
        }

        None
    }

    fn explore_island(
        grid: &mut Vec<Vec<Location>>,
        row: usize,
        col: usize,
        island_area: &mut i32,
    ) {
        if let Some(location) = Self::get_location(grid, row, col) {
            match location {
                Location::Visited | Location::Water => return,
                _ => (),
            }

            *location = Location::Visited;
            *island_area += 1;

            if let Some(col) = col.checked_sub(1) {
                Self::explore_island(grid, row, col, island_area);
            }
            if let Some(row) = row.checked_sub(1) {
                Self::explore_island(grid, row, col, island_area);
            }
            Self::explore_island(grid, row + 1, col, island_area);
            Self::explore_island(grid, row, col + 1, island_area);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step2_test() {
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
    fn step2_empty_grid_test() {
        assert_eq!(Solution::max_area_of_island(vec![]), 0);
    }

    #[test]
    fn step2_full_grid_test() {
        let (m, n) = (50, 50);
        let full_grid = (0..m)
            .map(|_| (0..n).map(|_| 1).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        assert_eq!(Solution::max_area_of_island(full_grid), 2500);
    }
}
