use serde::Deserialize;
use std::borrow::Cow;

use url::Url;

#[derive(Debug, Deserialize)]
struct User<'a> {
    #[serde(borrow)]
    name: Cow<'a, str>,
    age: u8,
}

fn main() {
    let url = Url::parse("https://tyr.com/rust?page=1024&sort=desc&extra=hello%20world").unwrap();
    let mut pairs = url.query_pairs();

    assert_eq!(pairs.count(), 3);

    let (mut k, v) = pairs.next().unwrap();
    println!("key: {}, v: {}", k, v);

    // 当修改发生时，k 变成 Owned
    k.to_mut().push_str("_lala");
    print_pairs((k, v));

    print_pairs(pairs.next().unwrap());

    // 在处理 extra=hello%20world 是，value 被处理成 hello world，所以这里的 value 是 Owned
    print_pairs(pairs.next().unwrap());

    // 让 User 中的 name 来使用 Cow 来引用 JSON 文本中的内容，以提高性能
    let json = r#"{"name": "Tyr", "age": 18}"#;
    let mut user: User = serde_json::from_str(json).unwrap();
    println!("{}", show_cow(&user.name));

    // Clone on Write，调用者需要所有权或者需要修改内容，它才会 clone 借用的数据
    user.name.to_mut().push_str(" Chen");
    println!("{}", show_cow(&user.name));
}

fn print_pairs(pair: (Cow<str>, Cow<str>)) {
    println!("Key: {}, value: {}", show_cow(&pair.0), show_cow(&pair.1));
}

fn show_cow(cow: &Cow<str>) -> String {
    match cow {
        Cow::Borrowed(v) => format!("Borrowed: {}", v),
        Cow::Owned(v) => format!("Owned: {}", v),
    }
}
