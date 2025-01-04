#[cfg(feature = "System+Globalization+CompareOptions")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CompareOptions {
    #[default]
    IgnoreCase = 1i32,
    IgnoreKanaType = 8i32,
    IgnoreNonSpace = 2i32,
    IgnoreSymbols = 4i32,
    IgnoreWidth = 16i32,
    None = 0i32,
    Ordinal = 1073741824i32,
    OrdinalIgnoreCase = 268435456i32,
    StringSort = 536870912i32,
}
#[cfg(feature = "System+Globalization+CompareOptions")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Globalization::CompareOptions =>
    "System.Globalization"."CompareOptions"
);
