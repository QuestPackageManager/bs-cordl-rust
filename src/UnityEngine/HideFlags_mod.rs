#[cfg(feature = "UnityEngine+HideFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HideFlags {
    DontSave = 52i32,
    DontSaveInBuild = 16i32,
    DontSaveInEditor = 4i32,
    DontUnloadUnusedAsset = 32i32,
    HideAndDontSave = 61i32,
    HideInHierarchy = 1i32,
    HideInInspector = 2i32,
    None = 0i32,
    NotEditable = 8i32,
}
#[cfg(feature = "UnityEngine+HideFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::HideFlags => "UnityEngine"
    ."HideFlags"
);
