#[cfg(feature = "UnityEngine+UIElements+StyleValueType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StyleValueType {
    #[default]
    AssetReference = 6i32,
    Color = 4i32,
    CommaSeparator = 11i32,
    Dimension = 3i32,
    Enum = 7i32,
    Float = 2i32,
    Function = 10i32,
    Invalid = 0i32,
    Keyword = 1i32,
    MissingAssetReference = 13i32,
    ResourcePath = 5i32,
    ScalableImage = 12i32,
    String = 9i32,
    Variable = 8i32,
}
#[cfg(feature = "UnityEngine+UIElements+StyleValueType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::StyleValueType =>
    "UnityEngine.UIElements"."StyleValueType"
);
