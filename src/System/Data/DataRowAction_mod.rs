#[cfg(feature = "System+Data+DataRowAction")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataRowAction {
    Add = 16i32,
    Change = 2i32,
    ChangeCurrentAndOriginal = 64i32,
    ChangeOriginal = 32i32,
    Commit = 8i32,
    Delete = 1i32,
    Nothing = 0i32,
    Rollback = 4i32,
}
#[cfg(feature = "System+Data+DataRowAction")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Data::DataRowAction => "System.Data"
    ."DataRowAction"
);
