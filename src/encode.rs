use std::os::raw::*;
use std::ptr;

use crate::boxed::WebpBox;
use crate::error::WebPSimpleError;
use crate::wrap_bytes;

/// Return the encoder's version number, packed in hexadecimal using 8bits for
/// each of major/minor/revision. E.g: v2.5.7 is 0x020507.
#[allow(non_snake_case)]
pub fn WebPGetEncoderVersion() -> u32 {
    (unsafe { libwebp_sys::WebPGetEncoderVersion() }) as u32
}

fn encode_size_check(len: usize, width: u32, height: u32, stride: u32, pixelwidth: usize) {
    assert_eq!(
        width as c_int as u32, width,
        "width {} not within c_int",
        width
    );
    assert_eq!(
        height as c_int as u32, height,
        "height {} not within c_int",
        height
    );
    assert_eq!(
        stride as c_int as u32, stride,
        "stride {} not within c_int",
        stride
    );
    let width = width as usize;
    let height = height as usize;
    let equal = if width == 0 {
        len == 0
    } else {
        len % pixelwidth == 0 && len / pixelwidth % width == 0 && len / pixelwidth / width == height
    };
    assert!(
        equal,
        "buffer size mismatch: {} * {} * {} != {}",
        width, height, pixelwidth, len
    );
}

macro_rules! wrap_encoder {
    ($name:ident, $pixelwidth:expr) => {
        #[allow(non_snake_case)]
        pub fn $name(
            rgb: &[u8],
            width: u32,
            height: u32,
            stride: u32,
            quality_factor: f32,
        ) -> Result<WebpBox<[u8]>, WebPSimpleError> {
            encode_size_check(rgb.len(), width, height, stride, $pixelwidth);
            let mut output: *mut u8 = ptr::null_mut();
            let result = unsafe {
                libwebp_sys::$name(
                    rgb.as_ptr(),
                    width as c_int,
                    height as c_int,
                    stride as c_int,
                    quality_factor as c_float,
                    &mut output,
                )
            };
            if result != 0 {
                unsafe { wrap_bytes(output, || result) }
            } else {
                Err(WebPSimpleError)
            }
        }
    };
}

wrap_encoder!(WebPEncodeRGB, 3);
wrap_encoder!(WebPEncodeBGR, 3);
wrap_encoder!(WebPEncodeRGBA, 4);
wrap_encoder!(WebPEncodeBGRA, 4);

macro_rules! wrap_lossless_encoder {
    ($name:ident, $pixelwidth:expr) => {
        #[allow(non_snake_case)]
        pub fn $name(
            rgb: &[u8],
            width: u32,
            height: u32,
            stride: u32,
        ) -> Result<WebpBox<[u8]>, WebPSimpleError> {
            encode_size_check(rgb.len(), width, height, stride, $pixelwidth);
            let mut output: *mut u8 = ptr::null_mut();
            let result = unsafe {
                libwebp_sys::$name(
                    rgb.as_ptr(),
                    width as c_int,
                    height as c_int,
                    stride as c_int,
                    &mut output,
                )
            };
            if result != 0 {
                unsafe { wrap_bytes(output, || result) }
            } else {
                Err(WebPSimpleError)
            }
        }
    };
}

wrap_lossless_encoder!(WebPEncodeLosslessRGB, 3);
wrap_lossless_encoder!(WebPEncodeLosslessBGR, 3);
wrap_lossless_encoder!(WebPEncodeLosslessRGBA, 4);
wrap_lossless_encoder!(WebPEncodeLosslessBGRA, 4);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::decode::*;

    fn lena() -> Vec<u8> {
        include_bytes!("lena.webp").to_vec()
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_WebPEncodeRGB() {
        let (width, height, buf) = WebPDecodeRGB(&lena()).unwrap();
        assert_eq!(width, 128);
        assert_eq!(height, 128);
        WebPEncodeRGB(&buf, width, height, width, 50.0).unwrap();
    }
}
