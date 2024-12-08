#[cfg(feature = "UnityEngine+TextCore+Text+ColorGradientMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorGradientMode {
    FourCornersGradient = 3i32,
    HorizontalGradient = 1i32,
    Single = 0i32,
    VerticalGradient = 2i32,
}
#[cfg(feature = "UnityEngine+TextCore+Text+ColorGradientMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TextCore::Text::ColorGradientMode
    => "UnityEngine.TextCore.Text"."ColorGradientMode"
);
