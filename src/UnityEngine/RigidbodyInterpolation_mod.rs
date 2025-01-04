#[cfg(feature = "UnityEngine+RigidbodyInterpolation")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RigidbodyInterpolation {
    #[default]
    Extrapolate = 2i32,
    Interpolate = 1i32,
    None = 0i32,
}
#[cfg(feature = "UnityEngine+RigidbodyInterpolation")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::RigidbodyInterpolation =>
    "UnityEngine"."RigidbodyInterpolation"
);
