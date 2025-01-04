#[cfg(feature = "UnityEngine+TextCore+Text+TagValueType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TagValueType {
    #[default]
    ColorValue = 4i32,
    None = 0i32,
    NumericalValue = 1i32,
    StringValue = 2i32,
}
#[cfg(feature = "UnityEngine+TextCore+Text+TagValueType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TextCore::Text::TagValueType =>
    "UnityEngine.TextCore.Text"."TagValueType"
);
