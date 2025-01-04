#[cfg(feature = "OVRTextureQualityFiltering")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRTextureQualityFiltering {
    #[default]
    Aniso16x = 5i32,
    Aniso2x = 2i32,
    Aniso4x = 3i32,
    Aniso8x = 4i32,
    Bilinear = 0i32,
    None = -1i32,
    Trilinear = 1i32,
}
#[cfg(feature = "OVRTextureQualityFiltering")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRTextureQualityFiltering =>
    ""."OVRTextureQualityFiltering"
);
