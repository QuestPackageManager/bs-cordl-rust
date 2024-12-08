#[cfg(feature = "System+Data+MappingType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MappingType {
    Attribute = 2i32,
    Element = 1i32,
    Hidden = 4i32,
    SimpleContent = 3i32,
}
#[cfg(feature = "System+Data+MappingType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Data::MappingType => "System.Data"
    ."MappingType"
);
