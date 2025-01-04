#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+BinaryArrayTypeEnum")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BinaryArrayTypeEnum {
    #[default]
    Jagged = 1i32,
    JaggedOffset = 4i32,
    Rectangular = 2i32,
    RectangularOffset = 5i32,
    Single = 0i32,
    SingleOffset = 3i32,
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+BinaryArrayTypeEnum")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::Formatters::Binary::BinaryArrayTypeEnum =>
    "System.Runtime.Serialization.Formatters.Binary"."BinaryArrayTypeEnum"
);
