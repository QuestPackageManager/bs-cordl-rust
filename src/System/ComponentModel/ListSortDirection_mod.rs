#[cfg(feature = "System+ComponentModel+ListSortDirection")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListSortDirection {
    Ascending = 0i32,
    Descending = 1i32,
}
#[cfg(feature = "System+ComponentModel+ListSortDirection")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::ComponentModel::ListSortDirection =>
    "System.ComponentModel"."ListSortDirection"
);
