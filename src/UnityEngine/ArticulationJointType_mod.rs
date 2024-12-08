#[cfg(feature = "UnityEngine+ArticulationJointType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArticulationJointType {
    FixedJoint = 0i32,
    PrismaticJoint = 1i32,
    RevoluteJoint = 2i32,
    SphericalJoint = 3i32,
}
#[cfg(feature = "UnityEngine+ArticulationJointType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ArticulationJointType =>
    "UnityEngine"."ArticulationJointType"
);
