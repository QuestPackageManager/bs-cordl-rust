#[cfg(feature = "UnityEngine+InputSystem+LowLevel+TouchFlags")]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TouchFlags {
    BeganInSameFrame = 128u8,
    IndirectTouch = 1u8,
    OrphanedPrimaryTouch = 64u8,
    PrimaryTouch = 8u8,
    TapPress = 16u8,
    TapRelease = 32u8,
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+TouchFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::LowLevel::TouchFlags
    => "UnityEngine.InputSystem.LowLevel"."TouchFlags"
);
