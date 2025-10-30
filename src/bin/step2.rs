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
  講師陣はどのようなコメントを残すだろうか？
  - gridの中身をenumに変換してから扱うのは過剰だと言われるかも。

  他の人のコードを読んで考えたこと
  - UnionFindとう単語を初めて聞いた。今回の問題のように集合（連続した陸地の集合を島として扱う）を扱うアルゴリズムの名称であることが分かった。
  https://github.com/nanae772/leetcode-arai60/pull/18/files#diff-5296083e07820ef771afd378e2e793cbd329911a7e343fbd38bb1272007f0fcaR10
  https://ja.wikipedia.org/wiki/%E7%B4%A0%E9%9B%86%E5%90%88%E3%83%87%E3%83%BC%E3%82%BF%E6%A7%8B%E9%80%A0

  - グローバル変数で状態を扱うことに対する反応について。
  プログラミングコンテスト文脈でジャッジシステムにより一度しか呼ばれない想定であれば問題ないがそういった制約下でしか使えないコードであることを理解できているかという話だと理解した。
  https://github.com/5103246/LeetCode_Arai60/pull/17/files#r2386269357

  - 上記の関連として標準出入力、関数呼出しによるオーバーヘッドの見積もりについて。
  関数呼出しによるオーバーヘッドは変数などをスタックフレームに格納して、関数呼出しが終わったらスタックフレームを復元するといった手順があるのでオーバーヘッドになる感覚はあった。
  しかし、標準入出力を利用した場合についてプロセスの立ち上げに具体的にどの程度時間がかかるのかといったことは考えたことがなかったし、答えられる知識を持っていないと思った。
  実行環境のCPUクロック数にもよるが、具体的な数値で関数呼び出しで1ns前後、標準入出力ではプロセスの立ち上げに10msといった感じでは思いつかないなと思った。
  https://github.com/irohafternoon/LeetCode/pull/1#discussion_r1996961516

  - 自分もBFSを実装するときにキューをLIFOのスタックで間違えて書いたものの動いたので不思議に思ってChatGPTを利用してレビューしたところ、DFSになっていただけだった。
  step1でDFSを再帰処理で実装していたが、再帰処理はスタックにスタックフレームを積んでいってLIFOとなるので、こうなるのかと理解した。
  DFS、BFSどちらを採用するかの基準として思いつくのは、深い階層まで必ず見に行かなくても浅い階層で早期リターン可能な問題ではBFSとかだろうか。
  BFS,DFSそれぞれどういう順番で探索するのか、対象の問題がどういう性質であるかを理解することが大切だなと思った。
  https://github.com/docto-rin/leetcode/pull/17/files#diff-5c15b5a457745340b0829a41cc85d0ec21654a482447ccb1facec02bdcd5e432R359

  - 自分もよくやりそうになる。ただし現状はコードに書き下せないので、繰り返し書くことすらできないといった違いはある。
  この問題は特に手作業でどうやるかは説明できるものの、自然言語で簡潔に表現し、さらにコードにする部分までのそれぞれのステップで距離があるような感覚だった。
  良い言語化だなと思った。
  https://github.com/yas-2023/leetcode_arai60/pull/17#discussion_r2432409292

  改善する時に考えたこと
  - step1では到達済みの座標の値を直接'0'で書き換えることで、チェック済の座標を管理していたが、HashSetでチェック済の座標を管理するのもありだと思った。
  この場合はO(m*n)の空間計算量が追加で必要になる。
  読み手にわかりやすくするならenumで'0'->water,'1'->land,visitedとした方が良いだろうか。この場合は最初に一度全走査してデータを変換するので時間計算量O(n*m)増加する。
  HashSetで訪問済みを管理するよりはenumの方が可読性が上がるという点でのメリットは大きそう。特に一度到達済の陸地を'0'として水域にするのは読み手に考えさせそう。
  - データの中身をcharからenumに置き換えるのをin-placeでやると良くないかなと思ったが、num_islandsメソッド引数のシグネチャーが&mutなどの参照ではないので気にする必要がない。
  具体的には呼び出し側はgrid変数の所有権をnum_islandsメソッドにmoveするので、そもそも呼び出し側でgridを扱えなくなる。
  - gridから特定の座標の値にアクセスする処理をヘルパー関数に切り出した。一度grid[y][x]とすべきところを、grid[x][y]として間違えたので。
  - 構造体にしてフィールドとメソッドを定義しようかと思ったが、xとyはループで全走査する変数なので、フィールドのメンバにして取り回しが良くなるような性質のものではないため止めた。
*/

#[derive(PartialEq, Eq)]
enum Location {
    Water,
    Land,
    Visited,
}

pub struct Solution {}
impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut lands_count = 0;
        let mut grid = Self::character_to_location(grid);

        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if let Some(location) = Self::get_location(&mut grid, x, y) {
                    match location {
                        Location::Visited | Location::Water => continue,
                        _ => (),
                    };

                    Self::explore_islands(&mut grid, x, y);
                    lands_count += 1;
                }
            }
        }

        lands_count
    }

    fn character_to_location(grid: Vec<Vec<char>>) -> Vec<Vec<Location>> {
        grid.into_iter()
            .map(|rows| {
                rows.into_iter()
                    .filter_map(|v| match v {
                        '0' => Some(Location::Water),
                        '1' => Some(Location::Land),
                        _ => None,
                    })
                    .collect()
            })
            .collect::<Vec<Vec<Location>>>()
    }

    fn get_location(grid: &mut Vec<Vec<Location>>, x: usize, y: usize) -> Option<&mut Location> {
        if let Some(rows) = grid.get_mut(y) {
            if let Some(v) = rows.get_mut(x) {
                return Some(v);
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
    fn step2_test() {
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
