#[cfg(feature = "System+Reflection+CorElementType")]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CorElementType {
    #[default]
    Array = 20u8,
    Boolean = 2u8,
    ByRef = 16u8,
    CModOpt = 32u8,
    CModReqd = 31u8,
    Char = 3u8,
    Class = 18u8,
    ELEMENT_TYPE_END = 0u8,
    ELEMENT_TYPE_FNPTR = 27u8,
    ELEMENT_TYPE_GENERICINST = 21u8,
    ELEMENT_TYPE_I = 24u8,
    ELEMENT_TYPE_I1 = 4u8,
    ELEMENT_TYPE_I2 = 6u8,
    ELEMENT_TYPE_I4 = 8u8,
    ELEMENT_TYPE_I8 = 10u8,
    ELEMENT_TYPE_INTERNAL = 33u8,
    ELEMENT_TYPE_MAX = 34u8,
    ELEMENT_TYPE_MODIFIER = 64u8,
    ELEMENT_TYPE_MVAR = 30u8,
    ELEMENT_TYPE_OBJECT = 28u8,
    ELEMENT_TYPE_PINNED = 69u8,
    ELEMENT_TYPE_PTR = 15u8,
    ELEMENT_TYPE_R4 = 12u8,
    ELEMENT_TYPE_R8 = 13u8,
    ELEMENT_TYPE_SENTINEL = 65u8,
    ELEMENT_TYPE_STRING = 14u8,
    ELEMENT_TYPE_SZARRAY = 29u8,
    ELEMENT_TYPE_TYPEDBYREF = 22u8,
    ELEMENT_TYPE_U = 25u8,
    ELEMENT_TYPE_U1 = 5u8,
    ELEMENT_TYPE_U2 = 7u8,
    ELEMENT_TYPE_U4 = 9u8,
    ELEMENT_TYPE_U8 = 11u8,
    ELEMENT_TYPE_VALUETYPE = 17u8,
    ELEMENT_TYPE_VAR = 19u8,
    ELEMENT_TYPE_VOID = 1u8,
}
#[cfg(feature = "System+Reflection+CorElementType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Reflection::CorElementType =>
    "System.Reflection"."CorElementType"
);
