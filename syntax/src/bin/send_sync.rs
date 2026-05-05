use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

/// Send/Sync 是 Rust 并发安全的基础
/// - 如果一个类型 T 实现了 Send trait，意味着 T 可以安全的从一个线程移动到另一个线程，也就是说所有权可以在线程间移动
/// - 如果一个类型 T 实现了 sync trait，则意味着 &T 可以安全的在多个线程中共享。一个类型 T 满足 Sync trait，当且仅当 &T 满足 Send trait
/// 对于 Send/Sync 在线程安全中的作用，可以理解为，如果一个类型 T: Send，那么 T 在某个线程中的独占访问是线程安全的；如果一个类型 T: Sync，那么 T 在线程间的只读共享是安全的

//  pub fn spawn<F, T>(f: F) -> JoinHandle<T>
//  where
//  F: FnOnce() -> T,
//  F: Send + 'static,
//  T: Send + 'static,
//  用 spawn 来创建一个新的线程，参数是一个闭包，这个闭包需要 Send + 'static
//  - 'static 的意思是闭包捕获的自由变量必须是一个拥有所有权的类型，或者是一个拥有静态生命周期的引用
//  - Send 意思是，这些被捕获的自由变量的所有权可以从一个线程移动到另一个线程

// `Rc<i32>` cannot be sent between threads safely
// within `{closure@syntax/src/bin/send_sync.rs:14:19: 14:26}`, the trait `Send` is not implemented for `Rc<i32>`
// Rc 不是线程安全的，所以没办法在多个线程中
fn rc_is_not_send_sync() {
    let a = Rc::new(1);
    let b = a.clone();
    let c = a.clone();

    thread::spawn(move || {
        // println!("c = {:?}", c);
    });
}

// Refcell 实现了 Send，但没有实现 Sync，所以 RefCell 可以在线程间转移所有权
fn refcell_is_not_send() {
    let a = RefCell::new(1);
    thread::spawn(move || {
        println!("a = {:?}", a);
    });
}

// Arc 一个引用计数的智能指针，它实现了线程安全的引用计数器
// RefCell 现在有多个 Arc 持有它，虽然 Arc 是 Send/Sync 的，但是 RefCell 不是 Sync
// `RefCell<i32>` cannot be shared between threads safely
// the trait `Sync` is not implemented for `RefCell<i32>`
fn refcell_is_not_sync() {
    let a = Arc::new(RefCell::new(1));
    let b = a.clone();
    let c = a.clone();
    thread::spawn(move || {
        // println!("c = {:?}", c);
    });
}

// 在多线程情况下，使用支持 Send/Sync 的 Arc，和 Mutex 一起，构造一个可以在多线程间共享且可以修改的类型
fn arc_mutex_is_send_sync() {
    let a = Arc::new(Mutex::new(1));
    let b = a.clone();
    let c = a.clone();
    let handle = thread::spawn(move || {
        let mut g = c.lock().unwrap();
        *g += 1;
    });

    {
        let mut g = b.lock().unwrap();
        *g += 1;
    }
    handle.join().unwrap();
    println!("a = {:?}", a);
}

fn main() {
    arc_mutex_is_send_sync();
}
