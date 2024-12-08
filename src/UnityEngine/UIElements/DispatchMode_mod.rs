#[cfg(feature = "UnityEngine+UIElements+DispatchMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DispatchMode {
    Default = 1i32,
    Immediate = 2i32,
}
#[cfg(feature = "UnityEngine+UIElements+DispatchMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::DispatchMode =>
    "UnityEngine.UIElements"."DispatchMode"
);
