#[cfg(feature = "OVR+OpenVR+EChaperoneConfigFile")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EChaperoneConfigFile {
    #[default]
    Live = 1i32,
    Temp = 2i32,
}
#[cfg(feature = "OVR+OpenVR+EChaperoneConfigFile")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::EChaperoneConfigFile =>
    "OVR.OpenVR"."EChaperoneConfigFile"
);
