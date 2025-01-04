#[cfg(feature = "System+Data+TreeAccessMethod")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TreeAccessMethod {
    #[default]
    INDEX_ONLY = 2i32,
    KEY_SEARCH_AND_INDEX = 1i32,
}
#[cfg(feature = "System+Data+TreeAccessMethod")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Data::TreeAccessMethod => "System.Data"
    ."TreeAccessMethod"
);
