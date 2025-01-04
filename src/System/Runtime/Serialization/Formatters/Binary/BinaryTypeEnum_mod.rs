#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+BinaryTypeEnum")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BinaryTypeEnum {
    #[default]
    Object = 2i32,
    ObjectArray = 5i32,
    ObjectUrt = 3i32,
    ObjectUser = 4i32,
    Primitive = 0i32,
    PrimitiveArray = 7i32,
    String = 1i32,
    StringArray = 6i32,
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+BinaryTypeEnum")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::Formatters::Binary::BinaryTypeEnum =>
    "System.Runtime.Serialization.Formatters.Binary"."BinaryTypeEnum"
);
