fn main() {
    let s = "hello world".to_owned();
    let mut s1 = s.as_str();
    let hello = strtok(&mut s1, ' ');
    println!("hello is: {}, s1: {}, s2: {}", hello, s1, s);

    // 用于从借用类型创建拥有所有权副本的方法，通常通过 clone 来实现
    let _t = (&5).to_owned();
}

fn strtok<'a>(s: &mut &'a str, delimeter: char) -> &'a str {
    if let Some(i) = s.find(delimeter) {
        let prefix = &s[..i];
        let suffix = &s[(i + delimeter.len_utf8())..];
        *s = suffix;
        prefix
    } else {
        let prefix = *s;
        *s = "";
        prefix
    }
}
