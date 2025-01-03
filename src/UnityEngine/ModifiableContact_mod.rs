#[cfg(feature = "UnityEngine+ModifiableContact")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ModifiableContact {
    pub contact: crate::UnityEngine::Vector3,
    pub separation: f32,
    pub targetVelocity: crate::UnityEngine::Vector3,
    pub maxImpulse: f32,
    pub normal: crate::UnityEngine::Vector3,
    pub restitution: f32,
    pub materialFlags: u32,
    pub materialIndex: u16,
    pub otherMaterialIndex: u16,
    pub staticFriction: f32,
    pub dynamicFriction: f32,
}
#[cfg(feature = "UnityEngine+ModifiableContact")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ModifiableContact => "UnityEngine"
    ."ModifiableContact"
);
#[cfg(feature = "UnityEngine+ModifiableContact")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ModifiableContact {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ModifiableContact")]
impl crate::UnityEngine::ModifiableContact {}
