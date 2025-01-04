#[cfg(feature = "LIV+SDK+Unity+RENDERING_PIPELINE")]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RENDERING_PIPELINE {
    #[default]
    DEFERRED = 84148994u32,
    FORWARD = 67305985u32,
    HIGH_DEFINITION = 5u32,
    UNDEFINED = 50462976u32,
    UNIVERSAL = 1284u32,
    VERTEX_LIT = 328707u32,
}
#[cfg(feature = "LIV+SDK+Unity+RENDERING_PIPELINE")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::LIV::SDK::Unity::RENDERING_PIPELINE =>
    "LIV.SDK.Unity"."RENDERING_PIPELINE"
);
