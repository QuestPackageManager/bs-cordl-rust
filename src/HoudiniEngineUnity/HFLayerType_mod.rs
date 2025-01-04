#[cfg(feature = "HoudiniEngineUnity+HFLayerType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HFLayerType {
    #[default]
    DEFAULT = 0i32,
    DETAIL = 3i32,
    HEIGHT = 1i32,
    MASK = 2i32,
}
#[cfg(feature = "HoudiniEngineUnity+HFLayerType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HFLayerType =>
    "HoudiniEngineUnity"."HFLayerType"
);
