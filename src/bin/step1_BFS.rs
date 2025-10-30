// Step1
// 目的: BFS(Breadth-First Search,幅優先探索)実装をしてみる

/*
ChatGPTによるとこの問題は幅優先探索でも実装できるそうなので実装だけしてみる。
  具体的な実装イメージが浮かばないので写経と自分なりに読みやすいように変更する。

  VecDequeは初めて使った。
  Vecでもremove()メソッドにより、先頭要素の取り出しが可能だがデータ構造の制約上O(n)となるので、VecDeque.front_popを利用している。
  先頭要素の取り出しを行いたい場合はVecDequeを使うようにドキュメントに書かれている。
  https://doc.rust-lang.org/std/vec/struct.Vec.html#method.remove

  VecDequeはFIFOを実現したい場合に利用する。Vecでは時間計算量O(n)となる先頭要素の取り出しがO(1)で行える。
  https://doc.rust-lang.org/std/collections/struct.VecDeque.html

  リングバッファという配列構造になっている。
  https://ja.wikipedia.org/wiki/%E3%83%AA%E3%83%B3%E3%82%B0%E3%83%90%E3%83%83%E3%83%95%E3%82%A1

  所感
  - exploreメソッドの引数が4つになっているのが気になる。改善するのであれば構造体にまとめるか、構造体のフィールドで値を取り回す方式が考えられると思った。
  - BFSは層ごとに探索していくとのことで、このプログラムを見たときに以下のような疑問が浮かんだ。
    - 層ごとに探索するのであれば、一度探索した層は探索する必要がないので、y-1のように一つ前の層に戻るような移動は不要では？
      - 層の概念の理解が間違っており、行(y)!=BFSにおける層である。
      - BFSにおける層はスタート座標からの距離であるという理解が正しそう。
      - スタート座標を5とする。
        - 層0: 5
        - 層1: 2,6,8,4 上下左右でスタート座標からの距離が1 ここの座標がvisited_locationにpush_backされて、次の層でpop_frontで取り出して原点として扱われる。
        - 層2: 1,3,9,7 層0からは距離が2 層1の座標から見たときに上下左右の座標
        x
      y[1,2,3]
       [4,5,6]
       [7,8,9]
      - 層2から確認する座標に重複が出てくるので、実装では訪問済みを0にして、訪問済みをスキップしている。
*/

use std::collections::VecDeque;

pub struct Solution {}
impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut islands_count = 0;
        let mut grid = grid;

        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if grid[y][x] == '1' {
                    grid[y][x] = '0';
                    Self::explore_islands(&mut grid, x, y);
                    islands_count += 1;
                }
            }
        }
        islands_count
    }

    fn explore_islands(grid: &mut Vec<Vec<char>>, start_x: usize, start_y: usize) {
        let mut visited_locations: VecDeque<(usize, usize)> = VecDeque::new();
        visited_locations.push_back((start_x, start_y));

        while let Some((x, y)) = visited_locations.pop_front() {
            if y > 0 {
                Self::explore(grid, x, y - 1, &mut visited_locations);
            }
            if x > 0 {
                Self::explore(grid, x - 1, y, &mut visited_locations);
            }
            Self::explore(grid, x + 1, y, &mut visited_locations);
            Self::explore(grid, x, y + 1, &mut visited_locations);
        }
    }

    fn explore(
        grid: &mut Vec<Vec<char>>,
        x: usize,
        y: usize,
        visited_locations: &mut VecDeque<(usize, usize)>,
    ) {
        if let Some(rows) = grid.get_mut(y) {
            if let Some(v) = rows.get_mut(x) {
                if *v == '0' {
                    return;
                }

                *v = '0';
                visited_locations.push_back((x, y));
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
