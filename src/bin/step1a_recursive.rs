// Step1a_recursive
// 目的: 別実装（再帰）への書き換えによる解法理解度の確認と実装練習

/*
  問題の理解
  - ソート済の重複のない整数からなる配列numsと整数targetが与えられる。
  numsにtargetが含まれる場合、numsのindexを返す。
  numsにtargetが含まれない場合、numsがソートされた状態でtargetをnumsのどの位置に挿入するべきかのindexを返す。
    - nums[1,3] target=2 output=1 nums[1]にtarget=2を挿入すればソートされた状態が維持できる
  時間計算量がO(log n)になるようなアルゴリズムで実装する必要がある。つまり、線形探索は使えない。

  何がわからなかったか
  - 再帰の基本ケースで　end < start とするところを end <= start としてしまった。
  - なぜ間違えたのか分からず end < start に直感的に修正していしまったので整理する。GPT-5.1の学習サポートモードに質問して理解を進める。
  - まずこの実装が扱っているのは閉区間[start,end]。つまり、start,end両方とも区間に含める。
    - 再帰に入るときの呼び出し方、再帰の基本ケースで分かる。
      - [start,middle - 1]と[middle+1,end]という呼び出しになっている。
        - ここで middle - 1,middle + 1 に注目するとmiddleが重複しないような呼び出しとなっている。
        閉区間だと値それ自体を含むので、[start,middle],[middle,end]とすると値が重複する。
  - 再帰の基本ケースで enc < start としている。
    - 閉区間[start,end]を扱っているので、start == end [start..=end] は要素を1つ含む。
    つまり、end <= start とすると、要素がまだ残っているのにも関わらず、基本ケースでstartを返してしまい誤りとなる。
    end < start のとき、startがendを超えていることから、この区間で表せる範囲にもう見るべき値が残っていないので、startを返す。
  - 再帰関数呼び出しの初期状態がnums.len() - 1 であることからも配列全体を示すときの区間の表し方として閉区間であることが分かる。[0..=nums.len() - 1]となる。

  区間について
  https://w3e.kanazawa-it.ac.jp/math/category/other/syuugou/henkan-tex.cgi?target=/math/category/other/syuugou/kukann.html&list=1

  何を考えて解いていたか
  - 再帰処理で実装するにはどうするか考える。
    - 基本ケース
      - target == nums[middle] return middle
      - end <= start return start <- これは間違いで end < start が正しい
    - 再帰ケース
      - middleの計算 start + end / 2
      - target < nums[middle] then end = middle - 1 else start = middle + 1
      - 更新したstart,endを利用して再帰処理に入る

  正解してから気づいたこと
  - 再帰処理の基本ケース条件を間違えて気付いたが、区間が値自体を含む閉区間なのか、含まない開区間なのかしっかり分かっていないと危うい。
  - 再帰処理にするのが難しいのではなくて、区間の開始、終了だけをずらす解法に思い至るまでに距離を感じる。
  - step1.rs,本ステップの実装ともに閉区間による処理を行っている。RustのRange記法では&nums[a..b]としたときに区間[a..b)となり、left-close right-openとなる。
  閉区間ではなく、左閉区間右開区間とするのが普通だということだろうか？ということが気になったのでGPT-5.1に聞いてみる。
    - Off-by-oneエラーを避けるため
      - &nums[a..b]がleft-close,right-openとなった根拠ではなさそうだが、&nums[a..b]のようなRange記法が[a,b)となるのは自然な感じがする。
        閉区間[a..b]とした場合、&nums[nums.len()]が範囲外アクセスとなりOff-by-oneエラーになるため。
        配列境界アクセス関連のエラーにOff-by-oneエラーという名前がついていること初めて知った。
        https://ja.wikipedia.org/wiki/Off-by-one%E3%82%A8%E3%83%A9%E3%83%BC
  こうやって見るとbinary searchを実装するときの区間の扱い方としては、left-close,right-openな半開区間が自然な気がするので、このバージョンの実装を行う。

  n = nums.le()
  時間計算量: O(log n)
  空間計算量: O(1)
*/

pub struct Solution {}
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        Self::search_insert_position(&nums, 0, nums.len() as i32 - 1, target)
    }

    fn search_insert_position(nums: &[i32], start: i32, end: i32, target: i32) -> i32 {
        if end < start {
            return start;
        }

        let middle = (start + end) / 2;
        let middle_value = nums[middle as usize];
        if middle_value == target {
            return middle;
        }

        if target < middle_value {
            return Self::search_insert_position(nums, start, middle - 1, target);
        }
        return Self::search_insert_position(nums, middle + 1, end, target);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step1a_test() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
        assert_eq!(Solution::search_insert(vec![1, 3], 2), 1);

        assert_eq!(Solution::search_insert(vec![], 8), 0);
    }
}
