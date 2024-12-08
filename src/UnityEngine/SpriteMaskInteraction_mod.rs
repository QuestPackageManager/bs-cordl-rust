#[cfg(feature = "UnityEngine+SpriteMaskInteraction")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpriteMaskInteraction {
    None = 0i32,
    VisibleInsideMask = 1i32,
    VisibleOutsideMask = 2i32,
}
#[cfg(feature = "UnityEngine+SpriteMaskInteraction")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::SpriteMaskInteraction =>
    "UnityEngine"."SpriteMaskInteraction"
);
