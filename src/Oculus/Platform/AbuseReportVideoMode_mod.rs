#[cfg(feature = "Oculus+Platform+AbuseReportVideoMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AbuseReportVideoMode {
    #[default]
    Collect = 1i32,
    Optional = 2i32,
    Skip = 3i32,
    Unknown = 0i32,
}
#[cfg(feature = "Oculus+Platform+AbuseReportVideoMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::AbuseReportVideoMode =>
    "Oculus.Platform"."AbuseReportVideoMode"
);
