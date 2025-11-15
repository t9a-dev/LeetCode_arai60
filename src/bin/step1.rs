// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  問題の理解
  - 柱の本数n、色の種類kが与えられる。隣接する柱が2本以上同じにならないように(2本連続はok)、全ての支柱を塗装するとき、塗装できるパターンの合計数を返す。
  全ての柱を塗装する必要があるので、色を塗っていない柱は存在できない。
  柱を塗装するパターンの合計数を返す。

  何がわからなかったか
  - 柱が1本のときに、一度色を塗ってしまうと別の色を塗れないと勘違いしていた。つまりk > 0でn=1のとき常に1になると思っていた。
  - 問題のカテゴリであるDynamic Programmingという単語を知らない。
  日本語で「動的計画法」。複雑な問題を部分問題に分割して解を求める。最後に部分問題の解を利用して全体の解を求める。
  https://ja.wikipedia.org/wiki/%E5%8B%95%E7%9A%84%E8%A8%88%E7%94%BB%E6%B3%95
  今までよりも少し抽象度が高い感じがする。

  何を考えて解いていたか
  - lintcodeでは電話番号登録しないとコード実行できなさそうなのでGPT-5でテストケースを生成して正否を確認する。
  - n * k で組み合わせが求められそうだが、具体例を考えて考慮するべき点が漏れていないか確認する。
    - n * kではダメそう。
  - (n-k) > k で色を塗れない
  - n-k = 1 -> n*k
  - (n-k) > 1 -> n+k

  整数=柱の数,r,g,b=色
  柱が一本のときは1色しか塗装できないのでそのままnを返す。
  1
  r

  n=2,k=1のとき2パターンとなる。
  1 2
  r r

  n=3,k=1の時色を塗れない柱があるので、0パターンとなる <- 間違っていて、単色だと連続して2本以上になるパターンしか塗れないので0になる。

  n=3,k=2のとき6パターンとなる。
  1 2 3
  r r g
  g g r
  r g r
  r g g
  g r g
  g r r

  一度コードを書いて通らないテストケースがあった。
  柱が1本のときに、全ての色を使えないのでパターンは1になると考えていたが、そもそもこの理解が間違っていた。
  全ての色を同時に使う必要はないので、柱が1つの場合は色の種類がそのまま答えになる。
  問題もよくわからないので答えを見て先に進む

  答えの実装(by GPT-5)の理解
  - 制約として同じ色は2つまでしか連続して良い。
    - 2つ同じ色が連続していれば、その色を除いたk-1色を次に利用できる。
    - 2つ同じ色が連続していなければ、k色を次に利用できる。

  正解してから気づいたこと
  - 問題のカテゴリである Dynamic Programming(動的計画法) は単語自体も初めて聞いたレベルだったこともあり、問題と解法理解に時間がかかった。
    - 特に解法で tail_two_same,tail_two_diffを更新しているところで何をしているのか、なぜこうするのかを理解するのにかなり時間がかかった。
    書籍「なっとく！アルゴリズム　第2版」の動的計画法の章を読んだものの、このトピック(動的計画法)が難しいということしか理解できなかった。書籍の例題で扱われているナップザック問題とこの問題はカテゴリこそ「動的計画法」で同じだが、解法はぜんぜん違うように見えるため。
    書籍の中で「問題と解法を理解して式にする（コードに落とし込む）簡単な方法は無く、試行錯誤するしか無い」とあり、まあそうだようなと思った。
    この章の中で冗談混じりに紹介されていた(P.251)問題解決のアルゴリズムとして、著名な物理学者リチャード・ファインマンによるアルゴリズム「ファインマンアルゴリズム」が紹介されていた。
      - 問題を紙に書き出す
      - ひたすらその問題を考える
      - 解決法を紙に書き留める
    当たり前のように思えるが、理解が難しくて手が止まってしまったときに問題を眺めるだけでなく、紙やノートに書き出してみるというのは有効な手段であることを改めて実感した。(現代(2025年)にいおいてはChatGPTを質問攻めにするのも含め)
      - 脱線するが物理学者ファインマンによるエッセイ集を手に取るきっかけとなった「僕は自分が思っていたほどは頭がよくなかった」というRedditでのポストと、このポストに対する返信の日本語翻訳を思い出した。
      https://b.log456.com/entry/20120110/p1
      NandToTetrisが難しくて進捗が止まった時に、自分は頭が良くないから向いていないので止めてしまおうと考えたが、このポストを読み返して踏みとどまっていたことなどを思い出した。

  所感
  - 学習勾配がだいぶきついように感じる。
  - とりあえず問題で何が聞かれているのかは分かったはずなので、時間・空間計算量はともかく手作業で解けるかを試す。(step1a.rs)
*/

pub struct Solution {}
impl Solution {
    pub fn num_ways(n: i32, k: i32) -> i32 {
        if n <= 0 || k <= 0 {
            return 0;
        };

        if n == 1 {
            return k;
        };

        // k=3(a,b,c) -> 3(aa,bb,cc)
        let mut tail_two_same = k;
        // k=3(a,b,c) -> 6(ab,ac,ba,bc,ca,cb)
        let mut tail_two_diff = k * (k - 1);

        if n == 2 {
            return tail_two_same + tail_two_diff;
        };

        for _ in 3..=n {
            let previous_tail_two_same = tail_two_same;
            let previous_tail_two_diff = tail_two_diff;

            tail_two_same = previous_tail_two_diff;
            tail_two_diff = (previous_tail_two_same + previous_tail_two_diff) * (k - 1)
        }

        tail_two_same + tail_two_diff
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step1_test() {
        assert_eq!(Solution::num_ways(3, 2), 6);
        assert_eq!(Solution::num_ways(2, 2), 4);
        assert_eq!(Solution::num_ways(3, 1), 0);
        assert_eq!(Solution::num_ways(4, 2), 10);

        // --- basic cases ---
        assert_eq!(Solution::num_ways(1, 1), 1);
        assert_eq!(Solution::num_ways(1, 2), 2);
        assert_eq!(Solution::num_ways(1, 3), 3);
        assert_eq!(Solution::num_ways(2, 1), 1);
        assert_eq!(Solution::num_ways(2, 3), 9);

        // --- constraint cases ---
        assert_eq!(Solution::num_ways(3, 1), 0);
        assert_eq!(Solution::num_ways(4, 1), 0);
        assert_eq!(Solution::num_ways(5, 1), 0);
        assert_eq!(Solution::num_ways(3, 3), 24);

        // --- larger values ---
        assert_eq!(Solution::num_ways(4, 3), 66);
        assert_eq!(Solution::num_ways(5, 2), 16);
        assert_eq!(Solution::num_ways(5, 3), 180);

        // --- n = 0 をどう扱うか（学習用） ---
        assert_eq!(Solution::num_ways(0, 1), 0);
        assert_eq!(Solution::num_ways(0, 3), 0);
    }
}
