//! # libwebp
//!
//! This is a binding to [the libwebp library](https://developers.google.com/speed/webp/download).
//!
//! ## Usage
//!
//! ### Preparation
//!
//! ```toml
//! # Cargo.toml
//!
//! [dependencies]
//! libwebp = { version = "0.1.0", features = ["0_6"] }
//! ```
//!
//! ### Simple decoding
//!
//! You can use [`WebPDecodeRGBA`] or [`WebPDecodeRGBAInto`] families for
//! simple decoding.
//!
//! [`WebPDecodeRGBA`]: fn.WebPDecodeRGBA.html
//! [`WebPDecodeRGBAInto`]: fn.WebPDecodeRGBAInto.html
//!
//! ```rust
//! use libwebp::WebPDecodeRGBA;
//!
//! let data: &[u8];
//! # let data: &[u8] = include_bytes!("lena.webp");
//!
//! let (width, height, buf) = WebPDecodeRGBA(data).unwrap();
//! # assert_eq!((width, height), (128, 128));
//! assert_eq!(buf.len(), width as usize * height as usize * 4);
//! eprintln!("width = {}, height = {}", width, height);
//! eprintln!(
//!     "top-left pixel: rgba({}, {}, {}, {})",
//!     buf[0],
//!     buf[1],
//!     buf[2],
//!     buf[3] as f64 / 255.0,
//! )
//! ```
//!
//! ### Simple encoding
//!
//! You can use [`WebPEncodeRGBA`] or [`WebPEncodeLosslessRGBA`] families for
//! simple encoding.
//!
//! [`WebPEncodeRGBA`]: fn.WebPEncodeRGBA.html
//! [`WebPEncodeLosslessRGBA`]: fn.WebPEncodeLosslessRGBA.html
//!
//! ```rust
//! use libwebp::{WebPEncodeRGBA, WebPEncodeLosslessRGBA};
//!
//! let buf: &[u8] = &[
//!     255, 255, 255, 255, // white
//!     255, 0, 0, 255, // red
//!     0, 255, 0, 255, // green
//!     0, 0, 255, 255, // blue
//! ];
//! let data = WebPEncodeRGBA(buf, 2, 2, 8, 75.0).unwrap();
//! let lossless_data = WebPEncodeLosslessRGBA(buf, 2, 2, 8).unwrap();
//! assert_eq!(&data[..4], b"RIFF");
//! assert_eq!(&data[8..12], b"WEBP");
//! assert_eq!(&lossless_data[..4], b"RIFF");
//! assert_eq!(&lossless_data[8..12], b"WEBP");
//! ```

pub use crate::decode::*;
pub use crate::encode::*;

pub mod boxed;
mod decode;
mod encode;
pub mod error;
