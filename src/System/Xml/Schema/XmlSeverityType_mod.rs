#[cfg(feature = "System+Xml+Schema+XmlSeverityType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XmlSeverityType {
    Error = 0i32,
    Warning = 1i32,
}
#[cfg(feature = "System+Xml+Schema+XmlSeverityType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::XmlSeverityType =>
    "System.Xml.Schema"."XmlSeverityType"
);
