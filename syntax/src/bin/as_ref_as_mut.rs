use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

/// 对于值类型转换和引用类型转换，Rust 提供了两套不同的 trait
/// - 值类型到值类型的转换：From<T> / Into<T> / TryFrom<T> / TryInto<T>
/// - 引用类型到引用类型的转换：AsRef<T> / AsMut<T>
///
/// pub trait From<T> {
///     fn from(T) -> Self;
/// }
///
/// pub trait Into<T> {
///     fn into(self) -> T;
/// }
///
/// 实现了 From<T> 的时候会自动实现 Into<T>，因为：
/// impl<T, U> Into<U> for T
/// where U: From<T> {
///     fn into(self) -> U {
///         U::from(Self)
///     }
/// }
///
/// 如果转换过程中可能会出错，可以使用 TryFrom<T> / TryInto<T>
/// 
/// 
/// 
/// pub trait AsRef<T> Where T: ?Sized {
///     fn as_ref(&self) -> &T;
/// }
/// 
/// pub trait AsMut<T> Where T: ?Sized {
///     fn as_mut(&self) -> &T;
/// }
/// 
/// AsMut<T> 除了使用可变引用生成可变引用外，其它都和 AsRef<T> 一样

fn print(v: impl Into<IpAddr>) {
    println!("{:?}", v.into());
}

enum Language {
    Rust,
    TypeScript,
    Elixir,
    Haskell,
}

impl AsRef<str> for Language {
    fn as_ref(&self) -> &str {
        match self {
            Language::Rust => "Rust",
            Language::Elixir => "Elixir",
            Language::TypeScript => "TypeScript",
            Language::Haskell => "Haskell",
        }
    }
}

fn print_ref(v: impl AsRef<str>) {
    println!("{}", v.as_ref());
}

fn main() {
    let v4: Ipv4Addr = "2.2.2.2".parse().unwrap();
    let v6: Ipv6Addr = "::1".parse().unwrap();

    // IpAddr 实现了 From<[u8; 4]>，转换 IPv4 地址
    print([1, 1, 1, 1]);
    // IpAddr 实现了 From<[u8; 16]>，转换 IPv6 地址
    print([0xfe80, 0, 0, 0, 0xaede, 0x48ff, 0xfe00, 0x1122]);
    // IPAddr 实现了 From<Ipv4Addr>
    print(v4);
    // IPAddr 实现了 From<Ipv6Addr>
    print(v6);

    let lang = Language::Rust;
    // &str 实现了 AsRef<str>
    print_ref("Hello world");
    // String 实现了 AsRef<str>
    print_ref("Hello world".to_string());
    // Language 也实现了 AsRef<str>
    print_ref(lang);
}
