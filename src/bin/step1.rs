// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  問題の理解
  - m*nの2進行列グリッドが与えられるので、一番大きい面積を持つ島の面積を返す。
  0=水域,1=陸地を表す。
  島の定義
    - 上下左右が水域である。
    - 陸地の上下左右に陸地がある場合、一つの島としてカウントする。（陸地が連続して一つの島となる。）
  [
    [1,1,1],
    [0,1,0],
    [1,0,0]
  ]

  何がわからなかったか
  - 再帰処理によるDFSで解こうと考えた時、スタックオーバーフローにならないかを事前に見積もってみたが、自身を持って見積もりを進められていないと思った。

  何を考えて解いていたか
  - 前回解いた問題（200. Number of Islands）に似ているなと思った。
  - 問題の制約上 1 =< m, n <= 50なので、再帰のDFSで解いた場合、最大スタック深さ50*50=2500となる。
  Rustでのスタックサイズは言語ではなくプラットフォーム依存とあるが、デフォルトでは2MB -> 2048KB -> 2,097,152Bとある。
  https://doc.rust-lang.org/std/thread/#stack-size
  1スタックフレームあたりのサイズがうまく見積もれない。 2,097,152B / 2500 = 839B なので1スタックフレームあたり839Byteまでなら大丈夫そう。
  つまり毎回全てのスタックフレームにgridデータを複製して積むような真似をしなければ大丈夫そう。
  ということで再帰のDFSで書いてみる。(概算が正しいか不安なのであとでChatGPTで確認してみる)
  - 島の数をカウントする処理では島の切れ目が検出できているのでその時点までの陸地の数を数えれば、面積として利用できるはず。
  - 外側で島の面積を入れる最大ヒープを用意して、ヒープにpushしていく。最後に最大ヒープからpopして返す。

  想定ユースケース
  - 地図アプリケーションで付近の施設の特定の指標（島の面積->レビュー数の多さなど）からソート済の参照リストを返すことで、近くの施設を検索する処理に使えそうだと思った。
    - 今回の問題では最大面積の島のサイズしか返していないが島のサイズのリストを全て降順ソートで返すようにするイメージ。
    - この場合はBFSで探索しつつ、ストリーミング処理で現在地から近い順でクライアントに結果を返すなど。

  正解してから気づいたこと
  - 前回の問題（200. Number of Islands）とほぼ同じコードだなと思った。前回の問題で自分が何をしたくて、コードが何をしているのか理解できているので特に苦労すること無く解けたと思った。
  - 再帰処理におけるスタックオーバーフローの見積もりについて。
    - Rustにおけるスタックサイズのデフォルトと書いていたのは正しくなくて、std::thread::spawnしたスレッドにおけるサイズである。
    なので、今回気にするするべきはプラットフォーム(Linux/macOS/Windows)のスタックサイズである。コーディングに利用しているのはmacOSなのでlimitコマンドで確認したところ7MBだった。
  - 再帰処理におけるスタックフレームサイズの見積もり
    - 今回の再帰処理のメソッド内で新たにデータを確保していないので、メソッドのシグネチャーのみからスタックフレームサイズを概算できる。
    以下からスタックフレームあたり24bytesと見積もれると理解した。
      - usizeは8bytesとある。x,yで利用しているので、16bytes。
      https://doc.rust-lang.org/std/primitive.usize.html#:~:text=64%20bit%20target%2C%20this%20is%208%20bytes

      - 殆どのプラットフォームでポインタ型と参照型は大体usizeと類似しているとあるので8bytesとして見積もって問題なさそう。
      https://doc.rust-lang.org/reference/types/pointer.html#:~:text=Despite%20pointers%20and%20references%20being%20similar%20to%20usizes%20in%20the%20machine%20code%20emitted%20on%20most%20platforms%2C

  - 再帰処理を書く場合はスタックオーバーフローを常に意識する必要があるのにも関わらず、なお再帰処理が有効な手法として残っているというのは速度的によほどメリットがあるのだろうかと疑問に思ったのでChatGPTに聞いてみた。
    - 簡単にまとめると以下のような内容だった。
      - 再帰処理は関数呼び出しのオーバーヘッドやスタックオーバーフローなどのデメリットがあるがコールスタック上にスタックフレームを積んでいくのでアドレスの局所性が高くCPUのキャッシュヒット率が高い。
      - ループによる反復実装では関数呼び出しのオーバーヘッドがなくなるが、配列であるため境界チェックやヒープ上にデータを確保するのでアドレスの局所性が下がり、データアクセスレイテンシが数ns~数十ns増加するケースがある。
    - こうやって見ると、速度的な面だけではなくコード記述のシンプルさなども考慮する点に加わってくるので一概にどちらとは言えず、トレードオフの関係なんだなと思った。
      - 再帰処理を書く時はスタックオーバーフローが常に頭をよぎるので、配列による反復処理の方が良いのではと思っていた。
      コールスタック上に連続してスタックフレームを積んでいくというアドレスの局所性によるキャッシュヒット率向上のメリットは知らなかったので勉強になった。
*/

use std::collections::BinaryHeap;

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
                if let Some(v) = Self::get_grid_value(&mut grid, x, y) {
                    if *v == 0 {
                        continue;
                    }

                    let mut island_area = 0;
                    Self::explore_islands(&mut grid, x, y, &mut island_area);

                    if island_area_by_max.peek().is_some_and(|v| *v < island_area) {
                        island_area_by_max.pop();
                        island_area_by_max.push(island_area);
                    }
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
            if let Some(v) = rows.get_mut(x) {
                return Some(v);
            }
        }

        None
    }

    fn explore_islands(grid: &mut Vec<Vec<i32>>, x: usize, y: usize, area_count: &mut i32) {
        if let Some(v) = Self::get_grid_value(grid, x, y) {
            if *v == 0 {
                return;
            }

            *v = 0;
            *area_count += 1;

            if let Some(x) = x.checked_sub(1) {
                Self::explore_islands(grid, x, y, area_count);
            }
            if let Some(y) = y.checked_sub(1) {
                Self::explore_islands(grid, x, y, area_count);
            }
            Self::explore_islands(grid, x + 1, y, area_count);
            Self::explore_islands(grid, x, y + 1, area_count);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step1_test() {
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
    fn step1_empty_grid_test() {
        assert_eq!(Solution::max_area_of_island(vec![]), 0);
    }

    #[test]
    fn step1_full_grid_test() {
        let (m, n) = (50, 50);
        let full_grid = (0..m)
            .map(|_| (0..n).map(|_| 1).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        assert_eq!(Solution::max_area_of_island(full_grid), 2500);
    }
}
