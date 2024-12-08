#[cfg(feature = "ColorMixAndWeightingApproach")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorMixAndWeightingApproach {
    FractionAndSum = 1i32,
    Maximum = 0i32,
}
#[cfg(feature = "ColorMixAndWeightingApproach")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for ColorMixAndWeightingApproach => ""
    ."ColorMixAndWeightingApproach"
);
