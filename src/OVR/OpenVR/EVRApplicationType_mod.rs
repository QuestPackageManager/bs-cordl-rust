#[cfg(feature = "OVR+OpenVR+EVRApplicationType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EVRApplicationType {
    #[default]
    VRApplication_Background = 3i32,
    VRApplication_Bootstrapper = 7i32,
    VRApplication_Max = 8i32,
    VRApplication_Other = 0i32,
    VRApplication_Overlay = 2i32,
    VRApplication_Scene = 1i32,
    VRApplication_SteamWatchdog = 6i32,
    VRApplication_Utility = 4i32,
    VRApplication_VRMonitor = 5i32,
}
#[cfg(feature = "OVR+OpenVR+EVRApplicationType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::EVRApplicationType => "OVR.OpenVR"
    ."EVRApplicationType"
);
