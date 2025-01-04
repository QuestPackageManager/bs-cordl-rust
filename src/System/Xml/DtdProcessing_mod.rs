#[cfg(feature = "System+Xml+DtdProcessing")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DtdProcessing {
    #[default]
    Ignore = 1i32,
    Parse = 2i32,
    Prohibit = 0i32,
}
#[cfg(feature = "System+Xml+DtdProcessing")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::DtdProcessing => "System.Xml"
    ."DtdProcessing"
);
