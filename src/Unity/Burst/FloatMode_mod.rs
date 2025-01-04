#[cfg(feature = "Unity+Burst+FloatMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FloatMode {
    #[default]
    Default = 0i32,
    Deterministic = 2i32,
    Fast = 3i32,
    Strict = 1i32,
}
#[cfg(feature = "Unity+Burst+FloatMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::FloatMode => "Unity.Burst"
    ."FloatMode"
);
