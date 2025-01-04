#[cfg(feature = "OVR+OpenVR+EVRApplicationProperty")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EVRApplicationProperty {
    #[default]
    ActionManifestURL_String = 54i32,
    Arguments_String = 14i32,
    BinaryPath_String = 13i32,
    Description_String = 50i32,
    ImagePath_String = 52i32,
    IsDashboardOverlay_Bool = 60i32,
    IsInstanced_Bool = 62i32,
    IsInternal_Bool = 63i32,
    IsTemplate_Bool = 61i32,
    LastLaunchTime_Uint64 = 70i32,
    LaunchType_String = 11i32,
    Name_String = 0i32,
    NewsURL_String = 51i32,
    Source_String = 53i32,
    URL_String = 15i32,
    WantsCompositorPauseInStandby_Bool = 64i32,
    WorkingDirectory_String = 12i32,
}
#[cfg(feature = "OVR+OpenVR+EVRApplicationProperty")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::EVRApplicationProperty =>
    "OVR.OpenVR"."EVRApplicationProperty"
);
