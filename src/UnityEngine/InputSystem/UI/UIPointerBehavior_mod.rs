#[cfg(feature = "UnityEngine+InputSystem+UI+UIPointerBehavior")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UIPointerBehavior {
    #[default]
    AllPointersAsIs = 2i32,
    SingleMouseOrPenButMultiTouchAndTrack = 0i32,
    SingleUnifiedPointer = 1i32,
}
#[cfg(feature = "UnityEngine+InputSystem+UI+UIPointerBehavior")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::UI::UIPointerBehavior
    => "UnityEngine.InputSystem.UI"."UIPointerBehavior"
);
