#[cfg(feature = "UnityEngine+LightShadows")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LightShadows {
    Hard = 1i32,
    None = 0i32,
    Soft = 2i32,
}
#[cfg(feature = "UnityEngine+LightShadows")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::LightShadows => "UnityEngine"
    ."LightShadows"
);
