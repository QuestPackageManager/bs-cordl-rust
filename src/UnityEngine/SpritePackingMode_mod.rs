#[cfg(feature = "UnityEngine+SpritePackingMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SpritePackingMode {
    #[default]
    Rectangle = 1i32,
    Tight = 0i32,
}
#[cfg(feature = "UnityEngine+SpritePackingMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::SpritePackingMode => "UnityEngine"
    ."SpritePackingMode"
);
