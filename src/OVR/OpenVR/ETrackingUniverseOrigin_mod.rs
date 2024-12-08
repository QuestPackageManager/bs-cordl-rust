#[cfg(feature = "OVR+OpenVR+ETrackingUniverseOrigin")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ETrackingUniverseOrigin {
    TrackingUniverseRawAndUncalibrated = 2i32,
    TrackingUniverseSeated = 0i32,
    TrackingUniverseStanding = 1i32,
}
#[cfg(feature = "OVR+OpenVR+ETrackingUniverseOrigin")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::ETrackingUniverseOrigin =>
    "OVR.OpenVR"."ETrackingUniverseOrigin"
);
