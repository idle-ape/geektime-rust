//! Vec<T> / &[T] / Box<[T]> 等集合容器
//!
//! 在 Rust 里，切片是描述一组属于同一类型、长度不确定的、在内存中连续存放的数据结构，用 [T] 来表述。
//! 因为长度不确定，所以切片是个 DST（Dynamically Sized Type）。
//!
//! 切片之于具体的数据结构，就像数据库中的视图之于表。切片一般只出现在数据结构的定义中，不能直接访问，在使用中主要用一下形式：
//!  - &[T]: 表示一个只读的切片引用
//!  - &mut [T]: 表示一个可写的切片引用
//!  - Box<[T]>: 一个在堆上分配的切片

fn main() {
    arr_vec_cmp();
    deref_for_which_support_slice();
}

fn arr_vec_cmp() {
    let arr = [1, 2, 3, 4, 5];
    let vec = vec![1, 2, 3, 4, 5];
    let s1 = &arr[..2];
    let s2 = &vec[..2];

    println!("s1: {:?}, s2: {:?}", s1, s2);

    // 两个切片是否相等，取决于长度和内容是否相等
    assert_eq!(s1, s2);

    // 切片可以和 Vec<T> / [T; N] 比较，也会看长度和容量
    // 对于 array 和 vector，虽然是不同的数据结构，一个放在栈上，一个放在堆上，但它们的切片是类似的；
    // 而且对于相同内容数据的相同切片，比如 &arr[1…3] 和 &vec[1…3]，这两者是等价的。
    // 除此之外，切片和对应的数据结构也可以直接比较，这是因为它们之间实现了 PartialEq trait
    assert_eq!(&arr[..], arr);
    assert_eq!(&vec[..], vec);
}

// 支持切片的具体数据类型，你可以根据需要，解引用转换成切片类型。
// 比如 Vec 和 [T; n] 会转化成为 &[T]，这是因为 Vec 实现了 Deref trait，而 array 内建了到 &[T] 的解引用
fn deref_for_which_support_slice() {
    let v = vec![1, 2, 3, 4];

    // Vec 实现了 Deref，&Vec<T> 会被自动解引用成 &[T](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.deref)，符合接口定义
    print_slice(&v);
    // vec的切片，直接就是 &[T]，符合接口定义
    print_slice(&v[..]);

    // &Vec<T> 也实现了 AsRef<[T]>: https://doc.rust-lang.org/std/vec/struct.Vec.html#impl-AsRef%3C%5BT%5D%3E-for-Vec%3CT,+A%3E
    // 加上引用的 blanket impl(https://doc.rust-lang.org/std/convert/trait.AsRef.html#impl-AsRef%3CU%3E-for-%26T) 组合而来
    print_slice1(&v);
    // &[T] 也实现了 AsRef<[T]>：由 [T] 自身的 impl(https://doc.rust-lang.org/std/primitive.slice.html#impl-AsRef%3C%5BT%5D%3E-for-%5BT%5D)
    // 加上引用的 blanket impl(https://doc.rust-lang.org/std/convert/trait.AsRef.html#impl-AsRef%3CU%3E-for-%26T) 组合而来
    print_slice1(&v[..]);
    // Vec<T> 也实现了 AsRef<[T]>: https://doc.rust-lang.org/std/vec/struct.Vec.html#impl-AsRef%3C%5BT%5D%3E-for-Vec%3CT,+A%3E
    print_slice1(v);

    // 数组虽然没有实现 Deref，但是它的解引用就是 &[T]
    let arr = [1, 2, 3, 4];
    // &[T; N] 通过编译器内建的 unsized coercion 直接变成 &[T]，不走 trait
    print_slice(&arr);
    // arr[..] 走切片语法（Index），得到 [T]，&arr[..] 即 &[T]
    print_slice(&arr[..]);
    // [T; N] 实现了 AsRef<[T]>(https://doc.rust-lang.org/std/primitive.array.html#impl-AsRef%3C%5BT%5D%3E-for-%5BT;+N%5D)，
    // 再叠加引用的 blanket impl，&[T; N] 也满足 AsRef<[T]>
    print_slice1(&arr);
    // [T] 自身的 AsRef<[T]> impl + 引用的 blanket impl，&[T] 满足 AsRef<[T]>
    print_slice1(&arr[..]);
    // [T; N] 直接（按值）满足 AsRef<[T]>，无需 blanket impl
    print_slice1(arr);
}

fn print_slice<T: std::fmt::Debug>(s: &[T]) {
    println!("{:?}", s);
}

fn print_slice1<T, U>(s: T)
where
    T: AsRef<[U]>,
    U: std::fmt::Debug,
{
    println!("{:?}", s.as_ref());
}
