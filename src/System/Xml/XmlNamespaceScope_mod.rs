#[cfg(feature = "System+Xml+XmlNamespaceScope")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum XmlNamespaceScope {
    #[default]
    All = 0i32,
    ExcludeXml = 1i32,
    Local = 2i32,
}
#[cfg(feature = "System+Xml+XmlNamespaceScope")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlNamespaceScope => "System.Xml"
    ."XmlNamespaceScope"
);
