## Unreleased

- Refactorings:
  - Split out `encode.rs` and `decode.rs`
  - Move `wrap_bytes` to `boxed.rs`
  - Abbreviate `libwebp_sys` as `sys`
- Fix WebPDecoderBGR which mistakenly delegated to RGB
- Implement `WebPDecode*Into`, including `WebPDecodeYUVInto`

## 0.1.0

Initial release.
