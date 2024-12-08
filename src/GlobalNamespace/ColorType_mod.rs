#[cfg(feature = "ColorType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorType {
    ColorA = 0i32,
    ColorB = 1i32,
    None = -1i32,
}
#[cfg(feature = "ColorType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ColorType => ""."ColorType"
);
