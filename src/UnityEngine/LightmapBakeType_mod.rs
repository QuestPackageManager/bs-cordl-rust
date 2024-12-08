#[cfg(feature = "UnityEngine+LightmapBakeType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LightmapBakeType {
    Baked = 2i32,
    Mixed = 1i32,
    Realtime = 4i32,
}
#[cfg(feature = "UnityEngine+LightmapBakeType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::LightmapBakeType => "UnityEngine"
    ."LightmapBakeType"
);
