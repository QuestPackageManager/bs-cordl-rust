#[cfg(feature = "UnityEngine+RotationDriveMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RotationDriveMode {
    Slerp = 1i32,
    XYAndZ = 0i32,
}
#[cfg(feature = "UnityEngine+RotationDriveMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::RotationDriveMode => "UnityEngine"
    ."RotationDriveMode"
);
