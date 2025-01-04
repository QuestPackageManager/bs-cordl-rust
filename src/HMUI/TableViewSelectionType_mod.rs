#[cfg(feature = "HMUI+TableViewSelectionType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TableViewSelectionType {
    #[default]
    DeselectableSingle = 3i32,
    Multiple = 2i32,
    None = 0i32,
    Single = 1i32,
}
#[cfg(feature = "HMUI+TableViewSelectionType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HMUI::TableViewSelectionType => "HMUI"
    ."TableViewSelectionType"
);
