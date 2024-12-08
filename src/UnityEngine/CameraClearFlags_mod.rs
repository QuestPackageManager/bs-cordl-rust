#[cfg(feature = "UnityEngine+CameraClearFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CameraClearFlags {
    Color = 2i32,
    Depth = 3i32,
    Nothing = 4i32,
    Skybox = 1i32,
}
#[cfg(feature = "UnityEngine+CameraClearFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::CameraClearFlags => "UnityEngine"
    ."CameraClearFlags"
);
