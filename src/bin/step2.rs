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
  https://discord.com/channels/1084280443945353267/1233603535862628432/1238727359280971796
  https://discord.com/channels/1084280443945353267/1233603535862628432/1238845284818092113
  - スタックを使って書く実装。木構造を扱うイメージなのでBFS,DFSのアプローチがある。

  https://github.com/Yoshiki-Iwasa/Arai60/pull/55/changes#r1738806743
  - heapを使った解法もあるみたい。処理自体はそこまで複雑ではないが、なぜこうしているのかみたいな部分が分からず、時間もかかりそうなのでスキップ

  https://github.com/Yoshiki-Iwasa/Arai60/pull/55/changes#diff-bca26255c140c7d5182e99d56cc77a000b943c6a2361201ff20f873ab6c09042R107
  - swapを使った解法。step1aでキューからpopする操作と、popした値を末尾にpushする操作がswapになっている。
  step1aと比べて再帰処理でVecではなく、インデックスを取り回すので省メモリになると理解。

  改善する時に考えたこと
  - 再帰処理でVecを取り回すのではなく、インデックスを取り回すようにする。

  所感
  - ある配列の特定の値や、特定の範囲の値をそのままVecで取り回すような解法を見た時に、Vecそのものではなく、ポインタを取り回すという解法にまだ気付けていない感じがある。
  - この解法では再帰処理で順列の配列を取り回す必要が無く、Vecを毎回クローンする罪悪感を感じないのが良いと思った。
  step1aと比べても、値を入れ替える部分についてはそこまで可読性は落ちておらず許容範囲だと感じる。
  - step1aで参考にした解法では、Vec::removeを利用しており、最悪ケースで時間計算量がO(n)になるのでVecDeque::pop_frontを利用する形にしていた。
  https://doc.rust-lang.org/std/vec/struct.Vec.html#method.remove
  そもそも値を入れ替えるという操作に注目するとswapが利用できるという点に気が付かなかった。
*/

pub struct Solution {}
impl Solution {
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut permutations = Vec::new();
        Self::make_permutations(&mut nums, 0, &mut permutations);

        permutations
    }

    fn make_permutations(nums: &mut [i32], swap_index: usize, permutations: &mut Vec<Vec<i32>>) {
        if nums.len() == swap_index {
            permutations.push(nums.to_vec());
            return;
        }

        for i in swap_index..nums.len() {
            nums.swap(swap_index, i);
            Self::make_permutations(nums, swap_index + 1, permutations);
            nums.swap(swap_index, i);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step2_test() {
        let mut expect = vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1],
        ];
        let mut actual = Solution::permute(vec![1, 2, 3]);
        expect.sort();
        actual.sort();
        assert_eq!(expect, actual);

        assert_eq!(
            Solution::permute(vec![0, 1]).sort(),
            vec![[0, 1], [1, 0]].sort()
        );
        assert_eq!(Solution::permute(vec![1]), vec![[1]]);
    }
}
