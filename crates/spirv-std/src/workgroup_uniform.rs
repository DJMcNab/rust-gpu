//! Types and traits for working with 'workgroup uniform types'

/// A type which can be used in a non workgroup_uniform
// HACK(DJMcNab): We piggyback on the diagnotic item system to
// give reasonable access to this trait in contexts with only a `TyCtxt`.
// The alternative would be a scan by path, which is doable but more messy
#[rustc_diagnostic_item = "RustGpuDivergent"]
pub unsafe auto trait Divergent {}

//In this region, we handle the current See: https://doc.rust-lang.org/reference/special-types-and-traits.html#auto-traits
// The automatic impls for the following are considered 'correct':
// `&mut T`, `*const T`, `*mut T`, `[T; n], [T]`: A divergent value can be modified in a divergent context, and an explicit impl can be added for non-divergent `T`
// Function item types: These store no data, and can be accessed at any point
// However, this unfortunately breaks down for function pointers:
// Consider:
// ```rust
// fn called_non_uniformly(){
//     let a: fn() = requires_uniform;
//     a();
// }
// ```
// Unfortunately, since we have to work within the constraints of the type system, we can't
// make `requires_uniform` function pointers !Divergent directly; we can't even detect such.
// These are handled by special casing in the handling code; (we prevent turning function item
// types for functions annotated with `#[spirv(workgroup_uniform)]` into function pointers)

/// A non-divergent value can be read in any context
unsafe impl<T> Divergent for &T {}
