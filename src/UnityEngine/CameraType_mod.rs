#[cfg(feature = "UnityEngine+CameraType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CameraType {
    Game = 1i32,
    Preview = 4i32,
    Reflection = 16i32,
    SceneView = 2i32,
    VR = 8i32,
}
#[cfg(feature = "UnityEngine+CameraType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::CameraType => "UnityEngine"
    ."CameraType"
);
