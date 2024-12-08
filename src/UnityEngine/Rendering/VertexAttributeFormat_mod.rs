#[cfg(feature = "UnityEngine+Rendering+VertexAttributeFormat")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VertexAttributeFormat {
    Float16 = 1i32,
    Float32 = 0i32,
    SInt16 = 9i32,
    SInt32 = 11i32,
    SInt8 = 7i32,
    SNorm16 = 5i32,
    SNorm8 = 3i32,
    UInt16 = 8i32,
    UInt32 = 10i32,
    UInt8 = 6i32,
    UNorm16 = 4i32,
    UNorm8 = 2i32,
}
#[cfg(feature = "UnityEngine+Rendering+VertexAttributeFormat")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::VertexAttributeFormat =>
    "UnityEngine.Rendering"."VertexAttributeFormat"
);
