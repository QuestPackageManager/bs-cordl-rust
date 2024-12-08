#[cfg(feature = "LIV+SDK+Unity+INVALIDATION_FLAGS")]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum INVALIDATION_FLAGS {
    EXCLUDE_BEHAVIOURS = 1879048200u32,
    HMD_CAMERA = 134480385u32,
    MR_CAMERA_PREFAB = 2052u32,
    NONE = 67240192u32,
    STAGE = 525314u32,
}
#[cfg(feature = "LIV+SDK+Unity+INVALIDATION_FLAGS")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::LIV::SDK::Unity::INVALIDATION_FLAGS =>
    "LIV.SDK.Unity"."INVALIDATION_FLAGS"
);
