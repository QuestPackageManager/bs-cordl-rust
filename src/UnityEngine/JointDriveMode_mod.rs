#[cfg(feature = "UnityEngine+JointDriveMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JointDriveMode {
    None = 0i32,
    Position = 1i32,
    PositionAndVelocity = 3i32,
    Velocity = 2i32,
}
#[cfg(feature = "UnityEngine+JointDriveMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::JointDriveMode => "UnityEngine"
    ."JointDriveMode"
);
