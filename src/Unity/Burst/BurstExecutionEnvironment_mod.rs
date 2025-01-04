#[cfg(feature = "Unity+Burst+BurstExecutionEnvironment")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BurstExecutionEnvironment {
    #[default]
    Default = 0i32,
    Deterministic = 1i32,
}
#[cfg(feature = "Unity+Burst+BurstExecutionEnvironment")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::BurstExecutionEnvironment =>
    "Unity.Burst"."BurstExecutionEnvironment"
);
