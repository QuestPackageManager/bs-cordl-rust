#[cfg(feature = "LIV+SDK+Unity+TEXTURE_COLOR_SPACE")]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TEXTURE_COLOR_SPACE {
    LINEAR = 16777729u32,
    SRGB = 33619970u32,
    UNDEFINED = 131328u32,
}
#[cfg(feature = "LIV+SDK+Unity+TEXTURE_COLOR_SPACE")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::LIV::SDK::Unity::TEXTURE_COLOR_SPACE =>
    "LIV.SDK.Unity"."TEXTURE_COLOR_SPACE"
);
