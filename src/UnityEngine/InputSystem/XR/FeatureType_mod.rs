#[cfg(feature = "UnityEngine+InputSystem+XR+FeatureType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FeatureType {
    #[default]
    Axis1D = 3i32,
    Axis2D = 4i32,
    Axis3D = 5i32,
    Binary = 1i32,
    Bone = 8i32,
    Custom = 0i32,
    DiscreteStates = 2i32,
    Eyes = 9i32,
    Hand = 7i32,
    Rotation = 6i32,
}
#[cfg(feature = "UnityEngine+InputSystem+XR+FeatureType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::XR::FeatureType =>
    "UnityEngine.InputSystem.XR"."FeatureType"
);
