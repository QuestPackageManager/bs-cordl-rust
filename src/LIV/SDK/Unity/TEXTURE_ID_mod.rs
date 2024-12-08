#[cfg(feature = "LIV+SDK+Unity+TEXTURE_ID")]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TEXTURE_ID {
    BACKGROUND_COLOR_BUFFER_ID = 1971210u32,
    FOREGROUND_COLOR_BUFFER_ID = 16784916u32,
    OPTIMIZED_COLOR_BUFFER_ID = 65566u32,
    UNDEFINED = 504629760u32,
}
#[cfg(feature = "LIV+SDK+Unity+TEXTURE_ID")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::LIV::SDK::Unity::TEXTURE_ID => "LIV.SDK.Unity"
    ."TEXTURE_ID"
);
