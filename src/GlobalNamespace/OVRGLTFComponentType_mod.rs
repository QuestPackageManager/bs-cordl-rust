#[cfg(feature = "OVRGLTFComponentType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRGLTFComponentType {
    BYTE = 5120i32,
    FLOAT = 5126i32,
    SHORT = 5122i32,
    UNSIGNED_BYTE = 5121i32,
    UNSIGNED_INT = 5125i32,
    UNSIGNED_SHORT = 5123i32,
}
#[cfg(feature = "OVRGLTFComponentType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRGLTFComponentType => ""
    ."OVRGLTFComponentType"
);
