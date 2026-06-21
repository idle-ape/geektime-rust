use std::collections::HashMap;

fn main() {
    // 长度为0
    let c1 = || println!("hello world");
    // 和参数无关，长度也为0
    let c2 = |i: i32| println!("hello: {i}");

    let name = String::from("tyr");
    let name1 = name.clone();
    let mut table = HashMap::new();
    table.insert("hello", "world");

    // 如果捕获一个引用，长度为8
    let c3 = || println!("hello: {}", name);

    // 捕获移动的数据 name1(长度 24) + table(长度 48)，closure 的长度 72
    let c4 = move || println!("hello: {}, {:?}", name1, table);

    let name2 = name.clone();

    // 和局部变量无关，捕获了一个 String name2， closure 长度为 24
    let c5 = move || {
        let x = 1;
        let name3 = String::from("lindsey");
        println!("hello: {}, {:?}, {:?}", x, name2, name3);
    };

    println!(
        "c1: {}, c2: {}, c3: {}, c4: {}, c5: {}, main: {}",
        size_of_val(&c1),
        size_of_val(&c2),
        size_of_val(&c3),
        size_of_val(&c4),
        size_of_val(&c5),
        size_of_val(&main),
    );

    // FnOnce 的定义如下：
    // pub trait FnOnce<Args> {
    //     type Output;
    //     extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
    // }
    let name3 = String::from("bourne");
    // 这个闭包什么也没做，只是把捕获的参数返回。就像一个结构体例，某个字段被转移之后之后，就不能在访问一样
    // 闭包内部的数据一旦被转移，这个闭吧就不完整了，也就无法再次访问，所以 c6 是一个 FnOnce 的闭包，只能调用一次
    let c6 = move |greeting: String| (greeting, name3);
    let result = c6("hello".to_string());
    println!("result: {:?}", result);
    // let result = c6("hello".to_string()); // use of moved value: `c6`

    // 这个闭包会 clone 内部的数据返回，所以他不是 FnOnce
    let c7 = move |greeting: String| (greeting, name.clone());
    println!("result: {:?}", c7("hello".to_string()));

    // FnMut 的定义如下：
    // pub trait FnMut<Args>: FnMut<Args> {
    //     extern "rust-call" fn call_mut(
    //         &mut self,
    //         args: Args
    //     ) -> Self::Output;
    // }
    // FnMut “继承”了 FnOnce，或者说 FnOnce 是 FnMut 的 super trait，所以 FnMut 也拥有 Output 这个关联类型和 call_once 这个方法
    // 像结构体一样如果像改变数据需要用 let mut 声明一样，如果想改变闭包捕获的数据结构，那么需要 FnMut
    let mut name4 = "hello".to_string();
    let mut name5 = name4.clone();
    let mut c8 = || {
        name4.push_str(" bourne");
        println!("c8: {name4}");
    };
    let mut c9 = move || {
        name5.push_str("!");
        println!("c9: {name5}");
    };
    c8();
    c9();
    call_mut(&mut c8);
    call_mut(&mut c9);
    call_once(c8);
    call_once(c9);

    // Fn 的定义
    // pub trait Fn<Args>: FnMut<Args> {
    //     extern "rust-call" fn call(&self, args: Args) -> Self::Output;
    // }
    // Fn 继承了 FnMut，或者说 FnMut 是 Fn 的 super trait，这也就意味着，任何需要 FnMut 或者 FnOnce 的地方，都可以传入满足 Fn 的闭包
    let v = vec![0u8; 1024];
    let v1 = vec![0u8; 1023];
    let mut c10 = |x: u64| v.len() as u64 * x;
    let mut c11 = move |x: u64| v1.len() as u64 * x;
    println!("direct call: {}", c10(2));
    println!("direct call: {}", c11(2));
    println!("call: {}", call(3, &c10));
    println!("call: {}", call(3, &c11));
    println!("call_mut_with_arg: {}", call_mut_with_arg(3, &mut c10));
    println!("call_mut_with_arg: {}", call_mut_with_arg(3, &mut c11));
    println!("call_once_with_arg: {}", call_once_with_arg(3, c10));
    println!("call_once_with_arg: {}", call_once_with_arg(3, c11));
}

fn call(arg: u64, c: &impl Fn(u64) -> u64) -> u64 {
    c(arg)
}

fn call_once_with_arg(arg: u64, c: impl FnOnce(u64) -> u64) -> u64 {
    c(arg)
}

fn call_mut_with_arg(arg: u64, c: &mut impl FnMut(u64) -> u64) -> u64 {
    c(arg)
}

// 在作为参数时，FnMut 也需要显式地使用 mut，或者 &mut
fn call_mut(c: &mut impl FnMut()) {
    c();
}

// 这里不需要 mut/&mut，因为 FnMut 继承了 FnOnce，可以当作 FnOnce 传入，用闭包相当于调用了 FnOnce::call_once()
fn call_once(c: impl FnOnce()) {
    c();
}
