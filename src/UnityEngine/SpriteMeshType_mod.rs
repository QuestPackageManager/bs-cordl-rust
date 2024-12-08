#[cfg(feature = "UnityEngine+SpriteMeshType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpriteMeshType {
    FullRect = 0i32,
    Tight = 1i32,
}
#[cfg(feature = "UnityEngine+SpriteMeshType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::SpriteMeshType => "UnityEngine"
    ."SpriteMeshType"
);
