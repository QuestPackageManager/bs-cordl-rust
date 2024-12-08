#[cfg(feature = "OVR+OpenVR+EVRSkeletalTransformSpace")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EVRSkeletalTransformSpace {
    Additive = 2i32,
    Model = 0i32,
    Parent = 1i32,
}
#[cfg(feature = "OVR+OpenVR+EVRSkeletalTransformSpace")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::EVRSkeletalTransformSpace =>
    "OVR.OpenVR"."EVRSkeletalTransformSpace"
);
