#[cfg(feature = "TMPro+Compute_DistanceTransform_EventTypes")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Compute_DistanceTransform_EventTypes {
    Completed = 1i32,
    Processing = 0i32,
}
#[cfg(feature = "TMPro+Compute_DistanceTransform_EventTypes")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::TMPro::Compute_DistanceTransform_EventTypes =>
    "TMPro"."Compute_DistanceTransform_EventTypes"
);
