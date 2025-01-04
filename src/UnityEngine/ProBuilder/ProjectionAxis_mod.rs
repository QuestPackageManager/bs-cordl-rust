#[cfg(feature = "UnityEngine+ProBuilder+ProjectionAxis")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProjectionAxis {
    #[default]
    X = 0i32,
    XNegative = 3i32,
    Y = 1i32,
    YNegative = 4i32,
    Z = 2i32,
    ZNegative = 5i32,
}
#[cfg(feature = "UnityEngine+ProBuilder+ProjectionAxis")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::ProjectionAxis =>
    "UnityEngine.ProBuilder"."ProjectionAxis"
);
