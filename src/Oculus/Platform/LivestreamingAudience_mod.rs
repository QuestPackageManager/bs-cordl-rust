#[cfg(feature = "Oculus+Platform+LivestreamingAudience")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LivestreamingAudience {
    #[default]
    Friends = 2i32,
    OnlyMe = 3i32,
    Public = 1i32,
    Unknown = 0i32,
}
#[cfg(feature = "Oculus+Platform+LivestreamingAudience")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::LivestreamingAudience =>
    "Oculus.Platform"."LivestreamingAudience"
);
