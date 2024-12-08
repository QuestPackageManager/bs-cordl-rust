#[cfg(feature = "OVR+OpenVR+EVRInputFilterCancelType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EVRInputFilterCancelType {
    VRInputFilterCancel_Momentum = 1i32,
    VRInputFilterCancel_Timers = 0i32,
}
#[cfg(feature = "OVR+OpenVR+EVRInputFilterCancelType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::EVRInputFilterCancelType =>
    "OVR.OpenVR"."EVRInputFilterCancelType"
);
