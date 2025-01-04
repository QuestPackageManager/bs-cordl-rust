#[cfg(feature = "System+Xml+XmlDateTimeSerializationMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum XmlDateTimeSerializationMode {
    #[default]
    Local = 0i32,
    RoundtripKind = 3i32,
    Unspecified = 2i32,
    Utc = 1i32,
}
#[cfg(feature = "System+Xml+XmlDateTimeSerializationMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlDateTimeSerializationMode =>
    "System.Xml"."XmlDateTimeSerializationMode"
);
