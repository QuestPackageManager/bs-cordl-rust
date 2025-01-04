#[cfg(feature = "UnityEngine+SpriteDrawMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SpriteDrawMode {
    #[default]
    Simple = 0i32,
    Sliced = 1i32,
    Tiled = 2i32,
}
#[cfg(feature = "UnityEngine+SpriteDrawMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::SpriteDrawMode => "UnityEngine"
    ."SpriteDrawMode"
);
