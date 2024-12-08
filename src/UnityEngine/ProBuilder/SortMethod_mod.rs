#[cfg(feature = "UnityEngine+ProBuilder+SortMethod")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortMethod {
    Clockwise = 0i32,
    CounterClockwise = 1i32,
}
#[cfg(feature = "UnityEngine+ProBuilder+SortMethod")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::SortMethod =>
    "UnityEngine.ProBuilder"."SortMethod"
);
