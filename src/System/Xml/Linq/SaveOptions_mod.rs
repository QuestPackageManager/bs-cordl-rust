#[cfg(feature = "System+Xml+Linq+SaveOptions")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SaveOptions {
    #[default]
    DisableFormatting = 1i32,
    None = 0i32,
    OmitDuplicateNamespaces = 2i32,
}
#[cfg(feature = "System+Xml+Linq+SaveOptions")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Linq::SaveOptions =>
    "System.Xml.Linq"."SaveOptions"
);
