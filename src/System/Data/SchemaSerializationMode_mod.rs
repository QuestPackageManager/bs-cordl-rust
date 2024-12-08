#[cfg(feature = "System+Data+SchemaSerializationMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchemaSerializationMode {
    ExcludeSchema = 2i32,
    IncludeSchema = 1i32,
}
#[cfg(feature = "System+Data+SchemaSerializationMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Data::SchemaSerializationMode =>
    "System.Data"."SchemaSerializationMode"
);
