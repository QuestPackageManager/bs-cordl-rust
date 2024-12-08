#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+ValueFixupEnum")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueFixupEnum {
    Array = 1i32,
    Empty = 0i32,
    Header = 2i32,
    Member = 3i32,
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+ValueFixupEnum")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::Formatters::Binary::ValueFixupEnum =>
    "System.Runtime.Serialization.Formatters.Binary"."ValueFixupEnum"
);
