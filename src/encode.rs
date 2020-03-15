use libwebp_sys as sys;
use std::os::raw::*;
use std::ptr;

use crate::boxed::{wrap_bytes, WebpBox};
use crate::error::WebPSimpleError;

/// Return the encoder's version number, packed in hexadecimal using 8bits for
/// each of major/minor/revision. E.g: v2.5.7 is 0x020507.
#[allow(non_snake_case)]
pub fn WebPGetEncoderVersion() -> u32 {
    (unsafe { sys::WebPGetEncoderVersion() }) as u32
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

#[allow(non_snake_case)]
pub fn WebPEncodeRGB(
    rgb: &[u8],
    width: u32,
    height: u32,
    stride: u32,
    quality_factor: f32,
) -> Result<WebpBox<[u8]>, WebPSimpleError> {
    encode_size_check(rgb.len(), width, height, stride, 3);
    let mut output: *mut u8 = ptr::null_mut();
    let result = unsafe {
        sys::WebPEncodeRGB(
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

#[allow(non_snake_case)]
pub fn WebPEncodeBGR(
    bgr: &[u8],
    width: u32,
    height: u32,
    stride: u32,
    quality_factor: f32,
) -> Result<WebpBox<[u8]>, WebPSimpleError> {
    encode_size_check(bgr.len(), width, height, stride, 3);
    let mut output: *mut u8 = ptr::null_mut();
    let result = unsafe {
        sys::WebPEncodeBGR(
            bgr.as_ptr(),
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

#[allow(non_snake_case)]
pub fn WebPEncodeRGBA(
    rgba: &[u8],
    width: u32,
    height: u32,
    stride: u32,
    quality_factor: f32,
) -> Result<WebpBox<[u8]>, WebPSimpleError> {
    encode_size_check(rgba.len(), width, height, stride, 4);
    let mut output: *mut u8 = ptr::null_mut();
    let result = unsafe {
        sys::WebPEncodeRGBA(
            rgba.as_ptr(),
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

#[allow(non_snake_case)]
pub fn WebPEncodeBGRA(
    bgra: &[u8],
    width: u32,
    height: u32,
    stride: u32,
    quality_factor: f32,
) -> Result<WebpBox<[u8]>, WebPSimpleError> {
    encode_size_check(bgra.len(), width, height, stride, 4);
    let mut output: *mut u8 = ptr::null_mut();
    let result = unsafe {
        sys::WebPEncodeBGRA(
            bgra.as_ptr(),
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

#[allow(non_snake_case)]
pub fn WebPEncodeLosslessRGB(
    rgb: &[u8],
    width: u32,
    height: u32,
    stride: u32,
) -> Result<WebpBox<[u8]>, WebPSimpleError> {
    encode_size_check(rgb.len(), width, height, stride, 3);
    let mut output: *mut u8 = ptr::null_mut();
    let result = unsafe {
        sys::WebPEncodeLosslessRGB(
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

#[allow(non_snake_case)]
pub fn WebPEncodeLosslessBGR(
    bgr: &[u8],
    width: u32,
    height: u32,
    stride: u32,
) -> Result<WebpBox<[u8]>, WebPSimpleError> {
    encode_size_check(bgr.len(), width, height, stride, 3);
    let mut output: *mut u8 = ptr::null_mut();
    let result = unsafe {
        sys::WebPEncodeLosslessBGR(
            bgr.as_ptr(),
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

#[allow(non_snake_case)]
pub fn WebPEncodeLosslessRGBA(
    rgba: &[u8],
    width: u32,
    height: u32,
    stride: u32,
) -> Result<WebpBox<[u8]>, WebPSimpleError> {
    encode_size_check(rgba.len(), width, height, stride, 4);
    let mut output: *mut u8 = ptr::null_mut();
    let result = unsafe {
        sys::WebPEncodeLosslessRGBA(
            rgba.as_ptr(),
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

#[allow(non_snake_case)]
pub fn WebPEncodeLosslessBGRA(
    bgra: &[u8],
    width: u32,
    height: u32,
    stride: u32,
) -> Result<WebpBox<[u8]>, WebPSimpleError> {
    encode_size_check(bgra.len(), width, height, stride, 4);
    let mut output: *mut u8 = ptr::null_mut();
    let result = unsafe {
        sys::WebPEncodeLosslessBGRA(
            bgra.as_ptr(),
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
