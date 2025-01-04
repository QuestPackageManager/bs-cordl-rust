#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+InternalMemberTypeE")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InternalMemberTypeE {
    #[default]
    Empty = 0i32,
    Field = 2i32,
    Header = 1i32,
    Item = 3i32,
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+InternalMemberTypeE")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::Formatters::Binary::InternalMemberTypeE =>
    "System.Runtime.Serialization.Formatters.Binary"."InternalMemberTypeE"
);
