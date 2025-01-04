#[cfg(feature = "UnityEngine+AvatarIKGoal")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AvatarIKGoal {
    #[default]
    LeftFoot = 0i32,
    LeftHand = 2i32,
    RightFoot = 1i32,
    RightHand = 3i32,
}
#[cfg(feature = "UnityEngine+AvatarIKGoal")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::AvatarIKGoal => "UnityEngine"
    ."AvatarIKGoal"
);
