#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+BinaryHeaderEnum")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryHeaderEnum {
    Array = 7i32,
    ArraySingleObject = 16i32,
    ArraySinglePrimitive = 15i32,
    ArraySingleString = 17i32,
    Assembly = 12i32,
    CrossAppDomainAssembly = 20i32,
    CrossAppDomainMap = 18i32,
    CrossAppDomainString = 19i32,
    MemberPrimitiveTyped = 8i32,
    MemberReference = 9i32,
    MessageEnd = 11i32,
    MethodCall = 21i32,
    MethodReturn = 22i32,
    Object = 1i32,
    ObjectNull = 10i32,
    ObjectNullMultiple = 14i32,
    ObjectNullMultiple256 = 13i32,
    ObjectString = 6i32,
    ObjectWithMap = 2i32,
    ObjectWithMapAssemId = 3i32,
    ObjectWithMapTyped = 4i32,
    ObjectWithMapTypedAssemId = 5i32,
    SerializedStreamHeader = 0i32,
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+BinaryHeaderEnum")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::Formatters::Binary::BinaryHeaderEnum =>
    "System.Runtime.Serialization.Formatters.Binary"."BinaryHeaderEnum"
);
