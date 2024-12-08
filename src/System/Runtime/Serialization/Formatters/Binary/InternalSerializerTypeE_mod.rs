#[cfg(
    feature = "System+Runtime+Serialization+Formatters+Binary+InternalSerializerTypeE"
)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InternalSerializerTypeE {
    Binary = 2i32,
    Soap = 1i32,
}
#[cfg(
    feature = "System+Runtime+Serialization+Formatters+Binary+InternalSerializerTypeE"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::Formatters::Binary::InternalSerializerTypeE =>
    "System.Runtime.Serialization.Formatters.Binary"."InternalSerializerTypeE"
);
