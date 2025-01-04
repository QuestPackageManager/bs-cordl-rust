#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+InternalObjectTypeE")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InternalObjectTypeE {
    #[default]
    Array = 2i32,
    Empty = 0i32,
    Object = 1i32,
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+InternalObjectTypeE")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::Formatters::Binary::InternalObjectTypeE =>
    "System.Runtime.Serialization.Formatters.Binary"."InternalObjectTypeE"
);
