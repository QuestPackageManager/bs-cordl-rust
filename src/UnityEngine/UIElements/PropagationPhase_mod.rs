#[cfg(feature = "UnityEngine+UIElements+PropagationPhase")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropagationPhase {
    AtTarget = 2i32,
    BubbleUp = 3i32,
    DefaultAction = 4i32,
    DefaultActionAtTarget = 5i32,
    None = 0i32,
    TrickleDown = 1i32,
}
#[cfg(feature = "UnityEngine+UIElements+PropagationPhase")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::PropagationPhase =>
    "UnityEngine.UIElements"."PropagationPhase"
);
