#[cfg(feature = "OVR+OpenVR+EChaperoneImportFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EChaperoneImportFlags {
    #[default]
    EChaperoneImport_BoundsOnly = 1i32,
}
#[cfg(feature = "OVR+OpenVR+EChaperoneImportFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::EChaperoneImportFlags =>
    "OVR.OpenVR"."EChaperoneImportFlags"
);
