#[cfg(feature = "UnityEngine+ScreenOrientation")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ScreenOrientation {
    #[default]
    AutoRotation = 5i32,
    Landscape = 3i32,
    LandscapeRight = 4i32,
    Portrait = 1i32,
    PortraitUpsideDown = 2i32,
    Unknown = 0i32,
}
#[cfg(feature = "UnityEngine+ScreenOrientation")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ScreenOrientation => "UnityEngine"
    ."ScreenOrientation"
);
