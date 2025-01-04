#[cfg(feature = "System+Data+SqlTypes+EComparison")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EComparison {
    #[default]
    EQ = 2i32,
    GE = 3i32,
    GT = 4i32,
    LE = 1i32,
    LT = 0i32,
    NE = 5i32,
}
#[cfg(feature = "System+Data+SqlTypes+EComparison")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Data::SqlTypes::EComparison =>
    "System.Data.SqlTypes"."EComparison"
);
