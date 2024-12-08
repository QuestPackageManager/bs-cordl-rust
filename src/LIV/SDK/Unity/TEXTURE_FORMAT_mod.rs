#[cfg(feature = "LIV+SDK+Unity+TEXTURE_FORMAT")]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TEXTURE_FORMAT {
    ARGB32 = 33619978u32,
    UNDEFINED = 16779776u32,
}
#[cfg(feature = "LIV+SDK+Unity+TEXTURE_FORMAT")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::LIV::SDK::Unity::TEXTURE_FORMAT =>
    "LIV.SDK.Unity"."TEXTURE_FORMAT"
);
