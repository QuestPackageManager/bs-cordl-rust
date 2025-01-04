#[cfg(feature = "UnityEngine+SpritePackingRotation")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SpritePackingRotation {
    #[default]
    Any = 15i32,
    FlipHorizontal = 1i32,
    FlipVertical = 2i32,
    None = 0i32,
    Rotate180 = 3i32,
}
#[cfg(feature = "UnityEngine+SpritePackingRotation")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::SpritePackingRotation =>
    "UnityEngine"."SpritePackingRotation"
);
