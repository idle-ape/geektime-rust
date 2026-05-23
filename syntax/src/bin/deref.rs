use std::ops::{Deref, DerefMut};

/// Deref 用于只读的解引用
///
/// pub trait Deref {
///     type Target: ?Sized;
///     fn deref(&self) -> &Self::Target;
/// }
///
/// DerefMut 用于可变解引用
///
/// pub trait DerefMut: Deref {
///     fn deref_mut(&mut self) -> &mut Self::Target;
/// }
///
/// 普通引用的解引用很直观，因为它只有一个指向值的地址，从这个地址可以获取到所需要的值，比如：
/// ``` rust
/// let mut x = 42;
/// let y = &mut x;
/// // 解引用，内部调用 DerefMut(其实就是 *self)
/// *y += 1;
/// ```
///
/// 但是对于只能指针来说，拿什么域来解引用就不那么直观了，比如之前的 Rc
/// ```
/// impl<T: ?Sized> Deref for Rc<T> {
///     type Target = T;
///
///     fn deref(&self) -> &T {
///         &self.inner().value
///     }
/// }
/// ```
///
/// 所以解引用时为：
/// ```
/// let a = Rc::new(1);
/// let b = a.clone();
/// println!("v = {}", *b); // *b其实是 *(b.deref())
/// ```

#[derive(Debug)]
struct Buffer<T>(Vec<T>);

impl<T> Buffer<T> {
    fn new(v: impl Into<Vec<T>>) -> Self {
        Self(v.into())
    }
}

impl<T> Deref for Buffer<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Buffer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn main() {
    // 因为 Vec 实现了 impl<T, const N: usize> From<[T; N]> for Vec<T>
    let mut buf = Buffer::new([1, 3, 2, 4]);
    // 因为 Buffer 实现了 Deref 和 DerefMut，这里 buf 可以直接访问 Vec<T> 的方法
    buf.sort(); // 等价于 (&mut buf).deref_mut().sort()，也就是 (&mut buf.0).sort()，因为 sort() 方法第一个参数是 &mut self，Rust 编译器会强制做 Deref/DerefMut 的解引用
    println!("buf: {:?}", buf);
}
