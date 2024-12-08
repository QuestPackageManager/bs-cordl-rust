#[cfg(feature = "UnityEngine+ProBuilder+PivotLocation")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PivotLocation {
    Center = 0i32,
    FirstCorner = 1i32,
}
#[cfg(feature = "UnityEngine+ProBuilder+PivotLocation")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::PivotLocation =>
    "UnityEngine.ProBuilder"."PivotLocation"
);
