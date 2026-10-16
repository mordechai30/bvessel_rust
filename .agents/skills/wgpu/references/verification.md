# GPU contract verification and measurement

## Creation and failure handling

- Compile host APIs against the resolved crate; validate WGSL and all selected entry points/variants against
  the selected Naga/backend. Host compilation does not validate shader execution or device capability.
- Label resources/passes/pipelines by purpose and variant; retain locations for generated WGSL.
- Install uncaptured-error and device-loss reporting. Capture recoverable creation errors in narrow scopes.
  In Rust 30, push_error_scope returns a guard; call pop before awaiting and nest scopes in reverse order.
  Do not overlap unrelated scope operations or async task reordering on the same device.
- Reject a returned shader/pipeline/resource when its captured error invalidates creation; do not keep using
  a handle merely because the Rust creation method returned it. Classify validation, allocation, and loss
  separately; a validation-only scope is not an allocation/device-loss handler.
- For Rust 30, pass Arc<dyn UncapturedErrorHandler> to on_uncaptured_error and a Send + 'static closure
  receiving reason/message to set_device_lost_callback. Verify signatures before adapting older examples.

## Focused acceptance cases

| Changed contract | Check on the supported target |
| --- | --- |
| Host/shader representation | Offsets, scalar-after-vec3, matrix columns, bytes and live counts |
| Per-command parameters | Distinct draw records; nonzero dynamic offsets; intended minimum/effective ranges |
| Compute sequencing | Several encoded steps, final ping-pong destination, dependent dispatch phases |
| Guarded barriers | Partial final group, inactive lanes, neutral values, every required barrier reachable |
| Shared uploads/readback | Reordered producers, delayed completion, byte/count saturation, wraparound |
| Obsolete state | Replacement/resize/loss before callback; reject old device/resource/request generations |
| Recovery and capabilities | Zero-size window, surface outcomes, f16/immediate/format fallback variants |

Observe resulting values and completion, not only whether commands validate. For visual paths, compare
linear/sRGB conversion, alpha order, mip contents, and quality after resize. For numeric paths, set an
explicit error budget and exercise extremes/repeated updates. Report unsupported or unrun target checks.

## Measurement

- Measure CPU preparation, encoding, GPU execution, transfers, and frame/presentation latency separately.
  Compare equal workload, resolution, geometry, iterations, and quality across supported target backends.
- Warm pipelines/allocations before steady-state measurements; inspect distributions and tail latency.
- For portable pass timestamps, grant TIMESTAMP_QUERY; gate inside-pass/encoder timestamp features
  separately. Multiply timestamp deltas by Queue::get_timestamp_period for nanoseconds.
- Resolve into QUERY_RESOLVE|COPY_SRC, then copy to MAP_READ|COPY_DST; align resolve offsets to 256 bytes.
  Read asynchronously and reclaim completed slots. Reject invalid, unavailable, stale, or misleading zeros.
- Compare pass timings with whole-frame captures: indirect validation, copies, clearing, and backend work
  can sit outside pass timestamps. Use Xcode GPU capture on Metal when it explains those costs.
- Measure batching, bundles, fusion, dynamic uniforms/storage, workgroup shape, half precision, and branch
  removal as hypotheses. Preserve numeric/visual quality; faster shaders can still make frames slower.
- Report hardware/backend, granted features/limits, compiler/crate, workload, repetitions, output checks,
  and evidence limits. Do not claim GPU execution, portability, soundness, or speedups from CPU-only checks.

## Example execution

Resolve example paths from the skill root. Run all commands/scripts outside sandbox restrictions.
Use an isolated temporary Cargo package; do not change product dependencies to exercise these examples.

- Use examples/gpu-contracts.rs as its library with wgpu = "=30.0.1" and half = "2"; add a test-only
  naga = { version = "=30.0.1", features = ["wgsl-in"] }. Keep the five adjacent WGSL files beside the Rust
  file for include_str. Resolve the temporary package, then run cargo test --locked.
- Expect four tests: host byte layout, stride/overflow, binary16 encoding, and Naga validation of all five
  shaders plus the WGSL object ABI. The validator grants offline capabilities for f16/immediates; that
  does not negotiate device features. The tests compile GPU helpers without invoking their GPU methods.
- For object.wgsl, build a pipeline with ObjectSlots::layout at group zero, its vertex/fragment entry points,
  and matching target state. Upload distinct records through the submission owner, bind the selected slot,
  draw exactly three generated vertices, then submit. Pass an actual memory budget to ObjectSlots::new.
- For specialized.wgsl, use the helper's main entry point and override ID zero. Bind writable storage and
  a four-byte live count; calculate ceiling dispatch count from the same lane count.
- For immediate-selection.wgsl, grant IMMEDIATES, request at least eight bytes, and set immediate_size = 8
  on a layout with the shader's two read-only storage bindings. Use select_draw before each three-vertex
  draw. Keep a buffer fallback for targets lacking that capability.
- For guarded-reduction.wgsl, use 64 lanes, input/partial storage, and the four-byte live count. Allocate
  enough partials and dispatch in one dimension. Reduce partials in a subsequent dispatch if needed;
  evaluate the changed floating-point grouping against the chosen tolerance.
- For mixed-precision.wgsl, grant SHADER_F16, encode vec2<f16> inputs, bind f32 output and the four-byte live
  count, and dispatch 64 lanes per group. Compare actual output with the f32 reference/error budget.
- Use checked_shader to reject scoped validation failures. Select f16 modules from the returned device's
  granted features after precision_device; keep unsupported enable directives out of the fallback module.

Do not interpret host tests or offline WGSL validation as a GPU run. Execute target-backend tests before
claiming resource negotiation, validation callbacks, output correctness, or performance on a device.

Sources: [queries][queries], [scoped errors][scope], [loss callback][loss], [timestamp units][queue].

[queries]: https://www.w3.org/TR/webgpu/#queries
[scope]: https://docs.rs/wgpu/30.0.1/wgpu/struct.Device.html#method.push_error_scope
[loss]: https://docs.rs/wgpu/30.0.1/wgpu/struct.Device.html#method.set_device_lost_callback
[queue]: https://docs.rs/wgpu/30.0.1/wgpu/struct.Queue.html#method.get_timestamp_period
