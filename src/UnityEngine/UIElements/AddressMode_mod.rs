#[cfg(feature = "UnityEngine+UIElements+AddressMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressMode {
    Clamp = 1i32,
    Mirror = 2i32,
    Wrap = 0i32,
}
#[cfg(feature = "UnityEngine+UIElements+AddressMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::AddressMode =>
    "UnityEngine.UIElements"."AddressMode"
);
