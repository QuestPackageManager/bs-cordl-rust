#[cfg(feature = "System+Data+DataViewRowState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataViewRowState {
    Added = 4i32,
    CurrentRows = 22i32,
    Deleted = 8i32,
    ModifiedCurrent = 16i32,
    ModifiedOriginal = 32i32,
    None = 0i32,
    OriginalRows = 42i32,
    Unchanged = 2i32,
}
#[cfg(feature = "System+Data+DataViewRowState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Data::DataViewRowState => "System.Data"
    ."DataViewRowState"
);
