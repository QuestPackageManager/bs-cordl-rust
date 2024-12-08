#[cfg(feature = "System+Xml+Schema+XmlSchemaValidity")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XmlSchemaValidity {
    Invalid = 2i32,
    NotKnown = 0i32,
    Valid = 1i32,
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaValidity")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::XmlSchemaValidity =>
    "System.Xml.Schema"."XmlSchemaValidity"
);
