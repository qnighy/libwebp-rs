use libwebp_sys as sys;
use std::marker::{PhantomPinned, Unpin};
use std::mem;
use std::os::raw::*;
use std::panic::{RefUnwindSafe, UnwindSafe};
use std::pin::Pin;
use std::ptr::{self, NonNull};
use std::slice;

use crate::boxed::{wrap_bytes, WebpBox, WebpYuvBox};
use crate::error::WebPSimpleError;

/// Return the decoder's version number, packed in hexadecimal using 8bits for
/// each of major/minor/revision.
///
/// E.g: v2.5.7 is `0x020507`.
///
/// ## Examples
///
/// ```rust
/// use libwebp::WebPGetDecoderVersion;
///
/// let version = WebPGetDecoderVersion();
/// ```
#[allow(non_snake_case)]
pub fn WebPGetDecoderVersion() -> u32 {
    (unsafe { sys::WebPGetDecoderVersion() }) as u32
}

/// Retrieve basic header information: width, height.
///
/// This function will also validate the header, returning
/// `Ok((width, height))` on success, `Err(_)` otherwise.
///
/// ## Errors
///
/// Returns `Err` if `data` doesn't contain a valid WebP header.
///
/// ## Examples
///
/// ```rust
/// use libwebp::WebPGetInfo;
///
/// let data: &[u8];
/// # let data: &[u8] = include_bytes!("lena.webp");
///
/// let (width, height) = WebPGetInfo(data).expect("Invalid WebP data");
/// # assert_eq!((width, height), (128, 128));
/// ```
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

/// Decodes WebP images pointed to by `data` and returns RGBA samples, along
/// with the dimensions (width and height).
///
/// The ordering of samples in memory is R, G, B, A, R, G, B, A... in scan
/// order (endian-independent).
///
/// ## Errors
///
/// Returns `Err` if `data` doesn't contain a valid WebP image.
///
/// ## Variants
///
/// - `WebPDecodeRGBA`
/// - [`WebPDecodeARGB`]
/// - [`WebPDecodeBGRA`]
/// - [`WebPDecodeRGB`]
/// - [`WebPDecodeBGR`]
///
/// [`WebPDecodeARGB`]: fn.WebPDecodeARGB.html
/// [`WebPDecodeBGRA`]: fn.WebPDecodeBGRA.html
/// [`WebPDecodeRGB`]: fn.WebPDecodeRGB.html
/// [`WebPDecodeBGR`]: fn.WebPDecodeBGR.html
///
/// ## Examples
///
/// ```rust
/// use libwebp::WebPDecodeRGBA;
///
/// let data: &[u8];
/// # let data: &[u8] = include_bytes!("lena.webp");
///
/// let (width, height, buf) = WebPDecodeRGBA(data).expect("Invalid WebP data");
/// # assert_eq!((width, height), (128, 128));
/// assert_eq!(buf.len(), width as usize * height as usize * 4);
/// eprintln!(
///     "top-left pixel: rgba({}, {}, {}, {})",
///     buf[0],
///     buf[1],
///     buf[2],
///     buf[3] as f64 / 255.0,
/// )
/// ```
#[allow(non_snake_case)]
pub fn WebPDecodeRGBA(data: &[u8]) -> Result<(u32, u32, WebpBox<[u8]>), WebPSimpleError> {
    let mut width: c_int = 0;
    let mut height: c_int = 0;
    let result = unsafe { sys::WebPDecodeRGBA(data.as_ptr(), data.len(), &mut width, &mut height) };
    let buf = (unsafe { wrap_bytes(result, || width as usize * height as usize * 4) })?;
    Ok((width as u32, height as u32, buf))
}

/// Same as [`WebPDecodeRGBA`], but returning A, R, G, B, A, R, G, B...
/// ordered data.
///
/// [`WebPDecodeRGBA`]: fn.WebPDecodeRGBA.html
#[allow(non_snake_case)]
pub fn WebPDecodeARGB(data: &[u8]) -> Result<(u32, u32, WebpBox<[u8]>), WebPSimpleError> {
    let mut width: c_int = 0;
    let mut height: c_int = 0;
    let result = unsafe { sys::WebPDecodeARGB(data.as_ptr(), data.len(), &mut width, &mut height) };
    let buf = (unsafe { wrap_bytes(result, || width as usize * height as usize * 4) })?;
    Ok((width as u32, height as u32, buf))
}

/// Same as [`WebPDecodeRGBA`], but returning B, G, R, A, B, G, R, A...
/// ordered data.
///
/// [`WebPDecodeRGBA`]: fn.WebPDecodeRGBA.html
#[allow(non_snake_case)]
pub fn WebPDecodeBGRA(data: &[u8]) -> Result<(u32, u32, WebpBox<[u8]>), WebPSimpleError> {
    let mut width: c_int = 0;
    let mut height: c_int = 0;
    let result = unsafe { sys::WebPDecodeBGRA(data.as_ptr(), data.len(), &mut width, &mut height) };
    let buf = (unsafe { wrap_bytes(result, || width as usize * height as usize * 4) })?;
    Ok((width as u32, height as u32, buf))
}

/// Same as [`WebPDecodeRGBA`], but returning R, G, B, R, G, B... ordered data.
///
/// If the bitstream contains transparency, it is ignored.
///
/// [`WebPDecodeRGBA`]: fn.WebPDecodeRGBA.html
#[allow(non_snake_case)]
pub fn WebPDecodeRGB(data: &[u8]) -> Result<(u32, u32, WebpBox<[u8]>), WebPSimpleError> {
    let mut width: c_int = 0;
    let mut height: c_int = 0;
    let result = unsafe { sys::WebPDecodeRGB(data.as_ptr(), data.len(), &mut width, &mut height) };
    let buf = (unsafe { wrap_bytes(result, || width as usize * height as usize * 3) })?;
    Ok((width as u32, height as u32, buf))
}

/// Same as [`WebPDecodeRGBA`], but returning B, G, R, B, G, R... ordered data.
///
/// If the bitstream contains transparency, it is ignored.
///
/// [`WebPDecodeRGBA`]: fn.WebPDecodeRGBA.html
#[allow(non_snake_case)]
pub fn WebPDecodeBGR(data: &[u8]) -> Result<(u32, u32, WebpBox<[u8]>), WebPSimpleError> {
    let mut width: c_int = 0;
    let mut height: c_int = 0;
    let result = unsafe { sys::WebPDecodeBGR(data.as_ptr(), data.len(), &mut width, &mut height) };
    let buf = (unsafe { wrap_bytes(result, || width as usize * height as usize * 3) })?;
    Ok((width as u32, height as u32, buf))
}

/// Decodes WebP images pointed to by `data` to Y'UV format[^1].
///
/// [^1] Also named Y'CbCr. See: [http://en.wikipedia.org/wiki/YCbCr](http://en.wikipedia.org/wiki/YCbCr)
///
/// ## Return value
///
/// It retuns a tuple with the following data in this order:
///
/// - width
/// - height
/// - stride
/// - uv\_stride
/// - a [`WebpYuvBox`] which contains the pointers to the Y, U and V planes.
///
/// [`WebpYuvBox`]: boxed/struct.WebPYuvBox.html
///
/// The dimension of the U and V planes are both `(width + 1) / 2` and `(height + 1)/ 2`.
/// The Y buffer has a stride returned as `stride`, while U and V
/// have a common stride returned as `uv_stride`.
///
/// ## Errors
///
/// Returns `Err` if `data` doesn't contain a valid WebP image.
///
/// ## Examples
///
/// ```rust
/// use libwebp::WebPDecodeYUV;
///
/// let data: &[u8];
/// # let data: &[u8] = include_bytes!("lena.webp");
///
/// let (width, height, stride, uv_stride, buf) =
///     WebPDecodeYUV(data).expect("Invalid WebP data");
/// # assert_eq!((width, height), (128, 128));
/// assert!(width <= stride);
/// assert!((width + 1) / 2 <= uv_stride);
/// assert_eq!(buf.y().len(), stride as usize * height as usize);
/// assert_eq!(buf.u().len(), uv_stride as usize * ((height as usize + 1) / 2));
/// assert_eq!(buf.v().len(), uv_stride as usize * ((height as usize + 1) / 2));
/// eprintln!(
///     "top-left pixel: yuv({}, {}, {})",
///     buf.y()[0],
///     buf.u()[0],
///     buf.v()[0],
/// )
/// ```
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

/// Decodes WebP images pointed to by `data` and writes RGBA samples to
/// `output_buffer`.
///
/// The parameter 'output_stride' specifies the distance (in bytes)
/// between scanlines. Hence, `output_buffer.len()` is expected to be at least
/// `output_stride` x `picture-height`.
///
/// The ordering of samples in memory is R, G, B, A, R, G, B, A... in scan
/// order (endian-independent).
///
/// ## Errors
///
/// Returns `Err` if `data` doesn't contain a valid WebP image, or
/// `output_buffer` is too small.
///
/// ## Variants
///
/// - `WebPDecodeRGBAInto`
/// - [`WebPDecodeARGBInto`]
/// - [`WebPDecodeBGRAInto`]
/// - [`WebPDecodeRGBInto`]
/// - [`WebPDecodeBGRInto`]
///
/// [`WebPDecodeARGBInto`]: fn.WebPDecodeARGBInto.html
/// [`WebPDecodeBGRAInto`]: fn.WebPDecodeBGRAInto.html
/// [`WebPDecodeRGBInto`]: fn.WebPDecodeRGBInto.html
/// [`WebPDecodeBGRInto`]: fn.WebPDecodeBGRInto.html
///
/// ## Examples
///
/// ```rust
/// use libwebp::{WebPGetInfo, WebPDecodeRGBAInto};
///
/// let data: &[u8];
/// # let data: &[u8] = include_bytes!("lena.webp");
///
/// let (width, height) = WebPGetInfo(data).expect("Invalid WebP header");
/// # assert_eq!((width, height), (128, 128));
/// let stride = width * 4;
/// let mut buf = vec![0; stride as usize * height as usize];
/// WebPDecodeRGBAInto(data, &mut buf, stride).expect("Invalid WebP data");
/// eprintln!(
///     "top-left pixel: rgba({}, {}, {}, {})",
///     buf[0],
///     buf[1],
///     buf[2],
///     buf[3] as f64 / 255.0,
/// )
/// ```
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

/// Same as [`WebPDecodeRGBAInto`], but returning A, R, G, B, A, R, G, B...
/// ordered data.
///
/// [`WebPDecodeRGBAInto`]: fn.WebPDecodeRGBAInto.html
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

/// Same as [`WebPDecodeRGBAInto`], but returning B, G, R, A, B, G, R, A...
/// ordered data.
///
/// [`WebPDecodeRGBAInto`]: fn.WebPDecodeRGBAInto.html
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

/// Same as [`WebPDecodeRGBAInto`], but returning R, G, B, R, G, B...
/// ordered data.
///
/// If the bitstream contains transparency, it is ignored.
///
/// [`WebPDecodeRGBAInto`]: fn.WebPDecodeRGBAInto.html
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

/// Same as [`WebPDecodeRGBAInto`], but returning B, G, R, B, G, R...
/// ordered data.
///
/// If the bitstream contains transparency, it is ignored.
///
/// [`WebPDecodeRGBAInto`]: fn.WebPDecodeRGBAInto.html
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

/// A variant of [`WebPDecodeYUVInto`] that operates directly into
/// pre-allocated buffers.
///
/// [`WebPDecodeYUVInto`]: fn.WebPDecodeYUVInto.html
///
/// ## Errors
///
/// Returns `Err` if `data` doesn't contain a valid WebP image, or
/// any of the buffers is too small.
///
/// ## Examples
///
/// ```rust
/// use libwebp::{WebPGetInfo, WebPDecodeYUVInto};
///
/// let data: &[u8];
/// # let data: &[u8] = include_bytes!("lena.webp");
///
/// let (width, height) = WebPGetInfo(data).expect("Invalid WebP header");
/// # assert_eq!((width, height), (128, 128));
/// let luma_stride = width;
/// let u_stride = (width + 1) / 2;
/// let v_stride = (width + 1) / 2;
/// let mut luma = vec![0; luma_stride as usize * height as usize];
/// let mut u = vec![0; u_stride as usize * ((height + 1) / 2) as usize];
/// let mut v = vec![0; v_stride as usize * ((height + 1) / 2) as usize];
///
/// WebPDecodeYUVInto(
///     data,
///     &mut luma,
///     luma_stride,
///     &mut u,
///     u_stride,
///     &mut v,
///     v_stride,
/// ).expect("Invalid WebP header");
/// eprintln!(
///     "top-left pixel: yuv({}, {}, {})",
///     luma[0],
///     u[0],
///     v[0],
/// )
/// ```
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

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WEBP_CSP_MODE {
    MODE_RGB = 0,
    MODE_RGBA = 1,
    MODE_BGR = 2,
    MODE_BGRA = 3,
    MODE_ARGB = 4,
    MODE_RGBA_4444 = 5,
    MODE_RGB_565 = 6,
    /// A variant of `MODE_RGBA` with premultiplied alpha
    MODE_rgbA = 7,
    /// A variant of `MODE_BGRA` with premultiplied alpha
    MODE_bgrA = 8,
    /// A variant of `MODE_ARGB` with premultiplied alpha
    MODE_Argb = 9,
    /// A variant of `MODE_RGBA_4444` with premultiplied alpha
    MODE_rgbA_4444 = 10,
    // YUV modes must come after RGB ones.
    MODE_YUV = 11,
    MODE_YUVA = 12, // yuv 4:2:0
}

impl WEBP_CSP_MODE {
    pub fn from_raw(raw: sys::WEBP_CSP_MODE) -> Self {
        use self::WEBP_CSP_MODE::*;

        match raw {
            sys::MODE_RGB => MODE_RGB,
            sys::MODE_RGBA => MODE_RGBA,
            sys::MODE_BGR => MODE_BGR,
            sys::MODE_BGRA => MODE_BGRA,
            sys::MODE_ARGB => MODE_ARGB,
            sys::MODE_RGBA_4444 => MODE_RGBA_4444,
            sys::MODE_RGB_565 => MODE_RGB_565,
            sys::MODE_rgbA => MODE_rgbA,
            sys::MODE_bgrA => MODE_bgrA,
            sys::MODE_Argb => MODE_Argb,
            sys::MODE_rgbA_4444 => MODE_rgbA_4444,
            sys::MODE_YUV => MODE_YUV,
            sys::MODE_YUVA => MODE_YUVA,
            _ => panic!("WEBP_CSP_MODE::from_raw: unknown value {:?}", raw),
        }
    }

    pub fn into_raw(self) -> sys::WEBP_CSP_MODE {
        use self::WEBP_CSP_MODE::*;

        match self {
            MODE_RGB => sys::MODE_RGB,
            MODE_RGBA => sys::MODE_RGBA,
            MODE_BGR => sys::MODE_BGR,
            MODE_BGRA => sys::MODE_BGRA,
            MODE_ARGB => sys::MODE_ARGB,
            MODE_RGBA_4444 => sys::MODE_RGBA_4444,
            MODE_RGB_565 => sys::MODE_RGB_565,
            MODE_rgbA => sys::MODE_rgbA,
            MODE_bgrA => sys::MODE_bgrA,
            MODE_Argb => sys::MODE_Argb,
            MODE_rgbA_4444 => sys::MODE_rgbA_4444,
            MODE_YUV => sys::MODE_YUV,
            MODE_YUVA => sys::MODE_YUVA,
        }
    }
}

#[allow(non_snake_case)]
pub fn WebPIsPremultipliedMode(mode: WEBP_CSP_MODE) -> bool {
    use self::WEBP_CSP_MODE::*;

    match mode {
        MODE_rgbA | MODE_bgrA | MODE_Argb | MODE_rgbA_4444 => true,
        MODE_RGB | MODE_RGBA | MODE_BGR | MODE_BGRA | MODE_ARGB | MODE_RGBA_4444 | MODE_RGB_565
        | MODE_YUV | MODE_YUVA => false,
    }
}

#[allow(non_snake_case)]
pub fn WebPIsAlphaMode(mode: WEBP_CSP_MODE) -> bool {
    use self::WEBP_CSP_MODE::*;

    match mode {
        MODE_RGBA | MODE_BGRA | MODE_ARGB | MODE_RGBA_4444 | MODE_rgbA | MODE_bgrA | MODE_Argb
        | MODE_rgbA_4444 | MODE_YUVA => true,
        MODE_RGB | MODE_BGR | MODE_RGB_565 | MODE_YUV => false,
    }
}

#[allow(non_snake_case)]
pub fn WebPIsRGBMode(mode: WEBP_CSP_MODE) -> bool {
    use self::WEBP_CSP_MODE::*;

    match mode {
        MODE_RGB | MODE_RGBA | MODE_BGR | MODE_BGRA | MODE_ARGB | MODE_RGBA_4444 | MODE_RGB_565
        | MODE_rgbA | MODE_bgrA | MODE_Argb | MODE_rgbA_4444 => true,
        MODE_YUV | MODE_YUVA => false,
    }
}

// #[derive(Debug)]
// pub struct WebPDecBuffer(sys::WebPDecBuffer);
//
// unsafe impl Send for WebPDecBuffer {}
// unsafe impl Sync for WebPDecBuffer {}
//
// impl Drop for WebPDecBuffer {
//     fn drop(&mut self) {
//         unsafe {
//             sys::WebPFreeDecBuffer(&mut self.0);
//         }
//     }
// }
//
// impl WebPDecBuffer {
//     pub unsafe fn from_raw(raw: sys::WebPDecBuffer) -> Self {
//         debug_assert_eq!(raw.is_external_memory, 0, "is_external_memory should be 0");
//         WebPDecBuffer(raw)
//     }
//
//     pub fn into_raw(self) -> sys::WebPDecBuffer {
//         let ret = unsafe { ptr::read(&self.0) };
//         mem::forget(self);
//         ret
//     }
//
//     pub fn colorspace(&self) -> WEBP_CSP_MODE {
//         WEBP_CSP_MODE::from_raw(self.0.colorspace)
//     }
//
//     pub fn set_colorspace(&mut self, colorspace: WEBP_CSP_MODE) {
//         self.0.colorspace = colorspace.into_raw();
//     }
//
//     pub fn width(&self) -> u32 {
//         self.0.width as u32
//     }
//
//     pub fn set_width(&mut self, width: u32) {
//         assert!(width as c_int >= 0);
//         assert_eq!(width as c_int as u32, width);
//         self.0.width = width as c_int;
//     }
//
//     pub fn height(&self) -> u32 {
//         self.0.height as u32
//     }
//
//     pub fn set_height(&mut self, height: u32) {
//         assert!(height as c_int >= 0);
//         assert_eq!(height as c_int as u32, height);
//         self.0.height = height as c_int;
//     }
// }
//
// #[allow(non_snake_case)]
// pub fn WebPInitDecBuffer() -> WebPDecBuffer {
//     let mut buf: sys::WebPDecBuffer = unsafe { mem::zeroed() };
//     let result = unsafe { sys::WebPInitDecBuffer(&mut buf) };
//     if result != 0 {
//         unsafe { WebPDecBuffer::from_raw(buf) }
//     } else {
//         panic!("libwebp version mismatch")
//     }
// }

#[allow(non_camel_case_types)]
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VP8StatusCode {
    VP8_STATUS_OK = 0,
    VP8_STATUS_OUT_OF_MEMORY = 1,
    VP8_STATUS_INVALID_PARAM = 2,
    VP8_STATUS_BITSTREAM_ERROR = 3,
    VP8_STATUS_UNSUPPORTED_FEATURE = 4,
    VP8_STATUS_SUSPENDED = 5,
    VP8_STATUS_USER_ABORT = 6,
    VP8_STATUS_NOT_ENOUGH_DATA = 7,
}

impl VP8StatusCode {
    pub fn from_raw(raw: sys::VP8StatusCode) -> Self {
        use self::VP8StatusCode::*;

        match raw {
            sys::VP8_STATUS_OK => VP8_STATUS_OK,
            sys::VP8_STATUS_OUT_OF_MEMORY => VP8_STATUS_OUT_OF_MEMORY,
            sys::VP8_STATUS_INVALID_PARAM => VP8_STATUS_INVALID_PARAM,
            sys::VP8_STATUS_BITSTREAM_ERROR => VP8_STATUS_BITSTREAM_ERROR,
            sys::VP8_STATUS_UNSUPPORTED_FEATURE => VP8_STATUS_UNSUPPORTED_FEATURE,
            sys::VP8_STATUS_SUSPENDED => VP8_STATUS_SUSPENDED,
            sys::VP8_STATUS_USER_ABORT => VP8_STATUS_USER_ABORT,
            sys::VP8_STATUS_NOT_ENOUGH_DATA => VP8_STATUS_NOT_ENOUGH_DATA,
            _ => panic!("VP8StatusCode::from_raw: unknown value {:?}", raw),
        }
    }

    pub fn into_raw(self) -> sys::WEBP_CSP_MODE {
        use self::VP8StatusCode::*;

        match self {
            VP8_STATUS_OK => sys::VP8_STATUS_OK,
            VP8_STATUS_OUT_OF_MEMORY => sys::VP8_STATUS_OUT_OF_MEMORY,
            VP8_STATUS_INVALID_PARAM => sys::VP8_STATUS_INVALID_PARAM,
            VP8_STATUS_BITSTREAM_ERROR => sys::VP8_STATUS_BITSTREAM_ERROR,
            VP8_STATUS_UNSUPPORTED_FEATURE => sys::VP8_STATUS_UNSUPPORTED_FEATURE,
            VP8_STATUS_SUSPENDED => sys::VP8_STATUS_SUSPENDED,
            VP8_STATUS_USER_ABORT => sys::VP8_STATUS_USER_ABORT,
            VP8_STATUS_NOT_ENOUGH_DATA => sys::VP8_STATUS_NOT_ENOUGH_DATA,
        }
    }
}

#[repr(transparent)]
pub struct WebPIDecoder(sys::WebPIDecoder, PhantomPinned);

unsafe impl Send for WebPIDecoder {}
unsafe impl Sync for WebPIDecoder {}
impl UnwindSafe for WebPIDecoder {}
impl RefUnwindSafe for WebPIDecoder {}

impl std::fmt::Debug for WebPIDecoder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("WebPIDecoder")
    }
}

impl WebPIDecoder {
    pub unsafe fn from_ptr<'a>(raw: *const sys::WebPIDecoder) -> Pin<&'a Self> {
        Pin::new_unchecked(&*(raw as *const WebPIDecoder))
    }

    pub unsafe fn from_mut_ptr<'a>(raw: *mut sys::WebPIDecoder) -> Pin<&'a mut Self> {
        Pin::new_unchecked(&mut *(raw as *mut WebPIDecoder))
    }

    pub fn as_ptr(self: Pin<&Self>) -> *const sys::WebPIDecoder {
        &self.0
    }

    pub fn as_mut_ptr(self: Pin<&mut Self>) -> *mut sys::WebPIDecoder {
        unsafe { &mut self.get_unchecked_mut().0 }
    }
}

#[derive(Debug)]
pub struct WebPIDecoderBox(NonNull<WebPIDecoder>);

unsafe impl Send for WebPIDecoderBox {}
unsafe impl Sync for WebPIDecoderBox {}
impl UnwindSafe for WebPIDecoderBox {}
impl RefUnwindSafe for WebPIDecoderBox {}
// Prior to 1.38.0 it isn't automatically Unpin
impl Unpin for WebPIDecoderBox {}

impl Drop for WebPIDecoderBox {
    fn drop(&mut self) {
        unsafe {
            sys::WebPIDelete(self.0.as_ptr() as *mut sys::WebPIDecoder);
        }
    }
}

impl WebPIDecoderBox {
    pub unsafe fn from_raw(raw: NonNull<sys::WebPIDecoder>) -> Self {
        WebPIDecoderBox(raw.cast::<WebPIDecoder>())
    }

    pub fn into_raw(self) -> NonNull<sys::WebPIDecoder> {
        let ret = self.0;
        mem::forget(self);
        ret.cast::<sys::WebPIDecoder>()
    }

    pub fn as_ref(&self) -> Pin<&WebPIDecoder> {
        unsafe { Pin::new_unchecked(self.0.as_ref()) }
    }

    pub fn as_mut(&mut self) -> Pin<&mut WebPIDecoder> {
        unsafe { Pin::new_unchecked(self.0.as_mut()) }
    }
}

// TODO: Implment external version
#[allow(non_snake_case)]
pub fn WebPINewDecoder() -> WebPIDecoderBox {
    let result = unsafe { sys::WebPINewDecoder(ptr::null_mut()) };
    if let Some(result) = NonNull::new(result) {
        unsafe { WebPIDecoderBox::from_raw(result) }
    } else {
        panic!("WebPINewDecoder: allocation failed");
    }
}

// TODO: Implment external version
#[allow(non_snake_case)]
pub fn WebPINewRGB(csp: WEBP_CSP_MODE) -> WebPIDecoderBox {
    assert!(WebPIsRGBMode(csp), "Not an RGB mode: {:?}", csp);
    let result = unsafe { sys::WebPINewRGB(csp.into_raw(), ptr::null_mut(), 0, 0) };
    if let Some(result) = NonNull::new(result) {
        unsafe { WebPIDecoderBox::from_raw(result) }
    } else {
        panic!("WebPINewRGB: allocation failed");
    }
}

// TODO: Implment external version
#[allow(non_snake_case)]
pub fn WebPINewYUVA() -> WebPIDecoderBox {
    let result = unsafe {
        sys::WebPINewYUVA(
            ptr::null_mut(),
            0,
            0,
            ptr::null_mut(),
            0,
            0,
            ptr::null_mut(),
            0,
            0,
            ptr::null_mut(),
            0,
            0,
        )
    };
    if let Some(result) = NonNull::new(result) {
        unsafe { WebPIDecoderBox::from_raw(result) }
    } else {
        panic!("WebPINewYUVA: allocation failed");
    }
}

#[allow(non_snake_case)]
pub fn WebPIAppend(idec: Pin<&mut WebPIDecoder>, data: &[u8]) -> VP8StatusCode {
    if data.is_empty() {
        // libwebp AppendToMemBuffer (src/dec/idec_dec.c) doesn't expect empty slice at the beginning.
        // Since the decoder status cannot be observed otherwise, we just panic here for now.
        panic!("WebPIAppend: appending an empty slice is not supported for now");
    }
    let result = unsafe { sys::WebPIAppend(idec.as_mut_ptr(), data.as_ptr(), data.len()) };
    VP8StatusCode::from_raw(result)
}

// TODO: check if it's safe to inconsistently pass data into WebPIUpdate
// #[allow(non_snake_case)]
// pub fn WebPIUpdate(idec: Pin<&mut WebPIDecoder>, data: &[u8]) -> VP8StatusCode {
//     let result = unsafe { sys::WebPIUpdate(idec.as_mut_ptr(), data.as_ptr(), data.len()) };
//     VP8StatusCode::from_raw(result)
// }

#[derive(Debug)]
pub struct WebPIDecGetRGBResult<'a> {
    pub buf: &'a [u8],
    pub last_y: u32,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
}

#[allow(non_snake_case)]
pub fn WebPIDecGetRGB(
    idec: Pin<&WebPIDecoder>,
) -> Result<WebPIDecGetRGBResult<'_>, WebPSimpleError> {
    let mut last_y: c_int = 0;
    let mut width: c_int = 0;
    let mut height: c_int = 0;
    let mut stride: c_int = 0;
    let result = unsafe {
        sys::WebPIDecGetRGB(
            idec.as_ptr(),
            &mut last_y,
            &mut width,
            &mut height,
            &mut stride,
        )
    };
    if !result.is_null() {
        // TODO: can this be stride * height?
        let len = stride as usize * last_y as usize;
        let buf = unsafe { slice::from_raw_parts(result, len) };
        Ok(WebPIDecGetRGBResult {
            buf,
            last_y: last_y as u32,
            width: width as u32,
            height: height as u32,
            stride: stride as u32,
        })
    } else {
        Err(WebPSimpleError)
    }
}

#[derive(Debug)]
pub struct WebPIDecGetYUVAResult<'a> {
    pub luma: &'a [u8],
    pub last_y: u32,
    pub u: &'a [u8],
    pub v: &'a [u8],
    pub a: Option<&'a [u8]>,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub uv_stride: u32,
    pub a_stride: u32,
}

#[allow(non_snake_case)]
pub fn WebPIDecGetYUVA(
    idec: Pin<&WebPIDecoder>,
) -> Result<WebPIDecGetYUVAResult<'_>, WebPSimpleError> {
    let mut last_y: c_int = 0;
    let mut u: *mut u8 = ptr::null_mut();
    let mut v: *mut u8 = ptr::null_mut();
    let mut a: *mut u8 = ptr::null_mut();
    let mut width: c_int = 0;
    let mut height: c_int = 0;
    let mut stride: c_int = 0;
    let mut uv_stride: c_int = 0;
    let mut a_stride: c_int = 0;
    let result = unsafe {
        sys::WebPIDecGetYUVA(
            idec.as_ptr(),
            &mut last_y,
            &mut u,
            &mut v,
            &mut a,
            &mut width,
            &mut height,
            &mut stride,
            &mut uv_stride,
            &mut a_stride,
        )
    };
    if !result.is_null() {
        // TODO: can this be stride * height?
        let luma_len = stride as usize * last_y as usize;
        let luma = unsafe { slice::from_raw_parts(result, luma_len) };
        // TODO: can this be uv_stride * ((height + 1) / 2)?
        let uv_len = uv_stride as usize * ((last_y as usize + 1) / 2);
        let u = unsafe { slice::from_raw_parts(u as *const u8, uv_len) };
        let v = unsafe { slice::from_raw_parts(v as *const u8, uv_len) };
        // TODO: can this be a_stride * height?
        let a = if !a.is_null() {
            let a_len = a_stride as usize * last_y as usize;
            Some(unsafe { slice::from_raw_parts(a as *const u8, a_len) })
        } else {
            None
        };
        Ok(WebPIDecGetYUVAResult {
            luma,
            last_y: last_y as u32,
            u,
            v,
            a,
            width: width as u32,
            height: height as u32,
            stride: stride as u32,
            uv_stride: uv_stride as u32,
            a_stride: a_stride as u32,
        })
    } else {
        Err(WebPSimpleError)
    }
}

#[cfg(test)]
mod tests {
    use rand::prelude::*;

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

    #[test]
    fn test_auto_traits() {
        use std::marker::PhantomData;

        struct Test1<T: ?Sized>(Test2<T>);
        struct Test2<T: ?Sized>(PhantomData<T>);
        impl<T: ?Sized> Test1<T> {
            fn new() -> Self {
                Test1(Test2(PhantomData))
            }
        }
        impl<T: ?Sized> std::ops::Deref for Test1<T> {
            type Target = Test2<T>;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl<T: ?Sized + Unpin> Test1<T> {
            fn is_unpin(&self) -> bool {
                true
            }
        }
        impl<T: ?Sized> Test2<T> {
            fn is_unpin(&self) -> bool {
                false
            }
        }

        fn is_send<T: ?Sized + Send>() {}
        fn is_sync<T: ?Sized + Sync>() {}
        fn is_unwind_safe<T: ?Sized + UnwindSafe>() {}
        fn is_ref_unwind_safe<T: ?Sized + RefUnwindSafe>() {}

        is_send::<WebPIDecoderBox>();
        is_sync::<WebPIDecoderBox>();
        is_unwind_safe::<WebPIDecoderBox>();
        is_ref_unwind_safe::<WebPIDecoderBox>();
        assert!(Test1::<WebPIDecoderBox>::new().is_unpin());

        is_send::<WebPIDecoder>();
        is_sync::<WebPIDecoder>();
        is_unwind_safe::<WebPIDecoder>();
        is_ref_unwind_safe::<WebPIDecoder>();
        assert!(!Test1::<WebPIDecoder>::new().is_unpin());
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_WebPINewDecoder() {
        let _idec = WebPINewDecoder();
    }

    #[test]
    fn test_incr_argb() {
        let data = lena();
        let mut rng = rand::thread_rng();
        for _ in 0..50 {
            let mut idec = WebPINewRGB(WEBP_CSP_MODE::MODE_ARGB);
            let mut idx = 0;
            while idx < data.len() {
                // TODO: include 0 as write_len
                let write_len = std::cmp::min(rng.gen_range(1, 64), data.len() - idx);
                let result = WebPIAppend(idec.as_mut(), &data[idx..idx + write_len]);
                idx += write_len;
                if result == VP8StatusCode::VP8_STATUS_OK {
                    break;
                } else if result == VP8StatusCode::VP8_STATUS_SUSPENDED {
                    if let Ok(result) = WebPIDecGetRGB(idec.as_ref()) {
                        if result.last_y >= 1 {
                            assert_eq!(
                                &result.buf[..24],
                                &[
                                    255, 226, 158, 113, 255, 226, 158, 113, 255, 226, 158, 113,
                                    255, 226, 158, 113, 255, 223, 155, 109, 255, 223, 155, 109,
                                ]
                            );
                        }
                    }
                } else {
                    panic!("Unexpected status: {:?}", result);
                }
            }
            let result = WebPIDecGetRGB(idec.as_ref()).unwrap();
            assert_eq!(result.width, 128);
            assert_eq!(result.height, 128);
            assert_eq!(result.last_y, 128);
            assert_eq!(
                &result.buf[..24],
                &[
                    255, 226, 158, 113, 255, 226, 158, 113, 255, 226, 158, 113, 255, 226, 158, 113,
                    255, 223, 155, 109, 255, 223, 155, 109,
                ]
            );
        }
    }

    #[test]
    fn test_incr_yuva() {
        let data = lena();
        let mut rng = rand::thread_rng();
        for _ in 0..50 {
            let mut idec = WebPINewYUVA();
            let mut idx = 0;
            while idx < data.len() {
                // TODO: include 0 as write_len
                let write_len = std::cmp::min(rng.gen_range(1, 64), data.len() - idx);
                let result = WebPIAppend(idec.as_mut(), &data[idx..idx + write_len]);
                idx += write_len;
                if result == VP8StatusCode::VP8_STATUS_OK {
                    break;
                } else if result == VP8StatusCode::VP8_STATUS_SUSPENDED {
                    if let Ok(result) = WebPIDecGetYUVA(idec.as_ref()) {
                        if result.last_y >= 1 {
                            assert_eq!(&result.luma[..6], &[165, 165, 165, 165, 162, 162]);
                            assert_eq!(&result.u[..6], &[98, 98, 98, 98, 98, 98]);
                            assert_eq!(&result.v[..6], &[161, 161, 161, 161, 161, 161]);
                        }
                    }
                } else {
                    panic!("Unexpected status: {:?}", result);
                }
            }
            let result = WebPIDecGetYUVA(idec.as_ref()).unwrap();
            assert_eq!(result.width, 128);
            assert_eq!(result.height, 128);
            assert_eq!(result.last_y, 128);
            assert_eq!(&result.luma[..6], &[165, 165, 165, 165, 162, 162]);
            assert_eq!(&result.u[..6], &[98, 98, 98, 98, 98, 98]);
            assert_eq!(&result.v[..6], &[161, 161, 161, 161, 161, 161]);
        }
    }
}
