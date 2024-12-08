#[cfg(feature = "OVRTextureFormat")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRTextureFormat {
    JPEG = 3i32,
    KTX2 = 1i32,
    NONE = 0i32,
    PNG = 2i32,
}
#[cfg(feature = "OVRTextureFormat")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for OVRTextureFormat => ""."OVRTextureFormat"
);
