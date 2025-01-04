#[cfg(
    feature = "System+Runtime+Serialization+Formatters+Binary+InternalObjectPositionE"
)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InternalObjectPositionE {
    #[default]
    Child = 2i32,
    Empty = 0i32,
    Headers = 3i32,
    Top = 1i32,
}
#[cfg(
    feature = "System+Runtime+Serialization+Formatters+Binary+InternalObjectPositionE"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::Formatters::Binary::InternalObjectPositionE =>
    "System.Runtime.Serialization.Formatters.Binary"."InternalObjectPositionE"
);
