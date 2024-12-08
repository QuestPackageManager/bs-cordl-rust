#[cfg(feature = "OVRGLTFType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRGLTFType {
    MAT4 = 5i32,
    NONE = 0i32,
    SCALAR = 1i32,
    VEC2 = 2i32,
    VEC3 = 3i32,
    VEC4 = 4i32,
}
#[cfg(feature = "OVRGLTFType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for OVRGLTFType => ""."OVRGLTFType"
);
