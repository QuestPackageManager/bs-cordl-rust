#[cfg(feature = "UnityEngine+LightmapsMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LightmapsMode {
    #[default]
    CombinedDirectional = 1i32,
    NonDirectional = 0i32,
}
#[cfg(feature = "UnityEngine+LightmapsMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::LightmapsMode => "UnityEngine"
    ."LightmapsMode"
);
