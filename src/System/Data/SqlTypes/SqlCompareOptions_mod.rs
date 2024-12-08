#[cfg(feature = "System+Data+SqlTypes+SqlCompareOptions")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SqlCompareOptions {
    BinarySort = 32768i32,
    BinarySort2 = 16384i32,
    IgnoreCase = 1i32,
    IgnoreKanaType = 8i32,
    IgnoreNonSpace = 2i32,
    IgnoreWidth = 16i32,
    None = 0i32,
}
#[cfg(feature = "System+Data+SqlTypes+SqlCompareOptions")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Data::SqlTypes::SqlCompareOptions =>
    "System.Data.SqlTypes"."SqlCompareOptions"
);
