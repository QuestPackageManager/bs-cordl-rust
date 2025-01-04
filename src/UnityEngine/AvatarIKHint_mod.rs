#[cfg(feature = "UnityEngine+AvatarIKHint")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AvatarIKHint {
    #[default]
    LeftElbow = 2i32,
    LeftKnee = 0i32,
    RightElbow = 3i32,
    RightKnee = 1i32,
}
#[cfg(feature = "UnityEngine+AvatarIKHint")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::AvatarIKHint => "UnityEngine"
    ."AvatarIKHint"
);
