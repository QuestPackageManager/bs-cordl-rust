#[cfg(feature = "System+Xml+XPath+XPathNamespaceScope")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum XPathNamespaceScope {
    #[default]
    All = 0i32,
    ExcludeXml = 1i32,
    Local = 2i32,
}
#[cfg(feature = "System+Xml+XPath+XPathNamespaceScope")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XPath::XPathNamespaceScope =>
    "System.Xml.XPath"."XPathNamespaceScope"
);
