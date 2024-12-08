#[cfg(feature = "UnityEngine+LightType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LightType {
    Area = 3i32,
    Directional = 1i32,
    Disc = 4i32,
    Point = 2i32,
    Spot = 0i32,
}
#[cfg(feature = "UnityEngine+LightType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::LightType => "UnityEngine"
    ."LightType"
);
