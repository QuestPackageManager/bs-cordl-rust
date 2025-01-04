#[cfg(feature = "UnityEngine+TouchPhase")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TouchPhase {
    #[default]
    Began = 0i32,
    Canceled = 4i32,
    Ended = 3i32,
    Moved = 1i32,
    Stationary = 2i32,
}
#[cfg(feature = "UnityEngine+TouchPhase")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TouchPhase => "UnityEngine"
    ."TouchPhase"
);
