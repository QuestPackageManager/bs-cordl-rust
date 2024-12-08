#[cfg(feature = "UnityEngine+XR+InputFeatureType")]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputFeatureType {
    Axis1D = 100992003u32,
    Axis2D = 117835012u32,
    Axis3D = 134678021u32,
    Binary = 67305985u32,
    Bone = 16713992u32,
    Custom = 50462976u32,
    DiscreteStates = 84148994u32,
    Eyes = 16842505u32,
    Hand = 4278781959u32,
    Rotation = 151521030u32,
    kUnityXRInputFeatureTypeInvalid = 33620223u32,
}
#[cfg(feature = "UnityEngine+XR+InputFeatureType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::XR::InputFeatureType =>
    "UnityEngine.XR"."InputFeatureType"
);
