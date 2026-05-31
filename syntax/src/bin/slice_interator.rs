//! 切片和迭代器
//! container_slice.rs 中讲到切片是容器的视图，那么迭代器定义了堆积和数据的各种各样的访问
//! 通过切片的 iter() 方法，可以生成一个迭代器，对切片进行迭代

fn main() {
    // 这里 Vec<T> 在调用 iter() 时被编译器解引用成 &[T]，所以可以访问 iter()，参考：https://doc.rust-lang.org/std/vec/struct.Vec.html#method.iter
    let result = vec![1, 2, 3, 4]
        .iter()
        .map(|v| v * v)
        .filter(|v| *v < 16)
        .take(1)
        .collect::<Vec<_>>();
    println!("{:?}", result);
}
