// Step4
/*
  目的: Step3で時間計算量、空間計算量の見積もりについて理解できていないことが分かった。
  コードを見て計算量見積もりをする時に手が止まるようなことが無いようにしたい。
*/

/*
  方法: ChatGPTの学習モードで簡単なコードと計算量を吐き出させて、計算量見積もりの練習をする。
*/

#![allow(warnings)]

/*
  時間計算量: O(N)
  計算が行われているループは入力の長さのN回分行われる

  空間計算量: O(1)
  一重ループでとくに新しい配列なども作っていない
*/
fn sum(nums: Vec<i32>) -> i32 {
    let mut total = 0;
    for n in nums {
        total += n;
    }
    total
}

/*
  時間計算量: O(N)
  for文で入力のサイズN回文計算を行っているため。

  空間計算量: O(N)
  入力の配列と同じサイズの配列をVec::with_capacityで確保しているため。
*/
fn double(nums: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::with_capacity(nums.len());
    for n in nums {
        result.push(n * 2);
    }
    result
}

/*
  時間計算量: O(N)
  n回処理を行うため。

  空間計算量: O(N)
  再帰呼び出時に呼び出しフレームがNに比例してメモリ上に存在するのでO(N)
*/
fn fact(n: i32) -> i32 {
    if n <= 1 {
        1
    } else {
        n * fact(n - 1)
    }
}
