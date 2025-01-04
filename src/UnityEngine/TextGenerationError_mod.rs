#[cfg(feature = "UnityEngine+TextGenerationError")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextGenerationError {
    #[default]
    CustomSizeOnNonDynamicFont = 1i32,
    CustomStyleOnNonDynamicFont = 2i32,
    NoFont = 4i32,
    None = 0i32,
}
#[cfg(feature = "UnityEngine+TextGenerationError")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TextGenerationError =>
    "UnityEngine"."TextGenerationError"
);
