#[cfg(feature = "Mono+Math+Prime+ConfidenceFactor")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfidenceFactor {
    ExtraHigh = 4i32,
    ExtraLow = 0i32,
    High = 3i32,
    Low = 1i32,
    Medium = 2i32,
    Provable = 5i32,
}
#[cfg(feature = "Mono+Math+Prime+ConfidenceFactor")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Mono::Math::Prime::ConfidenceFactor =>
    "Mono.Math.Prime"."ConfidenceFactor"
);
