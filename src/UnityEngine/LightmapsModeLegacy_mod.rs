#[cfg(feature = "UnityEngine+LightmapsModeLegacy")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LightmapsModeLegacy {
    Directional = 2i32,
    Dual = 1i32,
    Single = 0i32,
}
#[cfg(feature = "UnityEngine+LightmapsModeLegacy")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::LightmapsModeLegacy =>
    "UnityEngine"."LightmapsModeLegacy"
);
