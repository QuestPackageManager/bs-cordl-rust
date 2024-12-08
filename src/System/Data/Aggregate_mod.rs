#[cfg(feature = "System+Data+Aggregate")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Aggregate {
    Avg = 31i32,
    Count = 34i32,
    Max = 33i32,
    Min = 32i32,
    None = -1i32,
    StDev = 35i32,
    Sum = 30i32,
    Var = 37i32,
}
#[cfg(feature = "System+Data+Aggregate")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Data::Aggregate => "System.Data"
    ."Aggregate"
);
