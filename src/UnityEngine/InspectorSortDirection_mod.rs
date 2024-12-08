#[cfg(feature = "UnityEngine+InspectorSortDirection")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InspectorSortDirection {
    Ascending = 0i32,
    Descending = 1i32,
}
#[cfg(feature = "UnityEngine+InspectorSortDirection")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InspectorSortDirection =>
    "UnityEngine"."InspectorSortDirection"
);
