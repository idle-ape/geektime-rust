//! 切片 / 数组 / Vec / Box<[T]> 之间的关系：
//!
//!   ┌──────────────────────────┐                  ┌──────────────────────────────┐
//!   │ 切片 [T]  (DST)          │   &[T] / &mut[T] │ 切片引用 &[T] / &mut [T]     │
//!   │   ... [T][T][T] ...      │ ───────────────► │   栈: [ ptr | len(3) ]       │
//!   └──────────────────────────┘                  │           |                  │
//!                                                 │           v  (栈或堆)        │
//!                                                 │     ... [T][T][T] ...        │
//!                                                 └──────────────────────────────┘
//!                                                         ^         |
//!   ┌──────────────────────────┐    &arr[a..b]            |         |  to_vec()
//!   │ 数组 [T; n]              │ ─────────────────────────┘         |  into()
//!   │   栈: [T][T][T]          │ ◄──── try_into() ──────────────────┤
//!   └──────────────────────────┘                                    |  ^ as_slice()
//!         \                                                         v  | as_mut_slice()
//!          \ into_vec() / to_vec() / into()  ───►
//!           \ ◄──── try_into() ──────
//!            \↘                                                ┌──────────────────────────────┐
//!                                                              │ Vec<T>                       │
//!                                                              │   栈:[ ptr|cap(5)|len(3) ]   │
//!                                                              │           |                  │
//!                                                              │           v                  │
//!                                                              │   堆: [T][T][T][ ][ ]        │
//!   ┌──────────────────────────┐                               │                              │
//!   │ Box<[T]>                 │       into() /                │                              │
//!   │   栈: [ ptr | len(3) ]   │       into_boxed_slice()      │                              │
//!   │           |              │ ◄─────────────────────────────│                              │
//!   │           v              │                               │                              │
//!   │   堆: [T][T][T]          │ ──── into_vec() / into() ────►│                              │
//!   └──────────────────────────┘                               └──────────────────────────────┘

use std::ops::Deref;

fn main() {
    let arr = ['h', 'e', 'l', 'l', 'o'];
    let vec = vec!['h', 'e', 'l', 'l', 'o'];
    let s = String::from("hello");

    let s1 = &arr[1..3]; // char 类型的切片
    // vec[1..3] 借 Vec 的 Deref<Target=[T]> 自动解引用到 [T]，
    // 再走切片自己的 Index impl(https://doc.rust-lang.org/std/primitive.slice.html#impl-Index%3CI%3E-for-%5BT%5D)
    // 加上 Range<usize>: SliceIndex<[T]>(https://doc.rust-lang.org/std/slice/trait.SliceIndex.html#impl-SliceIndex%3C%5BT%5D%3E-for-Range%3Cusize%3E) 取得 &[T]
    let s2 = &vec[1..3];
    // &str 本身就是一个特殊的 slice
    let s3 = &s[1..3];
    println!("s1: {:?}, s2: {:?}, s3: {:?}", s1, s2, s3);

    // &[char] 和 &[char] 是否相等取决于长度和容量是否相等
    assert_eq!(s1, s2);

    // &[char] 和 &str 不能直接对比，把 s3 变成 Vec<char>
    assert_eq!(s2, s3.chars().collect::<Vec<_>>());

    // &[char] 可以通过迭代器转换成 String，String 和 &str 可以直接对比
    assert_eq!(String::from_iter(s2), s3);

    vec_box();
}

fn vec_box() {
    let mut v1 = vec![1, 2, 3, 4];
    println!("v1 len: {}, cap: {}", v1.len(), v1.capacity());
    v1.push(5);
    println!("after push, v1 len: {}, cap: {}", v1.len(), v1.capacity());

    // 通过 into_boxed_slice 将 Vec<T> 转换成 Box<[T]>，此时会丢弃掉多余的 capacity
    let b1 = v1.into_boxed_slice();
    let mut b2 = b1.clone();

    let v2 = b1.into_vec();
    println!("cap should be exactly 5: {}", v2.capacity());
    assert!(b2.deref() == v2);

    // Box<[T]> 可以修改其内部数据，但无法 push，因为 Box<[T]> 一旦生成就固定下来，没有 capacity，也无法增长
    b2[0] = 2;
    println!("b2: {:?}", b2);

    // Box<[T]> 和 Box<[T; N]> 并不相同
    let b3 = Box::new([2, 2, 3, 4, 5]);
    println!("b3: {:?}", b3);

    // b3.deref() 和 v2 无法比较
    let _b3_deref = b3.deref();
    // assert!(_b3_deref == v2); // the trait `PartialEq<Vec<{integer}>>` is not implemented for `&[{integer}; 5]`

    // b2 和 b3 相等
    assert!(b2 == b3);
}
