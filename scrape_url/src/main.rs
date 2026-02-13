use std::{collections::HashMap, fs, sync::OnceLock};

static GLOBAL_MAP: OnceLock<HashMap<i32, String>> = OnceLock::new();

fn main() -> Result<(), &'static str> {
    let mut args = std::env::args();
    args.next();
    args.next();
    let url = args.next().unwrap_or("https://www.rust-lang.org/".into());
    let output = "rust.md";

    println!("Fetching url: {url}");

    let body = reqwest::blocking::get(url).unwrap().text().unwrap();

    let md = html2md::parse_html(&body);
    if let Err(_) = fs::write(output, md) {
        return Err("convert error");
    }

    println!("Convert markdown has been saved in {output}");

    // 第一次调用时初始化
    let map = GLOBAL_MAP.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert(1, "Hello".to_string());
        m
    });
    println!("{:?}", map);

    let d = map.get(&1).ok_or("not found")?;
    println!("{d}");
    return Ok(());
}
