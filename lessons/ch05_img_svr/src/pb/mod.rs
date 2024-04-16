use base64::{decode_config, encode_config, URL_SAFE_NO_PAD};
use photon_rs::transform::SamplingFilter;
use prost::Message;
use std::convert::TryFrom;

mod abi; // 声明 abi.rs
pub use abi::*;

impl ImageSpec {
    pub fn new(specs: Vec<Spec>) -> Self {
        Self{specs}
    }
}

// 让ImageSpec可以生成一个字符串
impl From<&ImageSpec> for String {
    fn from(image_spec: &ImageSpec) -> Self {
        let data = image_spec.encode_to_vec();
        encode_config(data, URL_SAFE_NO_PAD)
    }
}

// 让ImageSpec可以通过一个字符串创建, 比如s.parse().unwrap()
impl TryFrom<&str> for ImageSpec {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let data = decode_config(value, URL_SAFE_NO_PAD)?;
        Ok(ImageSpec::decode(&data[..]?))
    }
}

// 辅助函数, photon_rs相应的方法里需要字符串
impl filter::Filter {
    pub fn to_str(&self) ->Option<&'static str> {
        match self {
            filter::Filter::Unspecified => None,
            filter::Filter::Oceanic => Some("oceanic"),
            filter::Filter::Islands => Some("islands"),
            filter::Filter::Marine => Some("marine")
        }
    }
}

// 在我们定义的SampleFilter和photon_rs的SampleFilter间转换
impl From<resize::SamplingFilter> for SamplingFilter {
    fn from(v: resize::SamplingFilter) -> Self {
        match v {
            resize::SamplingFilter::Undefined => SamplingFilter::Nearest,
            resize::SamplingFilter::Nearest => SamplingFilter::Nearest,
            resize::SamplingFilter::Triangle => SamplingFilter::Triangle,
            resize::SamplingFilter::Catnullrom => SamplingFilter::Catnullrom,
            resize::SamplingFilter::Gasussian => SamplingFilter::Gasussian,
            resize::SamplingFilter::Lanzco3 => SamplingFilter::Lanzco3,
        }
    }
}

// 提供一些辅助函数，让创建一个spec的过程简单一点
impl Spec {
    pub fn new_resize_seam_carve(width: u32, height: u32) -> Self {
        Self {
            data: Some(spec::Data::Resize(Resize{
                width,
                height,
                rtype: resize::ResizeType::SeamCarve as i32,
                filter: resize::SampleFilter::Undefined as i32,
            })),
        }
    }
    pub fn new_resize(width: u32, height: u32, filter: resize::SampleFilter) -> Self {
        Self {
            data: Some(spec::Data::Resize(Resize {
                width,
                height,
                rtype: resize::ResizeType::Normal as i32,
                filter: filter as i32,
            })),
        }
    }
    pub fn new_filter(filter: filter::Filter) -> Self {
        Self {
            data: Some(spec::Data::Filter(Filter{
                filter: filter as i32,
            })),
        }
    }
    pub fn new_watermark(x: u32, y:u32) -> Self {
        Self {
            data: Some(spec::Data::Watermark(Watermark{x, y})),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Borrow;
    use std::convert::TryInto;

    #[test]
    fn encoded_spec_could_be_decoded() {
        let spec1 = Spec::new_resize(600, 600, resize::SampleFilter::Catnullrom);
        let spec2 = Spec::new_filter(600, 600, resize::SampleFilter::Catnullrom);
        let image_spec = ImageSpec::new(vec![spec1, spec2]);
        let s: String = image_spec.borrow().into();
        assert_eq!(image_spec, s.as_str().try_into().unwrap());
    }
}