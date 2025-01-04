#[cfg(feature = "Unity+Burst+OptimizeFor")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OptimizeFor {
    #[default]
    Balanced = 4i32,
    Default = 0i32,
    FastCompilation = 3i32,
    Performance = 1i32,
    Size = 2i32,
}
#[cfg(feature = "Unity+Burst+OptimizeFor")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::OptimizeFor => "Unity.Burst"
    ."OptimizeFor"
);
