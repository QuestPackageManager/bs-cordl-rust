#[cfg(feature = "UnityEngine+ProBuilder+SortMethod")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SortMethod {
    #[default]
    Clockwise = 0i32,
    CounterClockwise = 1i32,
}
#[cfg(feature = "UnityEngine+ProBuilder+SortMethod")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::SortMethod =>
    "UnityEngine.ProBuilder"."SortMethod"
);
