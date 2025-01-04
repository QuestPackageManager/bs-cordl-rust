#[cfg(feature = "UnityEngine+ProBuilder+HandleAxis")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HandleAxis {
    #[default]
    Free = 8i32,
    X = 1i32,
    Y = 2i32,
    Z = 4i32,
}
#[cfg(feature = "UnityEngine+ProBuilder+HandleAxis")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::HandleAxis =>
    "UnityEngine.ProBuilder"."HandleAxis"
);
