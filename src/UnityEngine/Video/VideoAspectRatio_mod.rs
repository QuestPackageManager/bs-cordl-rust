#[cfg(feature = "UnityEngine+Video+VideoAspectRatio")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VideoAspectRatio {
    #[default]
    FitHorizontally = 2i32,
    FitInside = 3i32,
    FitOutside = 4i32,
    FitVertically = 1i32,
    NoScaling = 0i32,
    Stretch = 5i32,
}
#[cfg(feature = "UnityEngine+Video+VideoAspectRatio")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Video::VideoAspectRatio =>
    "UnityEngine.Video"."VideoAspectRatio"
);
