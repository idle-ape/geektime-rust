use std::str::Chars;

fn main() {
    //
    let s = "hello world".to_string();
    // &str 是 String 的切片
    let slice1 = &s[..5]; // 可以对字符串切片
                          // 也可以是 &str 的切片
    let slice2 = &slice1[1..3]; // 可以对切片再切片
    println!("{} {}", slice1, slice2); // 打印 hello el
}

// 错误，为什么？
fn lifetime1() -> String {
    let name = "Tyr".to_string();
    // 如果返回 &name[1..] 的话，name会在函数执行完之后释放，会导致悬垂引用
    name[1..].to_owned()
}

// 错误，为什么？
// 这里尝试返回 String 中的切片，但是 String 是按值传入的，
// 函数结束时 name 被释放，因此不能借用。如果给参数加上
// 生命周期并改为引用，就可以让返回值和输入保持同一个生命周期。
fn lifetime2<'a>(name: &'a String) -> &'a str {
    &name[1..]
}

// 正确，为什么？
fn lifetime3(name: &str) -> Chars {
    name.chars()
}
