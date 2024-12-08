#[cfg(feature = "UnityEngine+UIElements+SortDirection")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortDirection {
    Ascending = 0i32,
    Descending = 1i32,
}
#[cfg(feature = "UnityEngine+UIElements+SortDirection")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::SortDirection =>
    "UnityEngine.UIElements"."SortDirection"
);
