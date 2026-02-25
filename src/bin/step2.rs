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
  コメント集、他の人のコードを読んで考えたこと
  https://discord.com/channels/1084280443945353267/1210494002277908491/1211368894669787226
  - Erase-remove idiom　というものが関係あるらしい。
    https://en.wikipedia.org/wiki/Erase%E2%80%93remove_idiom
    - 実装例
      https://cplusplus.com/reference/algorithm/remove/
    C++のstd::removeでは指定された値を取り除くのにポインタによる操作を行っている。
    - 配列先頭のポインタfirst、配列末尾のポインタlast、削除対象の値valを引数に取っている。
    - result=firstとして同じアドレス位置を保持しておく。
    - firstがlastと等しくなるまでループを続ける。
      - firstポインタのアドレス位置に入っている値と、削除対象の値が等しくない時
        - resultとfirstのアドレスが異なるとき
          - resultのアドレス位置にfirstのアドレス位置に入っている値を入れる。
        - resultのアドレス位置を次の位置にインクリメント
      - firstのアドレス位置を次の位置にインクリメント
    - resultは削除対象のvalを除いた配列末尾のアドレスを指すポインタとして返される
    - ForwaredIteratorはConceptというC++の機能らしい。
    https://cpprefjp.github.io/reference/iterator/forward_iterator.html
    このアルゴリズムだと0を一回のループで取り除いた後に、元の配列の長さになるまで0をpush()すれば時間計算量がO(N)になると理解した。

  https://github.com/rihib/leetcode/pull/50#discussion_r1888189547
    > nums[zeroIndex], nums[i] = nums[i], nums[zeroIndex]
    > これ C++ だと zeroIndex == i の場合は未定義動作にあたるかと思いますが、Go では大丈夫でしょうか。
    > この質問は、本当に不安に思っているというよりは、言語仕様を調べたことがありますか、それとも漫然と経験上書いていますか、という質問です。
  自分はこのコードを読んで不安を感じることができなかった。配列から値を取り出して代入しているだけなので大丈夫なのではと感じた。

  https://github.com/fhiyo/leetcode/pull/54#discussion_r1729230640
    > まとめて 0 fill は、loop unrolling できたりするのでちょっと嬉しいこともあるでしょう。
  loop unrollingを初めて聞いた。手作業による書き換えやコンパイラの最適化によって、同じ処理内容ではあるが処理回数を削減すること。
  CPUのレジスタをより多く使う必要がある、展開後のコードの方が長くなるのでコードサイズが増加するなどのトレードオフが発生する。
  https://ja.wikipedia.org/wiki/%E3%83%AB%E3%83%BC%E3%83%97%E5%B1%95%E9%96%8B

  https://github.com/Yoshiki-Iwasa/Arai60/pull/59/changes#diff-8201a9b64da970a353f0eb106023266ce67230b908a5213848bdfa1793d8574c
  - Erase-remove idiomのRust実装例。retain(), resize()メソッドどちらも使ったことがなく新しいメソッドを知れた。

  https://github.com/naoto-iwase/leetcode/pull/55/changes#diff-781326e99985e6db2eab57d073695b257877a3871c5b21a5e284c0dd82038c5dR128
  - 0と0ではない値を入れ替える(swap)実装。この実装を行っている人が多い印象だった。

  改善する時に考えたこと
  - Vec::remove()ではなく、swapによる実装に変更して時間計算量をO(nums.len())に改善する

  所感
  - 少しパズルに感じる。step2a.rsでErase-remove idiomを試してみる。
*/

pub struct Solution {}
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut swap_target_index = 0;

        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums.swap(i, swap_target_index);
                swap_target_index += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step2_test() {
        let mut before_nums = vec![0, 1, 0, 3, 12];
        let after_nums = vec![1, 3, 12, 0, 0];
        Solution::move_zeroes(&mut before_nums);
        assert_eq!(before_nums, after_nums);

        let mut before_nums = vec![0];
        let after_nums = vec![0];
        Solution::move_zeroes(&mut before_nums);
        assert_eq!(before_nums, after_nums);

        let mut before_nums = vec![0, 0, 1];
        let after_nums = vec![1, 0, 0];
        Solution::move_zeroes(&mut before_nums);
        assert_eq!(before_nums, after_nums);
    }
}
