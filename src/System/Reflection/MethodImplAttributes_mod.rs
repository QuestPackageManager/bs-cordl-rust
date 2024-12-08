#[cfg(feature = "System+Reflection+MethodImplAttributes")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MethodImplAttributes {
    AggressiveInlining = 256i32,
    CodeTypeMask = 3i32,
    ForwardRef = 16i32,
    IL = 0i32,
    InternalCall = 4096i32,
    ManagedMask = 4i32,
    MaxMethodImplVal = 65535i32,
    Native = 1i32,
    NoInlining = 8i32,
    NoOptimization = 64i32,
    OPTIL = 2i32,
    PreserveSig = 128i32,
    SecurityMitigations = 1024i32,
    Synchronized = 32i32,
}
#[cfg(feature = "System+Reflection+MethodImplAttributes")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Reflection::MethodImplAttributes =>
    "System.Reflection"."MethodImplAttributes"
);
