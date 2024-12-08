#[cfg(feature = "System+Reflection+FieldAttributes")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldAttributes {
    Assembly = 3i32,
    FamANDAssem = 2i32,
    FamORAssem = 5i32,
    Family = 4i32,
    FieldAccessMask = 7i32,
    HasDefault = 32768i32,
    HasFieldMarshal = 4096i32,
    HasFieldRVA = 256i32,
    InitOnly = 32i32,
    Literal = 64i32,
    NotSerialized = 128i32,
    PinvokeImpl = 8192i32,
    Private = 1i32,
    PrivateScope = 0i32,
    Public = 6i32,
    RTSpecialName = 1024i32,
    ReservedMask = 38144i32,
    SpecialName = 512i32,
    Static = 16i32,
}
#[cfg(feature = "System+Reflection+FieldAttributes")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Reflection::FieldAttributes =>
    "System.Reflection"."FieldAttributes"
);
