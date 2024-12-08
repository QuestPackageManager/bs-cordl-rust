#[cfg(feature = "UnityEngine+FullScreenMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FullScreenMode {
    ExclusiveFullScreen = 0i32,
    FullScreenWindow = 1i32,
    MaximizedWindow = 2i32,
    Windowed = 3i32,
}
#[cfg(feature = "UnityEngine+FullScreenMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::FullScreenMode => "UnityEngine"
    ."FullScreenMode"
);
