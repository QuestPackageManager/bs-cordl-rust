#[cfg(feature = "LIV+SDK+Unity+TEXTURE_DEVICE")]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TEXTURE_DEVICE {
    #[default]
    DIRECTX = 84148994u32,
    METAL = 33619973u32,
    OPENGL = 328707u32,
    RAW = 67305985u32,
    UNDEFINED = 50462976u32,
    VULKAN = 16778500u32,
}
#[cfg(feature = "LIV+SDK+Unity+TEXTURE_DEVICE")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::LIV::SDK::Unity::TEXTURE_DEVICE =>
    "LIV.SDK.Unity"."TEXTURE_DEVICE"
);
