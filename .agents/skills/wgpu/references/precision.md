# Half representation and numerical precision

- Separate typed f16 arithmetic/buffers, half-float texture formats, Float16 vertex decoding, and packed
  half pairs in u32. Enabling arithmetic does not make formats filterable or every operation faster.
- For typed f16, check adapter SHADER_F16, request it with other requirements, verify the granted device,
  and place enable f16 before WGSL declarations. Provide a validated f32 module when optional; reject
  unsupported adapters when required. Unsupported enable directives cannot be removed by runtime branches.
- Match variant-specific host layouts/ranges: replacing f16 with f32 changes strides and allocation sizes.
- Explicitly convert operands and retain intended operations in f16 for half arithmetic. Assigning a
  previously calculated f32 result into f16 does not retroactively change the earlier arithmetic.
- Sample/load a half-float texture through f32 interfaces; do not declare texture_2d<f16>. Compact half
  vertex formats can feed f32 shader inputs. pack2x16float/unpack2x16float can support packed baseline
  storage.
- Convert compact half inputs to f32 before sensitive arithmetic; keep only the intended storage compact.
  Request SHADER_F16 for typed half buffers even if their arithmetic is performed after conversion.
- Encode IEEE binary16 bits with a tested converter such as half::f16; numerical casting to u16 is not
  encoding. For a little-endian pair, write two two-byte bit representations, not native struct padding.
- Use natural f16 scalar alignment/size 2/2, vec2 4/4, vec3 8/6, vec4 8/8, vec3 array stride 8, and
  mat3x3 size 24. Apply address-space restrictions separately and pad host transfers to required granularity.
- Account for finite maximum 65504, quantization, cancellation, overflow, and backend subnormal behavior.
  Retain f32 for long reductions, large coordinates, sensitive depth, or repeated-state error growth.
- Compare storage-only compression, mixed precision, and full half arithmetic independently at equal
  quality. Include conversion and register/bandwidth effects, extremes, and repeated updates against f32.
- Treat scaling/local coordinates as algorithm changes requiring explicit error tests, not harmless
  conversions.

Sources: [WGSL f16][f16], [packing][pack], [Rust feature restrictions][rust].

[f16]: https://www.w3.org/TR/WGSL/#extension-f16
[pack]: https://www.w3.org/TR/WGSL/#pack2x16float-builtin
[rust]: https://docs.rs/wgpu/30.0.1/wgpu/struct.Features.html#associatedconstant.SHADER_F16
