#[cfg(feature = "System+Xml+Linq+LoadOptions")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadOptions {
    None = 0i32,
    PreserveWhitespace = 1i32,
    SetBaseUri = 2i32,
    SetLineInfo = 4i32,
}
#[cfg(feature = "System+Xml+Linq+LoadOptions")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Linq::LoadOptions =>
    "System.Xml.Linq"."LoadOptions"
);
