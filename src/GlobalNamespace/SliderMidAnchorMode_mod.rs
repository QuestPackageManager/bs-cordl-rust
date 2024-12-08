#[cfg(feature = "SliderMidAnchorMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SliderMidAnchorMode {
    Clockwise = 1i32,
    CounterClockwise = 2i32,
    Straight = 0i32,
}
#[cfg(feature = "SliderMidAnchorMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SliderMidAnchorMode => ""
    ."SliderMidAnchorMode"
);
