#[cfg(feature = "UnityEngine+SpriteSortPoint")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpriteSortPoint {
    Center = 0i32,
    Pivot = 1i32,
}
#[cfg(feature = "UnityEngine+SpriteSortPoint")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::SpriteSortPoint => "UnityEngine"
    ."SpriteSortPoint"
);
