#[cfg(feature = "OVR+OpenVR+EColorSpace")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EColorSpace {
    #[default]
    Auto = 0i32,
    Gamma = 1i32,
    Linear = 2i32,
}
#[cfg(feature = "OVR+OpenVR+EColorSpace")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::EColorSpace => "OVR.OpenVR"
    ."EColorSpace"
);
