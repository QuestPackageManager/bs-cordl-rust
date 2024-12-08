#[cfg(feature = "Oculus+Platform+ReportRequestResponse")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReportRequestResponse {
    Handled = 1i32,
    Unavailable = 3i32,
    Unhandled = 2i32,
    Unknown = 0i32,
}
#[cfg(feature = "Oculus+Platform+ReportRequestResponse")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::ReportRequestResponse =>
    "Oculus.Platform"."ReportRequestResponse"
);
