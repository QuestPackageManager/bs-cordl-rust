#[cfg(feature = "UnityEngine+StereoTargetEyeMask")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StereoTargetEyeMask {
    Both = 3i32,
    Left = 1i32,
    None = 0i32,
    Right = 2i32,
}
#[cfg(feature = "UnityEngine+StereoTargetEyeMask")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::StereoTargetEyeMask =>
    "UnityEngine"."StereoTargetEyeMask"
);
