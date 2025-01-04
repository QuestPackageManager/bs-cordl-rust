#[cfg(feature = "System+Xml+NamespaceHandling")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NamespaceHandling {
    #[default]
    Default = 0i32,
    OmitDuplicates = 1i32,
}
#[cfg(feature = "System+Xml+NamespaceHandling")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::NamespaceHandling => "System.Xml"
    ."NamespaceHandling"
);
