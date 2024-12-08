#[cfg(feature = "OVR+OpenVR+EVRSkeletalMotionRange")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EVRSkeletalMotionRange {
    WithController = 0i32,
    WithoutController = 1i32,
}
#[cfg(feature = "OVR+OpenVR+EVRSkeletalMotionRange")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::EVRSkeletalMotionRange =>
    "OVR.OpenVR"."EVRSkeletalMotionRange"
);
