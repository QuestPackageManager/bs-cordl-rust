#[cfg(feature = "TMPro+ColorMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColorMode {
    #[default]
    FourCornersGradient = 3i32,
    HorizontalGradient = 1i32,
    Single = 0i32,
    VerticalGradient = 2i32,
}
#[cfg(feature = "TMPro+ColorMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::TMPro::ColorMode => "TMPro"."ColorMode"
);
