#[cfg(feature = "UnityEngine+Space")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Space {
    _cordl_Self = 1i32,
    World = 0i32,
}
#[cfg(feature = "UnityEngine+Space")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Space => "UnityEngine"."Space"
);
