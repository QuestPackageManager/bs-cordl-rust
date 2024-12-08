#[cfg(feature = "System+Reflection+TypeAttributes")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeAttributes {
    Abstract = 128i32,
    AnsiClass = 0i32,
    AutoClass = 131072i32,
    BeforeFieldInit = 1048576i32,
    ClassSemanticsMask = 32i32,
    CustomFormatClass = 196608i32,
    CustomFormatMask = 12582912i32,
    ExplicitLayout = 16i32,
    HasSecurity = 262144i32,
    Import = 4096i32,
    LayoutMask = 24i32,
    NestedAssembly = 5i32,
    NestedFamANDAssem = 6i32,
    NestedFamORAssem = 7i32,
    NestedFamily = 4i32,
    NestedPrivate = 3i32,
    NestedPublic = 2i32,
    Public = 1i32,
    RTSpecialName = 2048i32,
    ReservedMask = 264192i32,
    Sealed = 256i32,
    SequentialLayout = 8i32,
    Serializable = 8192i32,
    SpecialName = 1024i32,
    UnicodeClass = 65536i32,
    WindowsRuntime = 16384i32,
}
#[cfg(feature = "System+Reflection+TypeAttributes")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Reflection::TypeAttributes =>
    "System.Reflection"."TypeAttributes"
);
