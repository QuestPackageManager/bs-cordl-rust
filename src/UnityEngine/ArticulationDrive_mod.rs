#[cfg(feature = "UnityEngine+ArticulationDrive")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ArticulationDrive {
    pub lowerLimit: f32,
    pub upperLimit: f32,
    pub stiffness: f32,
    pub damping: f32,
    pub forceLimit: f32,
    pub target: f32,
    pub targetVelocity: f32,
    pub driveType: crate::UnityEngine::ArticulationDriveType,
}
#[cfg(feature = "UnityEngine+ArticulationDrive")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ArticulationDrive => "UnityEngine"
    ."ArticulationDrive"
);
#[cfg(feature = "UnityEngine+ArticulationDrive")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ArticulationDrive {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ArticulationDrive")]
impl crate::UnityEngine::ArticulationDrive {}
