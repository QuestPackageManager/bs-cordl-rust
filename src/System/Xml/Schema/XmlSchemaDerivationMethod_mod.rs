#[cfg(feature = "System+Xml+Schema+XmlSchemaDerivationMethod")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XmlSchemaDerivationMethod {
    All = 255i32,
    Empty = 0i32,
    Extension = 2i32,
    List = 8i32,
    None = 256i32,
    Restriction = 4i32,
    Substitution = 1i32,
    Union = 16i32,
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaDerivationMethod")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::XmlSchemaDerivationMethod
    => "System.Xml.Schema"."XmlSchemaDerivationMethod"
);
