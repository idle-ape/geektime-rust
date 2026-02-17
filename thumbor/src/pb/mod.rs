use std::io::{Read, Write};

use base64::{alphabet::URL_SAFE, engine, read, write};
use photon_rs::transform::SamplingFilter;
use prost::Message;

mod abi;

pub use abi::*;

impl ImageSpec {
    pub fn new(spces: Vec<Spec>) -> Self {
        Self { specs: spces }
    }
}

// 实现从 ImageSpec 到 String 的转换
impl From<&ImageSpec> for String {
    fn from(image_spec: &ImageSpec) -> Self {
        let data = image_spec.encode_to_vec();
        let engine = engine::GeneralPurpose::new(&URL_SAFE, engine::general_purpose::NO_PAD);
        let mut encoded = Vec::new();
        {
            let mut encoder = write::EncoderWriter::new(&mut encoded, &engine);
            encoder.write_all(&data).unwrap();
        }
        String::from_utf8(encoded).unwrap()
    }
}

// 让 ImageSpec 可以通过一个字符串创建, 比如 s.parse().unwrap
impl TryFrom<&str> for ImageSpec {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let engine = engine::GeneralPurpose::new(&URL_SAFE, engine::general_purpose::NO_PAD);
        let mut decoder = read::DecoderReader::new(value.as_bytes(), &engine);
        let mut buf = Vec::new();
        decoder.read_to_end(&mut buf)?;
        Ok(ImageSpec::decode(&buf[..])?)
    }
}

// 在自定义的 SampleFilter 和 photon_rs 的 SamplingFilter 之间进行转换
// 如：let nearest = SampleFilter::Triangle.into();
impl From<SampleFilter> for SamplingFilter {
    fn from(value: SampleFilter) -> Self {
        match value {
            SampleFilter::Undefined | SampleFilter::Nearest => SamplingFilter::Nearest,
            SampleFilter::Triangle => SamplingFilter::Triangle,
            SampleFilter::CatmullRom => SamplingFilter::CatmullRom,
            SampleFilter::Gaussian => SamplingFilter::Gaussian,
            SampleFilter::Lanczos3 => SamplingFilter::Lanczos3,
        }
    }
}

// 提供一些辅助函数
impl Spec {
    pub fn new_resize_seam_carve(width: u32, height: u32) -> Self {
        Self {
            data: Some(spec::Data::Resize(Resize {
                width,
                height,
                rtype: ResizeType::SeamCarve as i32,
                filter: SampleFilter::Undefined as i32,
            })),
        }
    }

    pub fn new_resize(width: u32, height: u32, filter: SampleFilter) -> Self {
        Self {
            data: Some(spec::Data::Resize(Resize {
                width,
                height,
                rtype: ResizeType::Normal.into(),
                filter: filter.into(),
            })),
        }
    }

    pub fn new_filter(filter: filter::Filter) -> Self {
        Self {
            data: Some(spec::Data::Filter(Filter {
                filter: filter.into(),
            })),
        }
    }

    pub fn new_watermark(x: u32, y: u32) -> Self {
        Self {
            data: Some(spec::Data::Watermark(Watermark { x, y })),
        }
    }
}

#[cfg(test)]
mod test {
    use std::vec;

    use super::*;

    #[test]
    fn encoded_spec_could_be_decoded() {
        let spec1 = Spec::new_resize(600, 600, SampleFilter::CatmullRom);
        let spec2 = Spec::new_filter(filter::Filter::Marine);
        let image_sepc = ImageSpec::new(vec![spec1, spec2]);
        let s: String = (&image_sepc).into();
        println!("{s}");
        assert_eq!(image_sepc, s.as_str().try_into().unwrap());
    }
}
