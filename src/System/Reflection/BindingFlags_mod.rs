#[cfg(feature = "System+Reflection+BindingFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BindingFlags {
    CreateInstance = 512i32,
    DeclaredOnly = 2i32,
    Default = 0i32,
    DoNotWrapExceptions = 33554432i32,
    ExactBinding = 65536i32,
    FlattenHierarchy = 64i32,
    GetField = 1024i32,
    GetProperty = 4096i32,
    IgnoreCase = 1i32,
    IgnoreReturn = 16777216i32,
    Instance = 4i32,
    InvokeMethod = 256i32,
    NonPublic = 32i32,
    OptionalParamBinding = 262144i32,
    Public = 16i32,
    PutDispProperty = 16384i32,
    PutRefDispProperty = 32768i32,
    SetField = 2048i32,
    SetProperty = 8192i32,
    Static = 8i32,
    SuppressChangeType = 131072i32,
}
#[cfg(feature = "System+Reflection+BindingFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Reflection::BindingFlags =>
    "System.Reflection"."BindingFlags"
);
