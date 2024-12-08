#[cfg(feature = "UnityEngine+UIElements+LanguageDirection")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LanguageDirection {
    Inherit = 0i32,
    LTR = 1i32,
    RTL = 2i32,
}
#[cfg(feature = "UnityEngine+UIElements+LanguageDirection")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::LanguageDirection =>
    "UnityEngine.UIElements"."LanguageDirection"
);
