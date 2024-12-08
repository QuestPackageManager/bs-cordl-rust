#[cfg(feature = "ColorChangeUIEventType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorChangeUIEventType {
    Drag = 0i32,
    PointerUp = 1i32,
}
#[cfg(feature = "ColorChangeUIEventType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for ColorChangeUIEventType => ""."ColorChangeUIEventType"
);
