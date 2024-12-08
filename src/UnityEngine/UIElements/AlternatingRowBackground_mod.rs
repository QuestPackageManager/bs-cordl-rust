#[cfg(feature = "UnityEngine+UIElements+AlternatingRowBackground")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlternatingRowBackground {
    All = 2i32,
    ContentOnly = 1i32,
    None = 0i32,
}
#[cfg(feature = "UnityEngine+UIElements+AlternatingRowBackground")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::AlternatingRowBackground => "UnityEngine.UIElements"
    ."AlternatingRowBackground"
);
