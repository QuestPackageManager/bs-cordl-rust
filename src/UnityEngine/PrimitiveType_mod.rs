#[cfg(feature = "UnityEngine+PrimitiveType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimitiveType {
    Capsule = 1i32,
    Cube = 3i32,
    Cylinder = 2i32,
    Plane = 4i32,
    Quad = 5i32,
    Sphere = 0i32,
}
#[cfg(feature = "UnityEngine+PrimitiveType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::PrimitiveType => "UnityEngine"
    ."PrimitiveType"
);
