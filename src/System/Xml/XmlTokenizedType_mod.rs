#[cfg(feature = "System+Xml+XmlTokenizedType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum XmlTokenizedType {
    #[default]
    CDATA = 0i32,
    ENTITIES = 5i32,
    ENTITY = 4i32,
    ENUMERATION = 9i32,
    _cordl_ID = 1i32,
    IDREF = 2i32,
    IDREFS = 3i32,
    NCName = 11i32,
    NMTOKEN = 6i32,
    NMTOKENS = 7i32,
    NOTATION = 8i32,
    None = 12i32,
    QName = 10i32,
}
#[cfg(feature = "System+Xml+XmlTokenizedType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlTokenizedType => "System.Xml"
    ."XmlTokenizedType"
);
