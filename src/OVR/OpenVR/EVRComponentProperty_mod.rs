#[cfg(feature = "OVR+OpenVR+EVRComponentProperty")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EVRComponentProperty {
    IsPressed = 8i32,
    IsScrolled = 16i32,
    IsStatic = 1i32,
    IsTouched = 4i32,
    IsVisible = 2i32,
}
#[cfg(feature = "OVR+OpenVR+EVRComponentProperty")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::EVRComponentProperty =>
    "OVR.OpenVR"."EVRComponentProperty"
);
