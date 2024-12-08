#[cfg(feature = "System+Reflection+MethodAttributes")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MethodAttributes {
    Abstract = 1024i32,
    Assembly = 3i32,
    CheckAccessOnOverride = 512i32,
    FamANDAssem = 2i32,
    FamORAssem = 5i32,
    Family = 4i32,
    Final = 32i32,
    HasSecurity = 16384i32,
    HideBySig = 128i32,
    MemberAccessMask = 7i32,
    NewSlot = 256i32,
    PinvokeImpl = 8192i32,
    Private = 1i32,
    PrivateScope = 0i32,
    Public = 6i32,
    RTSpecialName = 4096i32,
    RequireSecObject = 32768i32,
    ReservedMask = 53248i32,
    SpecialName = 2048i32,
    Static = 16i32,
    UnmanagedExport = 8i32,
    Virtual = 64i32,
}
#[cfg(feature = "System+Reflection+MethodAttributes")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Reflection::MethodAttributes =>
    "System.Reflection"."MethodAttributes"
);
