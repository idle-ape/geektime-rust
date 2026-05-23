//! 实现自己的智能指针

use std::{
    fmt::{Debug, Display},
    ops::Deref,
};

// 为什么最大是 30，因为是通过 enum 的方式来区分长字符串和短字符串：
//  - 长字符串 String 本身在栈上占 24 个字节，然后 eum 本身会通过一个字节的 tag 来区分当前是哪个变体，再加上 7 个字节的内存对齐部分，加起来内存大小就是 32 个字节
//  - 对于短字符串，除了 enum 本身一个字节的 tag 外，我们还需要一个字节来保存字符串的长度，所以用来存短字符串的数组最长只能是 30 个字节
const MINI_STRING_MAX_LEN: usize = 30;

struct MiniString {
    len: u8,
    data: [u8; MINI_STRING_MAX_LEN],
}

impl MiniString {
    fn new(v: impl AsRef<str>) -> Self {
        let bytes = v.as_ref().as_bytes();
        let len = bytes.len();
        let mut data = [0u8; MINI_STRING_MAX_LEN];
        data[..len].copy_from_slice(bytes);
        Self {
            len: len as u8,
            data,
        }
    }
}

impl Deref for MiniString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        str::from_utf8(&self.data[..self.len as usize]).unwrap()
    }
}

impl Debug for MiniString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.deref())
    }
}

#[derive(Debug)]
enum MyString {
    Inline(MiniString),
    Standard(String),
}

// 实现 Deref 接口对两种不同的场景统一得到 &str
impl Deref for MyString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            MyString::Inline(ms) => ms.deref(),
            MyString::Standard(s) => s.deref(),
        }
    }
}

impl<T> From<T> for MyString
where T: AsRef<str> + Into<String>
{
    fn from(value: T) -> Self {
        if value.as_ref().len() <= MINI_STRING_MAX_LEN {
            return MyString::Inline(MiniString::new(value));
        }
        MyString::Standard(value.into())
    }
}

impl Display for MyString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.deref())
    }
}

fn main() {
    let stack_anchor = 0u8;
    let stack_addr = &stack_anchor as *const u8 as usize;

    let len1 = std::mem::size_of::<MyString>();
    let len2 = std::mem::size_of::<MiniString>();
    println!("Len MyString: {}, Len MiniString: {}", len1, len2);

    let s1: MyString = "hello world".into();
    let s2: MyString = "这是一个超过了三十个字节的很长很长的字符串".into();
    let s1_data = s1.as_ptr() as usize;   // Deref 到 &str 后取首字节地址
    let s2_data = s2.as_ptr() as usize;
    let s1_self = &s1 as *const _ as usize; // s1 这个 enum 本身在栈上的地址

    println!("stack anchor : 0x{:x}", stack_addr);
    println!("s1 self      : 0x{:x}", s1_self);
    println!("s1 data ptr  : 0x{:x}  (offset from s1_self: {})",
             s1_data, s1_data as isize - s1_self as isize);
    println!("s2 data ptr  : 0x{:x}", s2_data);

    // debug 输出
    println!("s1: {:?}, s2: {:?}", s1, s2);
    // display 输出
    println!(
        "s1: {}({} bytes, {} chars), s2: {}({} bytes, {} chars)",
        s1,
        s1.as_bytes().len(),
        s1.chars().count(),
        s2,
        s2.as_bytes().len(),
        s2.chars().count()
    );

    // MyString 可以使用一切 &str 接口，因为 MyString 实现了 Deref
    assert!(s1.ends_with("world"));
    assert!(s2.starts_with("这"));
}
