use libwebp_sys as sys;
use std::os::raw::*;
use std::ptr;
use std::slice;

use crate::boxed::{wrap_bytes, WebpBox, WebpYuvBox};
use crate::error::WebPSimpleError;

/// Return the decoder's version number, packed in hexadecimal using 8bits for
/// each of major/minor/revision. E.g: v2.5.7 is 0x020507.
#[allow(non_snake_case)]
pub fn WebPGetDecoderVersion() -> u32 {
    (unsafe { sys::WebPGetDecoderVersion() }) as u32
}

/// Retrieve basic header information: width, height.
/// This function will also validate the header, returning true on success,
/// false otherwise. '*width' and '*height' are only valid on successful return.
/// Pointers 'width' and 'height' can be passed NULL if deemed irrelevant.
#[allow(non_snake_case)]
pub fn WebPGetInfo(data: &[u8]) -> Result<(u32, u32), WebPSimpleError> {
    let mut width: c_int = 0;
    let mut height: c_int = 0;
    let result = unsafe { sys::WebPGetInfo(data.as_ptr(), data.len(), &mut width, &mut height) };
    if result != 0 {
        Ok((width as u32, height as u32))
    } else {
        Err(WebPSimpleError)
    }
}

/// Decodes WebP images pointed to by 'data' and returns RGBA samples, along
/// with the dimensions in *width and *height. The ordering of samples in
/// memory is R, G, B, A, R, G, B, A... in scan order (endian-independent).
/// The returned pointer should be deleted calling WebPFree().
/// Returns NULL in case of error.
#[allow(non_snake_case)]
pub fn WebPDecodeRGBA(data: &[u8]) -> Result<(u32, u32, WebpBox<[u8]>), WebPSimpleError> {
    let mut width: c_int = 0;
    let mut height: c_int = 0;
    let result = unsafe { sys::WebPDecodeRGBA(data.as_ptr(), data.len(), &mut width, &mut height) };
    let buf = (unsafe { wrap_bytes(result, || width as usize * height as usize * 4) })?;
    Ok((width as u32, height as u32, buf))
}

/// Same as WebPDecodeRGBA, but returning A, R, G, B, A, R, G, B... ordered data.
#[allow(non_snake_case)]
pub fn WebPDecodeARGB(data: &[u8]) -> Result<(u32, u32, WebpBox<[u8]>), WebPSimpleError> {
    let mut width: c_int = 0;
    let mut height: c_int = 0;
    let result = unsafe { sys::WebPDecodeARGB(data.as_ptr(), data.len(), &mut width, &mut height) };
    let buf = (unsafe { wrap_bytes(result, || width as usize * height as usize * 4) })?;
    Ok((width as u32, height as u32, buf))
}

/// Same as WebPDecodeRGBA, but returning B, G, R, A, B, G, R, A... ordered data.
#[allow(non_snake_case)]
pub fn WebPDecodeBGRA(data: &[u8]) -> Result<(u32, u32, WebpBox<[u8]>), WebPSimpleError> {
    let mut width: c_int = 0;
    let mut height: c_int = 0;
    let result = unsafe { sys::WebPDecodeBGRA(data.as_ptr(), data.len(), &mut width, &mut height) };
    let buf = (unsafe { wrap_bytes(result, || width as usize * height as usize * 4) })?;
    Ok((width as u32, height as u32, buf))
}

/// Same as WebPDecodeRGBA, but returning R, G, B, R, G, B... ordered data.
/// If the bitstream contains transparency, it is ignored.
#[allow(non_snake_case)]
pub fn WebPDecodeRGB(data: &[u8]) -> Result<(u32, u32, WebpBox<[u8]>), WebPSimpleError> {
    let mut width: c_int = 0;
    let mut height: c_int = 0;
    let result = unsafe { sys::WebPDecodeRGB(data.as_ptr(), data.len(), &mut width, &mut height) };
    let buf = (unsafe { wrap_bytes(result, || width as usize * height as usize * 3) })?;
    Ok((width as u32, height as u32, buf))
}

/// Same as WebPDecodeRGB, but returning B, G, R, B, G, R... ordered data.
#[allow(non_snake_case)]
pub fn WebPDecodeBGR(data: &[u8]) -> Result<(u32, u32, WebpBox<[u8]>), WebPSimpleError> {
    let mut width: c_int = 0;
    let mut height: c_int = 0;
    let result = unsafe { sys::WebPDecodeBGR(data.as_ptr(), data.len(), &mut width, &mut height) };
    let buf = (unsafe { wrap_bytes(result, || width as usize * height as usize * 3) })?;
    Ok((width as u32, height as u32, buf))
}

/// Decode WebP images pointed to by 'data' to Y'UV format(*). The pointer
/// returned is the Y samples buffer. Upon return, *u and *v will point to
/// the U and V chroma data. These U and V buffers need NOT be passed to
/// WebPFree(), unlike the returned Y luma one. The dimension of the U and V
/// planes are both (*width + 1) / 2 and (*height + 1)/ 2.
/// Upon return, the Y buffer has a stride returned as '*stride', while U and V
/// have a common stride returned as '*uv_stride'.
/// Return NULL in case of error.
/// (*) Also named Y'CbCr. See: http://en.wikipedia.org/wiki/YCbCr
#[allow(non_snake_case)]
pub fn WebPDecodeYUV(data: &[u8]) -> Result<(u32, u32, u32, u32, WebpYuvBox), WebPSimpleError> {
    let mut width: c_int = 0;
    let mut height: c_int = 0;
    let mut u: *mut u8 = ptr::null_mut();
    let mut v: *mut u8 = ptr::null_mut();
    let mut stride: c_int = 0;
    let mut uv_stride: c_int = 0;
    let result = unsafe {
        sys::WebPDecodeYUV(
            data.as_ptr(),
            data.len(),
            &mut width,
            &mut height,
            &mut u,
            &mut v,
            &mut stride,
            &mut uv_stride,
        )
    };
    if !result.is_null() {
        let y_len = height as usize * stride as usize;
        let uv_len = (height as usize + 1) / 2 * uv_stride as usize;
        let buf = unsafe {
            WebpYuvBox::from_raw_yuv(
                slice::from_raw_parts_mut(result, y_len),
                slice::from_raw_parts_mut(u, uv_len),
                slice::from_raw_parts_mut(v, uv_len),
            )
        };
        Ok((
            width as u32,
            height as u32,
            stride as u32,
            uv_stride as u32,
            buf,
        ))
    } else {
        Err(WebPSimpleError)
    }
}

#[allow(non_snake_case)]
pub fn WebPDecodeRGBAInto(
    data: &[u8],
    output_buffer: &mut [u8],
    output_stride: u32,
) -> Result<(), WebPSimpleError> {
    assert!(output_stride as c_int >= 0);
    assert_eq!(output_stride as c_int as u32, output_stride);
    let result = unsafe {
        sys::WebPDecodeRGBAInto(
            data.as_ptr(),
            data.len(),
            output_buffer.as_mut_ptr(),
            output_buffer.len(),
            output_stride as c_int,
        )
    };
    if !result.is_null() {
        Ok(())
    } else {
        Err(WebPSimpleError)
    }
}

#[allow(non_snake_case)]
pub fn WebPDecodeARGBInto(
    data: &[u8],
    output_buffer: &mut [u8],
    output_stride: u32,
) -> Result<(), WebPSimpleError> {
    assert!(output_stride as c_int >= 0);
    assert_eq!(output_stride as c_int as u32, output_stride);
    let result = unsafe {
        sys::WebPDecodeARGBInto(
            data.as_ptr(),
            data.len(),
            output_buffer.as_mut_ptr(),
            output_buffer.len(),
            output_stride as c_int,
        )
    };
    if !result.is_null() {
        Ok(())
    } else {
        Err(WebPSimpleError)
    }
}

#[allow(non_snake_case)]
pub fn WebPDecodeBGRAInto(
    data: &[u8],
    output_buffer: &mut [u8],
    output_stride: u32,
) -> Result<(), WebPSimpleError> {
    assert!(output_stride as c_int >= 0);
    assert_eq!(output_stride as c_int as u32, output_stride);
    let result = unsafe {
        sys::WebPDecodeBGRAInto(
            data.as_ptr(),
            data.len(),
            output_buffer.as_mut_ptr(),
            output_buffer.len(),
            output_stride as c_int,
        )
    };
    if !result.is_null() {
        Ok(())
    } else {
        Err(WebPSimpleError)
    }
}

#[allow(non_snake_case)]
pub fn WebPDecodeRGBInto(
    data: &[u8],
    output_buffer: &mut [u8],
    output_stride: u32,
) -> Result<(), WebPSimpleError> {
    assert!(output_stride as c_int >= 0);
    assert_eq!(output_stride as c_int as u32, output_stride);
    let result = unsafe {
        sys::WebPDecodeRGBInto(
            data.as_ptr(),
            data.len(),
            output_buffer.as_mut_ptr(),
            output_buffer.len(),
            output_stride as c_int,
        )
    };
    if !result.is_null() {
        Ok(())
    } else {
        Err(WebPSimpleError)
    }
}

#[allow(non_snake_case)]
pub fn WebPDecodeBGRInto(
    data: &[u8],
    output_buffer: &mut [u8],
    output_stride: u32,
) -> Result<(), WebPSimpleError> {
    assert!(output_stride as c_int >= 0);
    assert_eq!(output_stride as c_int as u32, output_stride);
    let result = unsafe {
        sys::WebPDecodeBGRInto(
            data.as_ptr(),
            data.len(),
            output_buffer.as_mut_ptr(),
            output_buffer.len(),
            output_stride as c_int,
        )
    };
    if !result.is_null() {
        Ok(())
    } else {
        Err(WebPSimpleError)
    }
}

#[allow(non_snake_case)]
pub fn WebPDecodeYUVInto(
    data: &[u8],
    luma: &mut [u8],
    luma_stride: u32,
    u: &mut [u8],
    u_stride: u32,
    v: &mut [u8],
    v_stride: u32,
) -> Result<(), WebPSimpleError> {
    assert!(luma_stride as c_int >= 0);
    assert_eq!(luma_stride as c_int as u32, luma_stride);
    assert!(u_stride as c_int >= 0);
    assert_eq!(u_stride as c_int as u32, u_stride);
    assert!(v_stride as c_int >= 0);
    assert_eq!(v_stride as c_int as u32, v_stride);
    let result = unsafe {
        sys::WebPDecodeYUVInto(
            data.as_ptr(),
            data.len(),
            luma.as_mut_ptr(),
            luma.len(),
            luma_stride as c_int,
            u.as_mut_ptr(),
            u.len(),
            u_stride as c_int,
            v.as_mut_ptr(),
            v.len(),
            v_stride as c_int,
        )
    };
    if !result.is_null() {
        Ok(())
    } else {
        Err(WebPSimpleError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lena() -> Vec<u8> {
        include_bytes!("lena.webp").to_vec()
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_WebPDecodeRGBA() {
        let (width, height, buf) = WebPDecodeRGBA(&lena()).unwrap();
        assert_eq!(width, 128);
        assert_eq!(height, 128);
        assert_eq!(
            &buf[..24],
            &[
                226, 158, 113, 255, 226, 158, 113, 255, 226, 158, 113, 255, 226, 158, 113, 255,
                223, 155, 109, 255, 223, 155, 109, 255,
            ]
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_WebPDecodeARGB() {
        let (width, height, buf) = WebPDecodeARGB(&lena()).unwrap();
        assert_eq!(width, 128);
        assert_eq!(height, 128);
        assert_eq!(
            &buf[..24],
            &[
                255, 226, 158, 113, 255, 226, 158, 113, 255, 226, 158, 113, 255, 226, 158, 113,
                255, 223, 155, 109, 255, 223, 155, 109,
            ]
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_WebPDecodeBGRA() {
        let (width, height, buf) = WebPDecodeBGRA(&lena()).unwrap();
        assert_eq!(width, 128);
        assert_eq!(height, 128);
        assert_eq!(
            &buf[..24],
            &[
                113, 158, 226, 255, 113, 158, 226, 255, 113, 158, 226, 255, 113, 158, 226, 255,
                109, 155, 223, 255, 109, 155, 223, 255,
            ]
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_WebPDecodeRGB() {
        let (width, height, buf) = WebPDecodeRGB(&lena()).unwrap();
        assert_eq!(width, 128);
        assert_eq!(height, 128);
        assert_eq!(
            &buf[..24],
            &[
                226, 158, 113, 226, 158, 113, 226, 158, 113, 226, 158, 113, 223, 155, 109, 223,
                155, 109, 223, 155, 109, 223, 155, 109,
            ]
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_WebPDecodeBGR() {
        let (width, height, buf) = WebPDecodeBGR(&lena()).unwrap();
        assert_eq!(width, 128);
        assert_eq!(height, 128);
        assert_eq!(
            &buf[..24],
            &[
                113, 158, 226, 113, 158, 226, 113, 158, 226, 113, 158, 226, 109, 155, 223, 109,
                155, 223, 109, 155, 223, 109, 155, 223,
            ]
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_WebPDecodeYUV() {
        let (width, height, stride, uv_stride, buf) = WebPDecodeYUV(&lena()).unwrap();
        assert_eq!(width, 128);
        assert_eq!(height, 128);
        assert!(stride >= 128);
        assert!(uv_stride >= 64);
        assert_eq!(&buf.y()[..6], &[165, 165, 165, 165, 162, 162]);
        assert_eq!(&buf.u()[..6], &[98, 98, 98, 98, 98, 98]);
        assert_eq!(&buf.v()[..6], &[161, 161, 161, 161, 161, 161]);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_WebPDecodeRGBAInto() {
        let mut buf = vec![0; 128 * 128 * 4];
        WebPDecodeRGBAInto(&lena(), &mut buf, 128 * 4).unwrap();
        assert_eq!(
            &buf[..24],
            &[
                226, 158, 113, 255, 226, 158, 113, 255, 226, 158, 113, 255, 226, 158, 113, 255,
                223, 155, 109, 255, 223, 155, 109, 255,
            ]
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_WebPDecodeARGBInto() {
        let mut buf = vec![0; 128 * 128 * 4];
        WebPDecodeARGBInto(&lena(), &mut buf, 128 * 4).unwrap();
        assert_eq!(
            &buf[..24],
            &[
                255, 226, 158, 113, 255, 226, 158, 113, 255, 226, 158, 113, 255, 226, 158, 113,
                255, 223, 155, 109, 255, 223, 155, 109,
            ]
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_WebPDecodeBGRAInto() {
        let mut buf = vec![0; 128 * 128 * 4];
        WebPDecodeBGRAInto(&lena(), &mut buf, 128 * 4).unwrap();
        assert_eq!(
            &buf[..24],
            &[
                113, 158, 226, 255, 113, 158, 226, 255, 113, 158, 226, 255, 113, 158, 226, 255,
                109, 155, 223, 255, 109, 155, 223, 255,
            ]
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_WebPDecodeRGBInto() {
        let mut buf = vec![0; 128 * 128 * 3];
        WebPDecodeRGBInto(&lena(), &mut buf, 128 * 3).unwrap();
        assert_eq!(
            &buf[..24],
            &[
                226, 158, 113, 226, 158, 113, 226, 158, 113, 226, 158, 113, 223, 155, 109, 223,
                155, 109, 223, 155, 109, 223, 155, 109,
            ]
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_WebPDecodeBGRInto() {
        let mut buf = vec![0; 128 * 128 * 3];
        WebPDecodeBGRInto(&lena(), &mut buf, 128 * 3).unwrap();
        assert_eq!(
            &buf[..24],
            &[
                113, 158, 226, 113, 158, 226, 113, 158, 226, 113, 158, 226, 109, 155, 223, 109,
                155, 223, 109, 155, 223, 109, 155, 223
            ],
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_WebPDecodeYUVInto() {
        let mut luma = vec![0; 128 * 128];
        let mut u = vec![0; 64 * 64];
        let mut v = vec![0; 64 * 64];
        WebPDecodeYUVInto(&lena(), &mut luma, 128, &mut u, 64, &mut v, 64).unwrap();
        assert_eq!(&luma[..6], &[165, 165, 165, 165, 162, 162]);
        assert_eq!(&u[..6], &[98, 98, 98, 98, 98, 98]);
        assert_eq!(&v[..6], &[161, 161, 161, 161, 161, 161]);
    }
}
