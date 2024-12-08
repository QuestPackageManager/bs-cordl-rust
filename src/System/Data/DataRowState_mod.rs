#[cfg(feature = "System+Data+DataRowState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataRowState {
    Added = 4i32,
    Deleted = 8i32,
    Detached = 1i32,
    Modified = 16i32,
    Unchanged = 2i32,
}
#[cfg(feature = "System+Data+DataRowState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Data::DataRowState => "System.Data"
    ."DataRowState"
);
