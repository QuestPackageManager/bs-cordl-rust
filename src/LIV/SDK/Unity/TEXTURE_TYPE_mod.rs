#[cfg(feature = "LIV+SDK+Unity+TEXTURE_TYPE")]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TEXTURE_TYPE {
    COLOR_BUFFER = 655361u32,
    UNDEFINED = 167772416u32,
}
#[cfg(feature = "LIV+SDK+Unity+TEXTURE_TYPE")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::LIV::SDK::Unity::TEXTURE_TYPE => "LIV.SDK.Unity"
    ."TEXTURE_TYPE"
);
