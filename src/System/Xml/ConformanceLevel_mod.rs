#[cfg(feature = "System+Xml+ConformanceLevel")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ConformanceLevel {
    #[default]
    Auto = 0i32,
    Document = 2i32,
    Fragment = 1i32,
}
#[cfg(feature = "System+Xml+ConformanceLevel")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::ConformanceLevel => "System.Xml"
    ."ConformanceLevel"
);
