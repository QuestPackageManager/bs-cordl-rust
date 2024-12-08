#[cfg(feature = "System+Enum+ParseFailureKind")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Enum_ParseFailureKind {
    Argument = 1i32,
    ArgumentNull = 2i32,
    ArgumentWithParameter = 3i32,
    None = 0i32,
    UnhandledException = 4i32,
}
#[cfg(feature = "System+Enum+ParseFailureKind")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::Enum_ParseFailureKind =>
    "System"."Enum/ParseFailureKind"
);
