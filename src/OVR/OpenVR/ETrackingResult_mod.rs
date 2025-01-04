#[cfg(feature = "OVR+OpenVR+ETrackingResult")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ETrackingResult {
    #[default]
    Calibrating_InProgress = 100i32,
    Calibrating_OutOfRange = 101i32,
    Running_OK = 200i32,
    Running_OutOfRange = 201i32,
    Uninitialized = 1i32,
}
#[cfg(feature = "OVR+OpenVR+ETrackingResult")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::ETrackingResult => "OVR.OpenVR"
    ."ETrackingResult"
);
