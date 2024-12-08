#[cfg(feature = "UnityEngine+CollisionFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollisionFlags {
    Above = 2i32,
    Below = 4i32,
    CollidedSides = 1i32,
    None = 0i32,
}
#[cfg(feature = "UnityEngine+CollisionFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::CollisionFlags => "UnityEngine"
    ."CollisionFlags"
);
