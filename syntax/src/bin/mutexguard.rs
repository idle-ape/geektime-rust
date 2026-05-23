use std::borrow::Cow;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use lazy_static::lazy_static;

lazy_static! {
    // 一般情况下 Mutx 和 Arc 一起在多线程环境下提供对共享内存的使用
    // 如果把 Mutex 声明成 static，其声明周期是静态的，就不需要 Arc
    #[derive(Debug)]
    static ref METRICS: Mutex<HashMap<Cow<'static, str>, usize>> = Mutex::new(HashMap::new());
}

fn main() {
    let metrics = Arc::new(Mutex::new(HashMap::new()));

    for _ in 0..32 {
        let m = metrics.clone();
        thread::spawn(move || {
            let mut g = m.lock().unwrap();
            // 此时只有拿到了 Mutexguard 的线程可以访问 Hashmap
            let data = &mut *g;

            let entry = data.entry(Cow::Borrowed("hello")).or_insert(0);
            *entry += 1;
        });
    }

    for _ in 0..32 {
        thread::spawn(||{
            let mut g = METRICS.lock().unwrap();
            let data = &mut *g;
            let entry = data.entry("hello".into()).or_insert(0);
            *entry += 1;
        });
    }

    thread::sleep(Duration::from_millis(100));
    println!("metrics: {:?}", metrics);
    println!("METRICS: {:?}", *METRICS);
}
