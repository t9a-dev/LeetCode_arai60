// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  問題の理解
  141.Linked-List-Cycle
  -　リンクリストの先頭が与えられるので循環しているかを判定する。
  リンクリスト先頭からnextを見ていってnextが存在しなければ、falseを返す。
  リンクリストのnextに入っているアドレスを取得しておいて、nextで辿っていったとき同じアドレスを発見したら一周してきたことがわかるのでfalse。

  何がわからなかったか
  - Rustにおける循環参照するような型の定義や扱い。

  何を考えて解いていたか
  - この問題はLeetCodeでRust実装の回答を判定できないのでどうやってテストするか。
  ChatGPTを活用してテストコードを作成して、テストコードがパスするかで判断する。

  - テストを行うにはリンクリストで循環しているものとそうでないものを自分で生成する必要がある
  とりあえず実装方法が思いつかないのでChatGPTに作ってもらって、答えを理解したと思ったら答えを隠して自分で実装してみる。

  - headが渡された時点ではどこまでノードがつながっているがわからないので、全走査（nextを辿る）する必要がある。
  - nextで次への参照を見つけるたびにアドレスをハッシュセットに記録していき、これまでに出現したアドレスを見つけたら循環していると判定する。
  Box<T>はTを格納したヒープ領域へのスマートポインタを返すとあるので、ここからアドレスを取得して比較すればよいと思ったが、ある時点で取得したアドレス位置に
  ずっと同じデータが格納されていることが保証できないこととの兼ね合いで所有権を要求するので、安易にアドレスでの比較ができないようになっている模様。
  なのでBox<T>をclone()するようにコンパイラはヒントを出してくるが、Box<T>をclone()すると同じ値で別のヒープ領域にデータを配置するような動きになり、
  すでに見つけたアドレスなのかの判定ができなくなる。
  - Rc(reference count)でラップするとclone()しても値の複製は行われず、clone()元と同じ参照が得られる。内部的には参照カウントをインクリメントして、
  参照カウントが0（誰も値を見ていなくなった）になったら値をdropしているそう。(他の場所で参照されているのにdropしないように)
  プログラム実行中にどこかで参照されている間は同じメモリ領域を指し続けるので今回利用できる。
  - Box<T>,Rc<T>,Cell<T>,RefCell<T>について実装過程で調べることになったので、今まで曖昧なままにしていた部分が大分クリアになって良かった。

  正解してから気づいたこと
  - 変数名についてtraced_nodeよりはvisited_nodeほうがより直感的だと思った。
  - RwLock<T>よりもRefCell<T>の方が適切。ListNodeのnextを更新するために内部可変性が必要だが、RwLock<T>はマルチスレッドで利用するものなので、シングルスレッドで内部可変性を得るのにはRefCell<T>の方が適切である。
  Cell<T>はTがCopyトレイトを実装していることを要求するのでTがプリミティブ型である場合かつ、Tを丸ごと入れ替えるような場合に利用する。
  RefCell<T>は参照を返すので今回のようにTがCopyトレイトを実装しておらず構造体を丸ごと複製する必要がない場合に利用する。（構造体のフィールドのみを更新など）
  https://doc.rust-jp.rs/book-ja/ch15-05-interior-mutability.html#rct%E3%81%A8refcellt%E3%82%92%E7%B5%84%E3%81%BF%E5%90%88%E3%82%8F%E3%81%9B%E3%82%8B%E3%81%93%E3%81%A8%E3%81%A7%E5%8F%AF%E5%A4%89%E3%81%AA%E3%83%87%E3%83%BC%E3%82%BF%E3%81%AB%E8%A4%87%E6%95%B0%E3%81%AE%E6%89%80%E6%9C%89%E8%80%85%E3%82%92%E6%8C%81%E3%81%9F%E3%81%9B%E3%82%8B

  - Box<T>は必要ない。　今回の実装過程で初めて知ったが、Rc<T>もヒープ領域にデータを確保するため。ListNodeが自分自身の型をフィールドに持つようなコンパイル時にサイズが確定しない型であったので、Box<T>でラップしたがアドレスを比較するために
  HashSetにポインタ保存しておくという用途ではポインタの所有者がHashSetとListNodeと複数になるのでRc<T>の方が適切だった。
  Box<T>ではポインタの所有権が単独になるので、HashSetにポインタを保持しておくことができない。
  https://doc.rust-jp.rs/book-ja/ch15-04-rc.html

  - Rc<T>に対しては.clone()ではなく、Rc::clone()という書き方が推奨されている。
  どちらも動き方は同じだがRc<T>は値の複製を行わず参照カウンタのインクリメントのみの軽量な操作なので、Rc::clone()の方が他の型との違いが分かりやすいため。
  https://doc.rust-jp.rs/book-ja/ch15-04-rc.html#:~:text=%E3%81%A6%E3%81%84%E3%81%BE%E3%81%99%E3%80%82-,Rc,%E3%81%99

  - Rustの標準ライブラリstd::collections::LinkedListがあるが、これは双方向リンクリストと呼ばれるもので今回の題材となっている単方向リンクリストとは別物
  https://doc.rust-lang.org/std/collections/struct.LinkedList.html#

  - テストコードで利用しているbuild_list_with_cycleについて、インデックスアクセスの方法を添字からgetによる安全な方法にする。
  - テストケースでtail_nodeが自身にリンクするような自己参照パターンも生成できるように修正する。
*/

use std::{collections::HashSet, rc::Rc, sync::RwLock};

pub struct ListNode {
    next: Option<Rc<RwLock<Box<ListNode>>>>,
}

pub struct Solution {}
impl Solution {
    pub fn has_cycle(head: Option<Rc<RwLock<Box<ListNode>>>>) -> bool {
        let mut traced_node: HashSet<_> = HashSet::new();
        let mut next = head;

        while let Some(node) = next {
            if !traced_node.insert(Rc::as_ptr(&node)) {
                return true;
            }

            next = node.read().unwrap().next.clone();
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_list_with_cycle(
        cycle_position: Option<usize>,
        list_len: usize,
    ) -> Option<Rc<RwLock<Box<ListNode>>>> {
        if list_len == 0 {
            panic!("invalid list_len require 1");
        }

        let nodes = (0..list_len)
            .map(|_| Rc::new(RwLock::new(Box::new(ListNode { next: None }))))
            .collect::<Vec<_>>();
        let tail_position = list_len - 1;

        // nodeのnextに接続する
        for i in 0..tail_position {
            nodes[i].write().unwrap().next = Some(nodes[i + 1].clone())
        }

        //　tail_nodeのnextにcycle_posで指定されたnodeを接続
        if let Some(tail_next_position) = cycle_position {
            if tail_next_position < tail_position {
                nodes[tail_position].write().unwrap().next =
                    Some(nodes[tail_next_position].clone());
            }
        }

        Some(nodes[0].clone())
    }

    #[test]
    fn step1_no_cycle_test() {
        let expect = false;
        let no_cycle = build_list_with_cycle(None, 4);
        assert_eq!(Solution::has_cycle(no_cycle), expect);

        let no_cycle = build_list_with_cycle(None, 1);
        assert_eq!(Solution::has_cycle(no_cycle), expect);

        let no_cycle = build_list_with_cycle(Some(4), 2);
        assert_eq!(Solution::has_cycle(no_cycle), expect);

        let no_cycle = build_list_with_cycle(Some(2), 2);
        assert_eq!(Solution::has_cycle(no_cycle), expect);

        let with_cycle = build_list_with_cycle(Some(2), 3);
        assert_eq!(Solution::has_cycle(with_cycle), expect);
    }

    #[test]
    fn step1_with_cycle_test() {
        let expect = true;
        let with_cycle = build_list_with_cycle(Some(1), 4);
        assert_eq!(Solution::has_cycle(with_cycle), expect);

        let with_cycle = build_list_with_cycle(Some(1), 3);
        assert_eq!(Solution::has_cycle(with_cycle), expect);
    }
}
