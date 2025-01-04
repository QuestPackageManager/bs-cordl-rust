#[cfg(feature = "OVR+OpenVR+EVSync")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EVSync {
    #[default]
    NoWaitRender = 2i32,
    None = 0i32,
    WaitRender = 1i32,
}
#[cfg(feature = "OVR+OpenVR+EVSync")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::EVSync => "OVR.OpenVR"."EVSync"
);
