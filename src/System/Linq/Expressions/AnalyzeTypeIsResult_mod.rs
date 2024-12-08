#[cfg(feature = "System+Linq+Expressions+AnalyzeTypeIsResult")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnalyzeTypeIsResult {
    KnownAssignable = 2i32,
    KnownFalse = 0i32,
    KnownTrue = 1i32,
    Unknown = 3i32,
}
#[cfg(feature = "System+Linq+Expressions+AnalyzeTypeIsResult")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Linq::Expressions::AnalyzeTypeIsResult
    => "System.Linq.Expressions"."AnalyzeTypeIsResult"
);
