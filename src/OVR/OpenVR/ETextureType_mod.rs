#[cfg(feature = "OVR+OpenVR+ETextureType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ETextureType {
    #[default]
    DXGISharedHandle = 5i32,
    DirectX = 0i32,
    DirectX12 = 4i32,
    IOSurface = 3i32,
    Invalid = -1i32,
    Metal = 6i32,
    OpenGL = 1i32,
    Vulkan = 2i32,
}
#[cfg(feature = "OVR+OpenVR+ETextureType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::ETextureType => "OVR.OpenVR"
    ."ETextureType"
);
