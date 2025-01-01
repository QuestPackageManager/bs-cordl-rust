#[cfg(feature = "GradientDirection")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GradientDirection {
    Horizontal = 0i32,
    Vertical = 1i32,
}
#[cfg(feature = "GradientDirection")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::GradientDirection => ""
    ."GradientDirection"
);
