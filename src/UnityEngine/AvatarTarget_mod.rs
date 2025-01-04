#[cfg(feature = "UnityEngine+AvatarTarget")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AvatarTarget {
    #[default]
    Body = 1i32,
    LeftFoot = 2i32,
    LeftHand = 4i32,
    RightFoot = 3i32,
    RightHand = 5i32,
    Root = 0i32,
}
#[cfg(feature = "UnityEngine+AvatarTarget")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::AvatarTarget => "UnityEngine"
    ."AvatarTarget"
);
