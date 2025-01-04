#[cfg(feature = "System+Xml+WhitespaceHandling")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WhitespaceHandling {
    #[default]
    All = 0i32,
    None = 2i32,
    Significant = 1i32,
}
#[cfg(feature = "System+Xml+WhitespaceHandling")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::WhitespaceHandling => "System.Xml"
    ."WhitespaceHandling"
);
