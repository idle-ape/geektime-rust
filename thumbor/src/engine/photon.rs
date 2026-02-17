use crate::pb::{
    Contrast, Crop, Filter, Fliph, Flipv, Resize, ResizeType, SampleFilter, Spec, Watermark,
    filter, spec,
};
use bytes::Bytes;
use image::{DynamicImage, ImageBuffer, ImageFormat};
use lazy_static::lazy_static;
use photon_rs::{PhotonImage, native::open_image_from_bytes};
use photon_rs::{effects, filters, multiple, transform};
use std::io::Cursor;

use crate::engine::{Engine, SpecTransform};

// static WATERMARK: PhotonImage = OnceLock::new();

lazy_static! {
    // 预先把水印文件加载为静态变量
    static ref WATERMARK: PhotonImage = {
        let data = include_bytes!("../../rust-logo.png");
        let watermark = open_image_from_bytes(data).unwrap();
        transform::resize(&watermark, 64,64, transform::SamplingFilter::Nearest)
    };
}

// 目前支持 Photon engine
pub struct Photon(PhotonImage);

// 从 Bytes 转换成 Photon 结构
impl TryFrom<Bytes> for Photon {
    type Error = anyhow::Error;

    fn try_from(value: Bytes) -> Result<Self, Self::Error> {
        Ok(Self(open_image_from_bytes(&value)?))
    }
}

impl Engine for Photon {
    fn apply(&mut self, specs: &[Spec]) {
        for spec in specs.iter() {
            match spec.data {
                Some(spec::Data::Crop(ref v)) => self.transform(v),
                Some(spec::Data::Contrast(ref v)) => self.transform(v),
                Some(spec::Data::Filter(ref v)) => self.transform(v),
                Some(spec::Data::Fliph(ref v)) => self.transform(v),
                Some(spec::Data::Flipv(ref v)) => self.transform(v),
                Some(spec::Data::Resize(ref v)) => self.transform(v),
                Some(spec::Data::Watermark(ref v)) => self.transform(v),
                _ => {}
            }
        }
    }

    fn generate(self, format: ImageFormat) -> Vec<u8> {
        image_to_buf(self.0, format)
    }
}

// 裁剪图像
impl SpecTransform<&Crop> for Photon {
    fn transform(&mut self, op: &Crop) {
        let img = transform::crop(&self.0, op.x1, op.y1, op.x2, op.y2);
        self.0 = img;
    }
}

// 按指定系数调整图像对比度
impl SpecTransform<&Contrast> for Photon {
    fn transform(&mut self, op: &Contrast) {
        effects::adjust_contrast(&mut self.0, op.contrast);
    }
}

// 给图像应用滤镜
impl SpecTransform<&Filter> for Photon {
    fn transform(&mut self, op: &Filter) {
        match filter::Filter::try_from(op.filter).unwrap() {
            filter::Filter::Unspecified => {}
            f => filters::filter(&mut self.0, f.as_str_name()),
        }
    }
}

// 将图像水平翻转。
impl SpecTransform<&Fliph> for Photon {
    fn transform(&mut self, _op: &Fliph) {
        transform::fliph(&mut self.0);
    }
}

// 将图像垂直翻转。
impl SpecTransform<&Flipv> for Photon {
    fn transform(&mut self, _op: &Flipv) {
        transform::flipv(&mut self.0);
    }
}

// 调整图片大小
impl SpecTransform<&Resize> for Photon {
    fn transform(&mut self, op: &Resize) {
        let img = match ResizeType::try_from(op.rtype).unwrap() {
            ResizeType::Normal => transform::resize(
                &self.0,
                op.width,
                op.height,
                SampleFilter::try_from(op.filter).unwrap().into(),
            ),
            ResizeType::SeamCarve => transform::seam_carve(&self.0, op.width, op.height),
        };
        self.0 = img;
    }
}

// 增加水印
impl SpecTransform<&Watermark> for Photon {
    fn transform(&mut self, op: &Watermark) {
        multiple::watermark(&mut self.0, &WATERMARK, op.x as i64, op.y as i64);
    }
}

fn image_to_buf(img: PhotonImage, format: ImageFormat) -> Vec<u8> {
    let raw_pixels = img.get_raw_pixels();
    let width = img.get_width();
    let height = img.get_height();

    // 将 RGBA 转换为 RGB
    let rgb_pixels: Vec<u8> = raw_pixels
        .chunks(4)
        .flat_map(|chunk| vec![chunk[0], chunk[1], chunk[2]])
        .collect();

    let img_buffer = ImageBuffer::from_raw(width, height, rgb_pixels)
        .map(DynamicImage::ImageRgb8)
        .unwrap();

    let buffer = Vec::with_capacity(32768);
    let mut cursor = Cursor::new(buffer);
    img_buffer.write_to(&mut cursor, format).unwrap();
    cursor.into_inner()
}
