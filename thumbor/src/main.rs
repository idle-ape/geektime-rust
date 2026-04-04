use image::ImageFormat;
use std::{
    env,
    fs::File,
    hash::{DefaultHasher, Hash, Hasher},
    io::Read,
    num::NonZero,
    sync::Arc,
};
use tokio::sync::Mutex;

use anyhow::Result;
use axum::{
    Extension, Router,
    extract::Path,
    http::{HeaderMap, HeaderValue, StatusCode},
    routing::get,
};
use bytes::Bytes;
use lru::LruCache;
use percent_encoding::{NON_ALPHANUMERIC, percent_decode_str, percent_encode};
use serde::Deserialize;

mod pb;
use pb::*;
mod engine;
use engine::{Engine, Photon};
use tower::ServiceBuilder;
use tower_http::add_extension::AddExtensionLayer;
use tracing::info;

#[derive(Deserialize)]
struct Params {
    spec: String,
    url: String,
}

type Cache = Arc<Mutex<LruCache<u64, Bytes>>>;

#[tokio::main]
async fn main() {
    // init tracing
    tracing_subscriber::fmt::init();

    let cache: Cache = Arc::new(Mutex::new(LruCache::new(NonZero::new(1024).unwrap())));

    // constrcut router
    let app = Router::new()
        .route("/image/{spec}/{url}", get(generate))
        .layer(
            ServiceBuilder::new()
                .layer(AddExtensionLayer::new(cache))
                .into_inner(),
        );

    // run server
    let addr = "127.0.0.1:3000";

    let mut args = env::args();
    args.next();
    let image = args.next().unwrap_or(
        "https://images.pexels.com/photos/1562477/pexels-photo-1562477.jpeg?auto=compress&cs=tinysrgb&dpr=3&h=750&w=1260".into()
    );
    print_test_url(&image);

    tracing::debug!("listening on {addr}");
    let ln = tokio::net::TcpListener::bind(addr).await.unwrap();
    let _ = axum::serve(ln, app).await;
}

#[axum::debug_handler]
async fn generate(
    Path(Params { spec, url }): Path<Params>,
    Extension(cache): Extension<Cache>,
) -> Result<(HeaderMap, Vec<u8>), StatusCode> {
    let url = percent_decode_str(&url).decode_utf8_lossy();
    let data = retrieve_image(&url, cache).await.map_err(|e| {
        tracing::error!("Retrieve image failed: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    let spec: ImageSpec = spec
        .as_str()
        .try_into()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // 处理图片
    let mut engine: Photon = data
        .try_into()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    engine.apply(&spec.specs);

    let image = engine.generate(ImageFormat::Jpeg);
    info!("Finished processing: iamge size: {}", image.len());

    let mut headers = HeaderMap::new();
    headers.insert("content-type", HeaderValue::from_static("image/jpeg"));

    Ok((headers, image))
}

async fn retrieve_image(url: &str, cache: Cache) -> Result<Bytes> {
    let mut hasher = DefaultHasher::new();
    url.hash(&mut hasher);
    let key = hasher.finish();

    let mut g = cache.lock().await;
    let data = match g.get(&key) {
        Some(v) => {
            info!("Match cache {key}");
            v.to_owned()
        }
        None => {
            info!("Retrieve url");
            if url.contains("http") {
                // reqwest 和 tracing_subscriber 已经是顶级 namespace，如果你想直接使用 get，你可以 use reqwest::get，但你如果就是要通过 crate 名引入其功能，可以直接使用，就跟我们可以直接用 std::sync::Arc 一样。
                // Rust 下只要你加了依赖，对应的依赖就可以访问了，不存在 import 的过程。use 只是简化 namespace。
                let resp = reqwest::get(url).await?;
                let data = resp.bytes().await?;
                g.put(key, data.clone());
                data
            } else {
                let mut buf = Vec::new();
                File::open(url)?.read_to_end(&mut buf)?;
                Bytes::from(buf)
            }
        }
    };
    Ok(data)
}

// 调试辅助函数
fn print_test_url(url: &str) {
    use std::borrow::Borrow;
    let spec1 = Spec::new_resize(500, 800, SampleFilter::CatmullRom);
    let spec2 = Spec::new_watermark(20, 20);
    // let spec3 = Spec::new_filter(filter::Filter::Marine);
    let image_spec = ImageSpec::new(vec![spec1, spec2]);
    let s: String = image_spec.borrow().into();
    let test_image = percent_encode(url.as_bytes(), NON_ALPHANUMERIC).to_string();
    println!("test url: http://localhost:3000/image/{}/{}", s, test_image);
}
