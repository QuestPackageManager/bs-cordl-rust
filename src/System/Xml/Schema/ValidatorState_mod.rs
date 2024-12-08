#[cfg(feature = "System+Xml+Schema+ValidatorState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidatorState {
    Attribute = 5i32,
    Element = 4i32,
    EndElement = 9i32,
    EndOfAttributes = 6i32,
    Finish = 11i32,
    None = 0i32,
    SkipToEndElement = 10i32,
    Start = 1i32,
    Text = 7i32,
    TopLevelAttribute = 2i32,
    TopLevelTextOrWS = 3i32,
    Whitespace = 8i32,
}
#[cfg(feature = "System+Xml+Schema+ValidatorState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::ValidatorState =>
    "System.Xml.Schema"."ValidatorState"
);
