#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+Orientation")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Orientation {
    #[default]
    CCW = 1i32,
    CW = 0i32,
    Collinear = 2i32,
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+Orientation")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::Poly2Tri::Orientation
    => "UnityEngine.ProBuilder.Poly2Tri"."Orientation"
);
