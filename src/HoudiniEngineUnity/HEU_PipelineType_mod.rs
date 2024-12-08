#[cfg(feature = "HoudiniEngineUnity+HEU_PipelineType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HEU_PipelineType {
    BiRP = 1i32,
    HDRP = 3i32,
    URP = 2i32,
    Unsupported = 0i32,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PipelineType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_PipelineType =>
    "HoudiniEngineUnity"."HEU_PipelineType"
);
