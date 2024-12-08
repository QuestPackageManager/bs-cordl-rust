#[cfg(feature = "UnityEngine+AvatarMaskBodyPart")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AvatarMaskBodyPart {
    Body = 1i32,
    Head = 2i32,
    LastBodyPart = 13i32,
    LeftArm = 5i32,
    LeftFingers = 7i32,
    LeftFootIK = 9i32,
    LeftHandIK = 11i32,
    LeftLeg = 3i32,
    RightArm = 6i32,
    RightFingers = 8i32,
    RightFootIK = 10i32,
    RightHandIK = 12i32,
    RightLeg = 4i32,
    Root = 0i32,
}
#[cfg(feature = "UnityEngine+AvatarMaskBodyPart")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::AvatarMaskBodyPart => "UnityEngine"
    ."AvatarMaskBodyPart"
);
