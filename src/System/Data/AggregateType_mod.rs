#[cfg(feature = "System+Data+AggregateType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AggregateType {
    Count = 9i32,
    First = 8i32,
    Max = 7i32,
    Mean = 5i32,
    Min = 6i32,
    None = 0i32,
    StDev = 11i32,
    Sum = 4i32,
    Var = 10i32,
}
#[cfg(feature = "System+Data+AggregateType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Data::AggregateType => "System.Data"
    ."AggregateType"
);
