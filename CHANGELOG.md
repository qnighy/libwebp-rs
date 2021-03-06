## Unreleased

- MSRV is now 1.33.0
- Internal: bump dev-dependency of rand
- Split `WebPIDecoder` into `WebPIDecoder` and `WebPIDecoderBox`

## 0.1.2

- Implement bindings for incremental decoding
- Refactor: expand macros in `encode.rs`
- Add docs for simple decoding/encoding
- Implement auto traits for `WebpBox` and `WebpYuvBox`

## 0.1.1

- Refactorings:
  - Split out `encode.rs` and `decode.rs`
  - Move `wrap_bytes` to `boxed.rs`
  - Abbreviate `libwebp_sys` as `sys`
- Fix WebPDecoderBGR which mistakenly delegated to RGB
- Implement `WebPDecode*Into`, including `WebPDecodeYUVInto`

## 0.1.0

Initial release.
