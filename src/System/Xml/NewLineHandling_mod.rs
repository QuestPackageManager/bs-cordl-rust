#[cfg(feature = "System+Xml+NewLineHandling")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NewLineHandling {
    Entitize = 1i32,
    None = 2i32,
    Replace = 0i32,
}
#[cfg(feature = "System+Xml+NewLineHandling")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::NewLineHandling => "System.Xml"
    ."NewLineHandling"
);
