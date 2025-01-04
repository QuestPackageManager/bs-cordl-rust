#[cfg(feature = "System+Data+MissingSchemaAction")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MissingSchemaAction {
    #[default]
    Add = 1i32,
    AddWithKey = 4i32,
    Error = 3i32,
    Ignore = 2i32,
}
#[cfg(feature = "System+Data+MissingSchemaAction")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Data::MissingSchemaAction =>
    "System.Data"."MissingSchemaAction"
);
