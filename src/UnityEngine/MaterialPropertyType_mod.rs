#[cfg(feature = "UnityEngine+MaterialPropertyType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaterialPropertyType {
    ComputeBuffer = 6i32,
    ConstantBuffer = 5i32,
    Float = 0i32,
    Int = 1i32,
    Matrix = 3i32,
    Texture = 4i32,
    Vector = 2i32,
}
#[cfg(feature = "UnityEngine+MaterialPropertyType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::MaterialPropertyType =>
    "UnityEngine"."MaterialPropertyType"
);
