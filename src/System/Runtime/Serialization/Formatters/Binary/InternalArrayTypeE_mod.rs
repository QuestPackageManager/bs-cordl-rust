#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+InternalArrayTypeE")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InternalArrayTypeE {
    Base64 = 4i32,
    Empty = 0i32,
    Jagged = 2i32,
    Rectangular = 3i32,
    Single = 1i32,
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+InternalArrayTypeE")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::Formatters::Binary::InternalArrayTypeE =>
    "System.Runtime.Serialization.Formatters.Binary"."InternalArrayTypeE"
);
