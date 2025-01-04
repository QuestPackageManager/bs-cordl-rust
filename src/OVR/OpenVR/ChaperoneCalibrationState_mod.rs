#[cfg(feature = "OVR+OpenVR+ChaperoneCalibrationState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ChaperoneCalibrationState {
    #[default]
    Error = 200i32,
    Error_BaseStationConflict = 202i32,
    Error_BaseStationUninitialized = 201i32,
    Error_CollisionBoundsInvalid = 204i32,
    Error_PlayAreaInvalid = 203i32,
    OK = 1i32,
    Warning = 100i32,
    Warning_BaseStationMayHaveMoved = 101i32,
    Warning_BaseStationRemoved = 102i32,
    Warning_SeatedBoundsInvalid = 103i32,
}
#[cfg(feature = "OVR+OpenVR+ChaperoneCalibrationState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::ChaperoneCalibrationState =>
    "OVR.OpenVR"."ChaperoneCalibrationState"
);
