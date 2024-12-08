#[cfg(feature = "OVR+OpenVR+ETrackedDeviceClass")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ETrackedDeviceClass {
    Controller = 2i32,
    DisplayRedirect = 5i32,
    GenericTracker = 3i32,
    HMD = 1i32,
    Invalid = 0i32,
    Max = 6i32,
    TrackingReference = 4i32,
}
#[cfg(feature = "OVR+OpenVR+ETrackedDeviceClass")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::ETrackedDeviceClass => "OVR.OpenVR"
    ."ETrackedDeviceClass"
);
