#[cfg(feature = "Unity+Burst+FloatPrecision")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloatPrecision {
    High = 1i32,
    Low = 3i32,
    Medium = 2i32,
    Standard = 0i32,
}
#[cfg(feature = "Unity+Burst+FloatPrecision")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::FloatPrecision => "Unity.Burst"
    ."FloatPrecision"
);
