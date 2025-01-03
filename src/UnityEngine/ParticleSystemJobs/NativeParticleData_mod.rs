#[cfg(feature = "UnityEngine+ParticleSystemJobs+NativeParticleData")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct NativeParticleData {
    pub count: i32,
    pub positions: crate::UnityEngine::ParticleSystemJobs::NativeParticleData_Array3,
    pub velocities: crate::UnityEngine::ParticleSystemJobs::NativeParticleData_Array3,
    pub axisOfRotations: crate::UnityEngine::ParticleSystemJobs::NativeParticleData_Array3,
    pub rotations: crate::UnityEngine::ParticleSystemJobs::NativeParticleData_Array3,
    pub rotationalSpeeds: crate::UnityEngine::ParticleSystemJobs::NativeParticleData_Array3,
    pub sizes: crate::UnityEngine::ParticleSystemJobs::NativeParticleData_Array3,
    pub startColors: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub aliveTimePercent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub inverseStartLifetimes: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppObject,
    >,
    pub randomSeeds: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub customData1: crate::UnityEngine::ParticleSystemJobs::NativeParticleData_Array4,
    pub customData2: crate::UnityEngine::ParticleSystemJobs::NativeParticleData_Array4,
    pub meshIndices: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+ParticleSystemJobs+NativeParticleData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ParticleSystemJobs::NativeParticleData =>
    "UnityEngine.ParticleSystemJobs"."NativeParticleData"
);
#[cfg(feature = "UnityEngine+ParticleSystemJobs+NativeParticleData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ParticleSystemJobs::NativeParticleData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ParticleSystemJobs+NativeParticleData")]
impl crate::UnityEngine::ParticleSystemJobs::NativeParticleData {
    #[cfg(feature = "UnityEngine+ParticleSystemJobs+NativeParticleData+Array3")]
    pub type Array3 = crate::UnityEngine::ParticleSystemJobs::NativeParticleData_Array3;
    #[cfg(feature = "UnityEngine+ParticleSystemJobs+NativeParticleData+Array4")]
    pub type Array4 = crate::UnityEngine::ParticleSystemJobs::NativeParticleData_Array4;
}
#[cfg(feature = "UnityEngine+ParticleSystemJobs+NativeParticleData+Array3")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct NativeParticleData_Array3 {
    pub x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+ParticleSystemJobs+NativeParticleData+Array3")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ParticleSystemJobs::NativeParticleData_Array3 =>
    "UnityEngine.ParticleSystemJobs"."NativeParticleData/Array3"
);
#[cfg(feature = "UnityEngine+ParticleSystemJobs+NativeParticleData+Array3")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ParticleSystemJobs::NativeParticleData_Array3 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ParticleSystemJobs+NativeParticleData+Array3")]
impl crate::UnityEngine::ParticleSystemJobs::NativeParticleData_Array3 {}
#[cfg(feature = "UnityEngine+ParticleSystemJobs+NativeParticleData+Array4")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct NativeParticleData_Array4 {
    pub x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub w: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+ParticleSystemJobs+NativeParticleData+Array4")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ParticleSystemJobs::NativeParticleData_Array4 =>
    "UnityEngine.ParticleSystemJobs"."NativeParticleData/Array4"
);
#[cfg(feature = "UnityEngine+ParticleSystemJobs+NativeParticleData+Array4")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ParticleSystemJobs::NativeParticleData_Array4 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ParticleSystemJobs+NativeParticleData+Array4")]
impl crate::UnityEngine::ParticleSystemJobs::NativeParticleData_Array4 {}
