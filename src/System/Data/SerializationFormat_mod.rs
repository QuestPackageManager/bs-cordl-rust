#[cfg(feature = "System+Data+SerializationFormat")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SerializationFormat {
    Binary = 1i32,
    Xml = 0i32,
}
#[cfg(feature = "System+Data+SerializationFormat")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Data::SerializationFormat =>
    "System.Data"."SerializationFormat"
);
