#[cfg(feature = "System+TypeNameFormatFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeNameFormatFlags {
    FormatAngleBrackets = 64i32,
    FormatAssembly = 4i32,
    FormatBasic = 0i32,
    FormatFullInst = 2i32,
    FormatGenericParam = 256i32,
    FormatNamespace = 1i32,
    FormatNoVersion = 16i32,
    FormatSerialization = 259i32,
    FormatSignature = 8i32,
    FormatStubInfo = 128i32,
}
#[cfg(feature = "System+TypeNameFormatFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::TypeNameFormatFlags => "System"
    ."TypeNameFormatFlags"
);
