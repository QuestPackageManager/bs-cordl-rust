#[cfg(feature = "UnityEngine+ColorGamut")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColorGamut {
    #[default]
    DisplayP3 = 3i32,
    DolbyHDR = 5i32,
    HDR10 = 4i32,
    P3D65G22 = 6i32,
    Rec2020 = 2i32,
    Rec709 = 1i32,
    sRGB = 0i32,
}
#[cfg(feature = "UnityEngine+ColorGamut")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ColorGamut => "UnityEngine"
    ."ColorGamut"
);
