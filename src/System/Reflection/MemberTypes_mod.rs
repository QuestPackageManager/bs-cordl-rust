#[cfg(feature = "System+Reflection+MemberTypes")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MemberTypes {
    #[default]
    All = 191i32,
    Constructor = 1i32,
    Custom = 64i32,
    Event = 2i32,
    Field = 4i32,
    Method = 8i32,
    NestedType = 128i32,
    Property = 16i32,
    TypeInfo = 32i32,
}
#[cfg(feature = "System+Reflection+MemberTypes")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Reflection::MemberTypes =>
    "System.Reflection"."MemberTypes"
);
