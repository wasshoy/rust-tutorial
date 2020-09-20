// 外部テスト用のファイル
extern crate adder;

mod common;

#[test]
fn it_adds_two() {
    // テスト対象ではないファイルからモジュールとして関数を使用
    common::setup();
    assert_eq!(4, adder::add_two(2))
}
