#[cfg(feature = "System+Xml+Schema+RestrictionFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RestrictionFlags {
    Enumeration = 16i32,
    FractionDigits = 2048i32,
    Length = 1i32,
    MaxExclusive = 128i32,
    MaxInclusive = 64i32,
    MaxLength = 4i32,
    MinExclusive = 512i32,
    MinInclusive = 256i32,
    MinLength = 2i32,
    Pattern = 8i32,
    TotalDigits = 1024i32,
    WhiteSpace = 32i32,
}
#[cfg(feature = "System+Xml+Schema+RestrictionFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::RestrictionFlags =>
    "System.Xml.Schema"."RestrictionFlags"
);
