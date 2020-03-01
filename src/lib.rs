use std::slice;

use crate::boxed::WebpBox;
pub use crate::decode::*;
pub use crate::encode::*;
use crate::error::WebPSimpleError;

pub mod boxed;
mod decode;
mod encode;
pub mod error;

#[inline]
unsafe fn wrap_bytes<F>(ptr: *mut u8, get_len: F) -> Result<WebpBox<[u8]>, WebPSimpleError>
where
    F: FnOnce() -> usize,
{
    if !ptr.is_null() {
        let len = get_len();
        Ok(WebpBox::from_raw(slice::from_raw_parts_mut(ptr, len)))
    } else {
        Err(WebPSimpleError)
    }
}
