#[cfg(feature = "OVR+OpenVR+ETrackedControllerRole")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ETrackedControllerRole {
    #[default]
    Invalid = 0i32,
    LeftHand = 1i32,
    Max = 4i32,
    OptOut = 3i32,
    RightHand = 2i32,
}
#[cfg(feature = "OVR+OpenVR+ETrackedControllerRole")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::ETrackedControllerRole =>
    "OVR.OpenVR"."ETrackedControllerRole"
);
