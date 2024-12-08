#[cfg(feature = "LightRotationDirection")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LightRotationDirection {
    Automatic = 0i32,
    Clockwise = 1i32,
    Counterclockwise = 2i32,
}
#[cfg(feature = "LightRotationDirection")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for LightRotationDirection => ""."LightRotationDirection"
);
