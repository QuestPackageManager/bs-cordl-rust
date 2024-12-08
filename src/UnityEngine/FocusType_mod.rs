#[cfg(feature = "UnityEngine+FocusType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusType {
    Keyboard = 1i32,
    Native = 0i32,
    Passive = 2i32,
}
#[cfg(feature = "UnityEngine+FocusType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::FocusType => "UnityEngine"
    ."FocusType"
);
