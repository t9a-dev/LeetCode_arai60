// Step1b
// 目的: 二分探索を利用した時間計算量の削減をしている実装の理解

/*
  問題の理解
  - 整数を持つ配列numsが与えられる。numsの並び順を変えずに厳密に増加する部分列を作ったときに最長となる部分列の長さを返す。
  case1: nums=[1,0,2,3] subsequence=[1,2,3] return=3
  case2: nums=[0,1,0,3,2,3] subsequence=[0,1,2,3] return=4
  case3: nums=[3,3,3,3] subsequence=[3] return=1

  参考
  Rust実装
    https://github.com/Yoshiki-Iwasa/Arai60/pull/46/files
  Python実装
    https://github.com/fhiyo/leetcode/pull/32/files#diff-c3956ddb663527250d29d4deb34f80770992fa1ac0063b42f34147932f2df3d4R
  理解の助けになりそうな情報
    https://github.com/olsen-blue/Arai60/pull/31/files#diff-b7fbb0dce1473afc0264185268f1a1ef6d682a3a8c997d43bc8bdd636a66ce4aR37

  解法の理解
  - まずRust実装を写経して、データの流れを追ってみる。ちなみにソートされていない配列に対して二分探索する意味がよく分からないが一旦横においておく。
  nums=[1, 98, 99, 100, 2, 4] return=4 lis=[1,2,4,100]
  - num=1, insert_position=0, lis=[1]
  - num=98, insert_position=1, lis=[1,98]
  - num=99, insert_position=2, lis=[1,98,99]
  - num=100, insert_position=3, lis=[1,98,99,100]
  - num=2, insert_position=1, lis=[1,2,99,100] <- lis[insert_position] = num　で更新している
  - num=4, insert_position=2, lis=[1,2,4,100] <- lis[insert_position] = num　で更新している
  自分の言葉で説明してみる。
  - numsの数列からlis(longest increasing subsequence 最長増加部分列)を作りたい。
    - 増加部分列とは先頭から末尾に向けて増加していく数列のこと。numsの数値から作れる増加部分列のうち、一番長い数列を作りたい。
  - lisを空で初期化する。
  - nums[i]を増加部分列に加えるとしたら、どの位置に挿入するのかをpartition_point()メソッドで確認している。
  探すときの条件はlisの中でnums[i]より小さい数値。見つけた数値の次の位置を挿入位置とする。
    - 挿入位置がlisのサイズ以上であれば、lisの数列のどの数値よりもnums[i]が大きいということなので、lis末尾にnums[i]を加える。<- 増加部分列の制約を満たす
    - 挿入位置がlisのサイズ以下である場合
      - nums[i]はlis内のある数値よりも大きいことは分かっている。
      - 見つけた挿入位置は直前の数値と比べてnums[i]が大きくなることが保証されているので、lisの挿入位置の値をnums[i]で更新する
      - 挿入位置以降にnums[i]よりも大きい数値が存在する場合は、挿入位置自体が右にずれて行くので、挿入位置は常に
      lis[left_index] < lis[insert_position] < lis[right_index] になる。
    - lisは最長増加部分列なので、lisの長さを解として返す。

  正解してから気づいたこと
  - step1では考えうる全ての状況をそのままコードで表現しようとしてうまく行かなかったような感じ。
  - 動的計画法という解法自体がそうなのかも知れないが、問題の規則性を見つけること一般化してコードに落とし込むという点が大事なので、コードを書きながら設計するような方法は難しいなと思った。
    - step1では問題についての理解が曖昧なままコードを書いてめちゃくちゃになる -> めちゃくちゃなコードを見ながら考える -> めちゃくちゃなコードを直そうとしてめちゃくちゃになる
    という良くないループに入っていた気がするので次回以降は理解できたと思うまでコードにしないアプローチで問題に取り組もうなどと考えた。
  - 二分探索を利用している理由がよくわからなかったのでChatGPT(GPT-5.1)に聞いたところ理解できた。
  まず二分探索を使う理由が分からなかった背景として、nums[i]よりも小さくなる値をlisから探すだけなら、lis先頭から線形探索して早期リターンすればよいのでは？と思っていた。
  lisは常に増加部分列となる=昇順ソートされた数列となるので二分探索を利用できる。二分探索を使えば一度の探索ごとに半分の探索候補を捨てていけるので時間計算量がO(log n)になると理解した。
  線形探索では常に先頭（又は末尾）から順に見ていくので、最悪時間計算量がO(n)になる。

  所感
  - step1aもこの解法もぱっと見た感じ何をしているかという点で分かりづらさは変わらないと感じた。
  - どういう処理の流れになっているかを確認した後では、こちらの二分探索を利用した解法の方が自然だと思った。
*/

pub struct Solution {}
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        // lis(longest_increasing_subsequence)
        let mut lis = Vec::new();

        for num in nums {
            let insert_position = lis.partition_point(|num_in_lis| *num_in_lis < num);

            if lis.len() <= insert_position {
                lis.push(num);
                continue;
            }

            lis[insert_position] = num;
        }

        lis.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn play_ground() {
        let insert_position = vec![0].partition_point(|v| *v < 0);
        assert_eq!(insert_position, 0);

        let insert_position = vec![0].partition_point(|v| *v < 1);
        assert_eq!(insert_position, 1);

        let insert_position = vec![0].partition_point(|v| *v < -1);
        assert_eq!(insert_position, 0);
    }

    #[test]
    fn step1b_test() {
        assert_eq!(Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
        assert_eq!(Solution::length_of_lis(vec![0, 1, 0, 3, 2, 3]), 4);
        assert_eq!(Solution::length_of_lis(vec![7, 7, 7, 7, 7, 7, 7]), 1);
        assert_eq!(
            Solution::length_of_lis(vec![5, 7, -24, 12, 13, 2, 3, 12, 5, 6, 35]),
            6
        );
        assert_eq!(Solution::length_of_lis(vec![1, 98, 99, 100, 2, 4]), 4);
        assert_eq!(Solution::length_of_lis(vec![7]), 1);
        assert_eq!(Solution::length_of_lis(vec![2, 1]), 1);
        assert_eq!(Solution::length_of_lis(vec![]), 0);
    }
}
