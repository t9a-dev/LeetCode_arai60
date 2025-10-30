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
  時間計算量: O(n * m) グリッド全走査している
  空間計算量: O(n * m) グリッド全走査しながらスタックフレームをスタックに積んでいる
*/

/*
  1回目: 9分57秒
  2回目: 8分52秒
  3回目: 9分50秒
  4回目: 8分10秒 <- 毎回ギリギリなので一応4回目
*/

/*
  感想:
  - if x > 0 , if y > 0 のindexの境界値チェックがもう少しどうにかならないかなと思って調べたが、記述量が増える方向の選択肢しかなかったので今回は見送った。
  ↑x,yがusizeなので負数になると実行時エラーになるので境界値チェックしている
    - check_sub(1)でマイナス値にならなかった場合にSome(x)で結果が帰ってくるが、記述量が増えるだけな感じがする。
    - as isizeで扱う方法もあるが、as やtry_into()でキャストする箇所が増えたりするとミスが増えそう。
  - explore_islandsのmatch文の中で *location = Location::Visited;するか迷ったが、ガード節に近いのはmatch外側で代入する方だなと思った。
*/

enum Location {
    Water,
    Land,
    Visited,
}

pub struct Solution {}
impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut islands_count = 0;
        let mut grid = Self::character_to_location(grid);

        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if let Some(location) = Self::get_location(&mut grid, x, y) {
                    match location {
                        Location::Visited | Location::Water => continue,
                        _ => (),
                    }

                    Self::explore_islands(&mut grid, x, y);
                    islands_count += 1;
                }
            }
        }

        islands_count
    }

    fn character_to_location(grid: Vec<Vec<char>>) -> Vec<Vec<Location>> {
        grid.into_iter()
            .map(|rows| {
                rows.into_iter()
                    .filter_map(|location| match location {
                        '0' => Some(Location::Water),
                        '1' => Some(Location::Land),
                        _ => None,
                    })
                    .collect()
            })
            .collect()
    }

    fn get_location(grid: &mut Vec<Vec<Location>>, x: usize, y: usize) -> Option<&mut Location> {
        if let Some(rows) = grid.get_mut(y) {
            if let Some(location) = rows.get_mut(x) {
                return Some(location);
            }
        }

        None
    }

    fn explore_islands(grid: &mut Vec<Vec<Location>>, x: usize, y: usize) {
        if let Some(location) = Self::get_location(grid, x, y) {
            match location {
                Location::Visited | Location::Water => return,
                _ => (),
            }

            *location = Location::Visited;

            if x > 0 {
                Self::explore_islands(grid, x - 1, y);
            }
            if y > 0 {
                Self::explore_islands(grid, x, y - 1);
            }
            Self::explore_islands(grid, x + 1, y);
            Self::explore_islands(grid, x, y + 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step3_test() {
        let grid = vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ];
        assert_eq!(Solution::num_islands(grid), 1);

        let grid = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ];
        assert_eq!(Solution::num_islands(grid), 3);

        let grid = vec![
            vec!['1', '1', '1'],
            vec!['0', '1', '0'],
            vec!['1', '1', '1'],
        ];
        assert_eq!(Solution::num_islands(grid), 1);

        let grid = vec![
            vec!['0', '0', '1', '1', '1'],
            vec!['0', '1', '1', '1', '1'],
            vec!['0', '1', '1', '0', '1'],
            vec!['1', '1', '1', '1', '1'],
            vec!['1', '1', '1', '0', '0'],
            vec!['1', '1', '1', '0', '0'],
            vec!['1', '0', '1', '0', '0'],
            vec!['1', '1', '1', '0', '0'],
        ];
        assert_eq!(Solution::num_islands(grid), 1);
    }
}
