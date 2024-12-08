#[cfg(feature = "OVR+OpenVR+EVRApplicationTransitionState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EVRApplicationTransitionState {
    VRApplicationTransition_NewAppLaunched = 20i32,
    VRApplicationTransition_None = 0i32,
    VRApplicationTransition_OldAppQuitSent = 10i32,
    VRApplicationTransition_WaitingForExternalLaunch = 11i32,
}
#[cfg(feature = "OVR+OpenVR+EVRApplicationTransitionState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::EVRApplicationTransitionState =>
    "OVR.OpenVR"."EVRApplicationTransitionState"
);
