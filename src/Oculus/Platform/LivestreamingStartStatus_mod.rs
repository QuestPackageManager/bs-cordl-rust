#[cfg(feature = "Oculus+Platform+LivestreamingStartStatus")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LivestreamingStartStatus {
    #[default]
    MissingParameters = -4i32,
    NoFbConnect = -2i32,
    NoPackageSet = -1i32,
    NoSessionId = -3i32,
    Success = 1i32,
    Unknown = 0i32,
}
#[cfg(feature = "Oculus+Platform+LivestreamingStartStatus")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::LivestreamingStartStatus =>
    "Oculus.Platform"."LivestreamingStartStatus"
);
