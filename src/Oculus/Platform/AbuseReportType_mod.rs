#[cfg(feature = "Oculus+Platform+AbuseReportType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbuseReportType {
    Object = 1i32,
    Unknown = 0i32,
    User = 2i32,
}
#[cfg(feature = "Oculus+Platform+AbuseReportType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::AbuseReportType =>
    "Oculus.Platform"."AbuseReportType"
);
