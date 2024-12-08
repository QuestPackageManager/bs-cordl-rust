#[cfg(feature = "UnityEngine+UIElements+UIR+OwnedState")]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OwnedState {
    Inherited = 0u8,
    Owned = 1u8,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+OwnedState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::UIR::OwnedState =>
    "UnityEngine.UIElements.UIR"."OwnedState"
);
