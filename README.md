# libwebp

This is a binding to [the libwebp library](https://developers.google.com/speed/webp/download).

## Usage

### Preparation

```toml
# Cargo.toml

[dependencies]
libwebp = { version = "0.1.2", features = ["0_6"] }
```

### Simple decoding

```rust
use libwebp::WebPDecodeRGBA;

let data: &[u8];

let (width, height, buf) = WebPDecodeRGBA(data).unwrap();
eprintln!("width = {}, height = {}", width, height);
eprintln!(
    "top-left pixel: rgba({}, {}, {}, {})",
    buf[0],
    buf[1],
    buf[2],
    buf[3] as f64 / 255.0,
)
```

### Simple encoding

```rust
use libwebp::WebPEncodeRGBA;

let buf: &[u8] = &[
    255, 255, 255, 255, // white
    255, 0, 0, 255, // red
    0, 255, 0, 255, // green
    0, 0, 255, 255, // blue
];
let data = WebPEncodeRGBA(buf, 2, 2, 8, 75.0).unwrap();
let lossless_data = WebPEncodeLosslessRGBA(buf, 2, 2, 8).unwrap();
```

## Minimum Supported Rust Version (MSRV)

Rust 1.31.0

## Features

- `demux` ... enables `libwebpdemux` functions.
- `mux` ... enables `libwebpmux` functions.
- `0_5` ... enables functions introduced in libwebp 0.5.0.
- `0_6` ... enables functions introduced in libwebp 0.6.0.
- `1_1` ... enables functions introduced in libwebp 1.1.0.
- `static` ... statically link against the bundled libwebp.
- `extern-types` ... enables `#![feature(extern_types)]`.

## Linking

If libwebp is found in the system, it links against the library.
Otherwise it builds and links against the bundled libwebp.

In these cases, static link is preferred:

- For musl target.
- When cross-compiling.
- `static` feature is turned on.
- `LIBWEBP_SYS_STATIC` environment variable is set to `1` when building.

## Related repositories

- https://github.com/qnighy/libwebp-sys2-rs
- https://github.com/qnighy/libwebp-image-rs

## Completeness

- `types.h`
  - [ ] `WebPMalloc`
  - [x] `WebPFree`
- `decode.h`
  - [x] `WebPGetDecoderVersion`
  - [x] `WebPGetInfo`
  - [x] `WebPDecode*`
  - [x] `WebPDecodeYUV`
  - [x] `WebPDecode*Into`
  - [x] `WebPDecodeYUVInto`
  - [x] `WEBP_CSP_MODE`
  - [x] `WebPIsPremultipliedMode`
  - [x] `WebPIsAlphaMode`
  - [x] `WebPIsRGBMode`
  - [ ] `WebPRGBABuffer`
  - [ ] `WebPYUVABuffer`
  - [ ] `WebPDecBuffer`
  - [ ] `WebPInitDecBuffer`
  - [ ] `WebPFreeDecBuffer`
  - [x] `VP8StatusCode`
  - [x] `WebPIDecoder` (internal memory)
  - [ ] `WebPIDecoder` (external memory)
  - [x] `WebPINewDecoder` (internal memory)
  - [ ] `WebPINewDecoder` (external memory)
  - [x] `WebPINewRGB` (internal memory)
  - [ ] `WebPINewRGB` (external memory)
  - [x] `WebPINewYUVA` (internal memory)
  - [ ] `WebPINewYUVA` (external memory)
  - [x] `WebPIDelete`
  - [x] `WebPIAppend`
  - [ ] `WebPIUpdate`
  - [x] `WebPIDecGetRGB`
  - [x] `WebPIDecGetYUVA`
  - [ ] `WebPIDecodedArea`
  - [ ] `WebPBitstreamFeatures`
  - [ ] `WebPGetFeatures`
  - [ ] `WebPDecoderOptions`
  - [ ] `WebPDecoderConfig`
  - [ ] `WebPInitDecoderConfig`
  - [ ] `WebPIDecode`
  - [ ] `WebPDecode`
- `encode.h`
  - [x] `WebPGetEncoderVersion`
  - [x] `WebPEncode*`
  - [x] `WebPEncodeLossless*`
  - [ ] `WebPImageHint`
  - [ ] `WebPConfig`
  - [ ] `WebPPreset`
  - [ ] `WebPConfigInit`
  - [ ] `WebPConfigPreset`
  - [ ] `WebPConfigLosslessPreset`
  - [ ] `WebPValidateConfig`
  - [ ] `WebPAuxStats`
  - [ ] `WebPWriterFunction`
  - [ ] `WebPMemoryWriter`
  - [ ] `WebPMemoryWriterInit`
  - [ ] `WebPMemoryWriterClear`
  - [ ] `WebPMemoryWrite`
  - [ ] `WebPProgressHook`
  - [ ] `WebPEncCSP`
  - [ ] `WebPEncodingError`
  - [ ] `WEBP_MAX_DIMENSION`
  - [ ] `WebPPicture`
  - [ ] `WebPPictureInit`
  - [ ] `WebPPictureAlloc`
  - [ ] `WebPPictureFree`
  - [ ] `WebPPictureCopy`
  - [ ] `WebPPlaneDistortion`
  - [ ] `WebPPictureDistortion`
  - [ ] `WebPPictureCrop`
  - [ ] `WebPPictureView`
  - [ ] `WebPPictureIsView`
  - [ ] `WebPPictureRescale`
  - [ ] `WebPPictureImportRGB`
  - [ ] `WebPPictureImportRGBA`
  - [ ] `WebPPictureImportRGBX`
  - [ ] `WebPPictureImportBGR`
  - [ ] `WebPPictureImportBGRA`
  - [ ] `WebPPictureImportBGRX`
  - [ ] `WebPPictureARGBToYUVA`
  - [ ] `WebPPictureARGBToYUVADithered`
  - [ ] `WebPPictureSharpARGBToYUVA`
  - [ ] `WebPPictureSmartARGBToYUVA`
  - [ ] `WebPPictureYUVAToARGB`
  - [ ] `WebPCleanupTransparentArea`
  - [ ] `WebPPictureHasTransparency`
  - [ ] `WebPBlendAlpha`
  - [ ] `WebPEncode`
- `mux_types.h`
  - Not at all
- `demux.h`
  - Not at all
- `mux.h`
  - Not at all


