#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+InternalMemberValueE")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InternalMemberValueE {
    #[default]
    Empty = 0i32,
    InlineValue = 1i32,
    Nested = 2i32,
    Null = 4i32,
    Reference = 3i32,
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+InternalMemberValueE")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::Formatters::Binary::InternalMemberValueE =>
    "System.Runtime.Serialization.Formatters.Binary"."InternalMemberValueE"
);
