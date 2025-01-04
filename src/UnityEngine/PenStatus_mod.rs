#[cfg(feature = "UnityEngine+PenStatus")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PenStatus {
    #[default]
    Barrel = 2i32,
    Contact = 1i32,
    Eraser = 8i32,
    Inverted = 4i32,
    None = 0i32,
}
#[cfg(feature = "UnityEngine+PenStatus")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::PenStatus => "UnityEngine"
    ."PenStatus"
);
