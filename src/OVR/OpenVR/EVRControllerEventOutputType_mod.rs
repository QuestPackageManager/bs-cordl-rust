#[cfg(feature = "OVR+OpenVR+EVRControllerEventOutputType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EVRControllerEventOutputType {
    ControllerEventOutput_OSEvents = 0i32,
    ControllerEventOutput_VREvents = 1i32,
}
#[cfg(feature = "OVR+OpenVR+EVRControllerEventOutputType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::EVRControllerEventOutputType =>
    "OVR.OpenVR"."EVRControllerEventOutputType"
);
