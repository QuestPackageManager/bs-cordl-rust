#[cfg(feature = "OVR+OpenVR+EVRMouseButton")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EVRMouseButton {
    #[default]
    Left = 1i32,
    Middle = 4i32,
    Right = 2i32,
}
#[cfg(feature = "OVR+OpenVR+EVRMouseButton")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::EVRMouseButton => "OVR.OpenVR"
    ."EVRMouseButton"
);
