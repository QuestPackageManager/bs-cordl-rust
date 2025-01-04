#[cfg(feature = "System+Xml+Schema+AttributeMatchState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AttributeMatchState {
    #[default]
    AnyAttributeLax = 4i32,
    AnyAttributeSkip = 5i32,
    AnyIdAttributeFound = 1i32,
    AttributeFound = 0i32,
    AttributeNameMismatch = 8i32,
    ProhibitedAnyAttribute = 6i32,
    ProhibitedAttribute = 7i32,
    UndeclaredAttribute = 3i32,
    UndeclaredElementAndAttribute = 2i32,
    ValidateAttributeInvalidCall = 9i32,
}
#[cfg(feature = "System+Xml+Schema+AttributeMatchState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::AttributeMatchState =>
    "System.Xml.Schema"."AttributeMatchState"
);
