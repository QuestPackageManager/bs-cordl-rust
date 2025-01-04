#[cfg(feature = "UnityEngine+ProBuilder+PivotPoint")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PivotPoint {
    #[default]
    ActiveElement = 2i32,
    Center = 0i32,
    IndividualOrigins = 1i32,
}
#[cfg(feature = "UnityEngine+ProBuilder+PivotPoint")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::PivotPoint =>
    "UnityEngine.ProBuilder"."PivotPoint"
);
