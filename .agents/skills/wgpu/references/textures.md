# Texture, attachment, and color contracts

## Lifetime and capabilities

- Select format, resolution, samples, view range, and usages from required quality and actual consumers.
  Avoid dead MRT channels, unneeded stencil/HDR, and retaining disposable presentation textures as history.
- Clear when prior contents are unused; load only needed history. Store outputs consumed later; discard
  dead attachments. Discard cannot preserve a later load, and tile GPUs can pay significant bandwidth for
  stores.
- For Rust 30 pass-local targets, TRANSIENT_ATTACHMENT|RENDER_ATTACHMENT must have no other texture usages
  and clear/discard operations. Keep resolve output persistent; never use transient flags for sampled/history
  data.
- Check format usages, storage access, filtering, blending, and sample counts. Adapter support beyond the
  portable baseline still needs relevant device features; inspect get_texture_format_features on Rust.
- Match floating/normalized, integer, depth/comparison texture and sampler declarations to actual formats.
  For storage textures, use exact format/access and gate read/read-write/native extensions on actual support.
  Start portable feedback processing with sampled input and a distinct writable output.

## Surface views and MSAA

- Select Rust surface format/present/alpha options from capabilities; match configured attachment views,
  including sRGB and permitted view_formats. Browser preferred-format advice is not a native API signature.
- Keep color/depth/MSAA dimensions and sample counts coherent after resize. Resolve color into a compatible
  single-sample target; discard MSAA color if only resolve survives. Do not assume automatic depth resolve
  or ordinary filtered sampling of multisampled textures.

## Sampling, color, and alpha

- Use textureLoad for exact/integer/unfilterable texels. Use implicit-derivative textureSample only where
  its stage/control flow is valid; use explicit LOD/gradients where legal for compute/nonuniform access.
  Do not suppress uniformity diagnostics to conceal an invalid sampling design.
- Use sRGB for authored color and linear formats for normals, roughness, IDs, and numeric state. Decode/encode
  through the selected views once; preserve linear lighting/blending and keep alpha separate from gamma.
- Use a linear float intermediate, such as Rgba16Float, when HDR range is needed. Tone-map to SDR output
  or configure a verified HDR color-space/luminance path; a float attachment alone does not establish HDR
  presentation.
- For straight alpha, use source RGB factor source-alpha and destination factor one-minus-source-alpha;
  for premultiplied RGB, use source factor one. Match the alpha equation and surface composition mode.
  Normally disable depth writes for ordinary alpha blending; masked/cutout depth policy differs.

## Mips and asset transfer

- Compare offline mip generation with runtime downsample passes from actual asset/update frequency.
  Allocate and populate every sampled mip. For downsampling, bind only nonoverlapping source/destination
  levels, clamp dimensions to at least one, preserve odd edges, and average color in linear space.
- Renormalize normal maps and use appropriate mask/depth reductions; do not blindly average all data textures.
- Preserve valid mip chains and supported BC/ETC2/ASTC choices/transcoding for compressed assets. Compare
  quality plus transcode/upload cost; compression is not suitable for every data/render target. Match
  sampler LOD settings to populated levels and use mips for minification when quality/cache behavior needs it.
- Decode assets away from critical encoding work and preserve numeric/channel representation through upload.
  For Wasm external image/video interop, honor origin, conversion, and frame lifetime; do not transplant
  JavaScript host object calls into native Rust.

Sources: [formats][formats], [texture operations][ops], [Rust capabilities][rust], [attachments][passes].

[formats]: https://www.w3.org/TR/webgpu/#texture-format-caps
[ops]: https://www.w3.org/TR/WGSL/#texture-builtin-functions
[rust]: https://docs.rs/wgpu/30.0.1/wgpu/struct.Adapter.html#method.get_texture_format_features
[passes]: https://www.w3.org/TR/webgpu/#render-passes
