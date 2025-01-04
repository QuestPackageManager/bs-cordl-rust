#[cfg(feature = "System+Xml+Schema+XmlSchemaDatatypeVariety")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum XmlSchemaDatatypeVariety {
    #[default]
    Atomic = 0i32,
    List = 1i32,
    Union = 2i32,
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaDatatypeVariety")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::XmlSchemaDatatypeVariety =>
    "System.Xml.Schema"."XmlSchemaDatatypeVariety"
);
