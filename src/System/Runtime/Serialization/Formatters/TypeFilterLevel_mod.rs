#[cfg(feature = "System+Runtime+Serialization+Formatters+TypeFilterLevel")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TypeFilterLevel {
    #[default]
    Full = 3i32,
    Low = 2i32,
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+TypeFilterLevel")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::Formatters::TypeFilterLevel =>
    "System.Runtime.Serialization.Formatters"."TypeFilterLevel"
);
