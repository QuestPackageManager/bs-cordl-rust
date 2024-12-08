#[cfg(feature = "OVR+OpenVR+ECollisionBoundsStyle")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ECollisionBoundsStyle {
    COLLISION_BOUNDS_STYLE_ADVANCED = 3i32,
    COLLISION_BOUNDS_STYLE_BEGINNER = 0i32,
    COLLISION_BOUNDS_STYLE_COUNT = 5i32,
    COLLISION_BOUNDS_STYLE_INTERMEDIATE = 1i32,
    COLLISION_BOUNDS_STYLE_NONE = 4i32,
    COLLISION_BOUNDS_STYLE_SQUARES = 2i32,
}
#[cfg(feature = "OVR+OpenVR+ECollisionBoundsStyle")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::ECollisionBoundsStyle =>
    "OVR.OpenVR"."ECollisionBoundsStyle"
);
