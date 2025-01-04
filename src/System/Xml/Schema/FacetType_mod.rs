#[cfg(feature = "System+Xml+Schema+FacetType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FacetType {
    #[default]
    Enumeration = 6i32,
    FractionDigits = 12i32,
    Length = 1i32,
    MaxExclusive = 9i32,
    MaxInclusive = 10i32,
    MaxLength = 3i32,
    MinExclusive = 7i32,
    MinInclusive = 8i32,
    MinLength = 2i32,
    None = 0i32,
    Pattern = 4i32,
    TotalDigits = 11i32,
    Whitespace = 5i32,
}
#[cfg(feature = "System+Xml+Schema+FacetType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::FacetType =>
    "System.Xml.Schema"."FacetType"
);
