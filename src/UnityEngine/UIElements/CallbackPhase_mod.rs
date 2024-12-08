#[cfg(feature = "UnityEngine+UIElements+CallbackPhase")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CallbackPhase {
    TargetAndBubbleUp = 1i32,
    TrickleDownAndTarget = 2i32,
}
#[cfg(feature = "UnityEngine+UIElements+CallbackPhase")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::CallbackPhase =>
    "UnityEngine.UIElements"."CallbackPhase"
);
