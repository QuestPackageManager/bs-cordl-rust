#[cfg(feature = "UnityEngine+UIElements+HelpBoxMessageType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HelpBoxMessageType {
    #[default]
    Error = 3i32,
    Info = 1i32,
    None = 0i32,
    Warning = 2i32,
}
#[cfg(feature = "UnityEngine+UIElements+HelpBoxMessageType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::HelpBoxMessageType =>
    "UnityEngine.UIElements"."HelpBoxMessageType"
);
