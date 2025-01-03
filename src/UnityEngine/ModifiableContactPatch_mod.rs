#[cfg(feature = "UnityEngine+ModifiableContactPatch")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ModifiableContactPatch {
    pub massProperties: crate::UnityEngine::ModifiableMassProperties,
    pub normal: crate::UnityEngine::Vector3,
    pub restitution: f32,
    pub dynamicFriction: f32,
    pub staticFriction: f32,
    pub startContactIndex: u8,
    pub contactCount: u8,
    pub materialFlags: u8,
    pub internalFlags: u8,
    pub materialIndex: u16,
    pub otherMaterialIndex: u16,
}
#[cfg(feature = "UnityEngine+ModifiableContactPatch")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ModifiableContactPatch =>
    "UnityEngine"."ModifiableContactPatch"
);
#[cfg(feature = "UnityEngine+ModifiableContactPatch")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ModifiableContactPatch {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ModifiableContactPatch")]
impl crate::UnityEngine::ModifiableContactPatch {
    #[cfg(feature = "UnityEngine+ModifiableContactPatch+Flags")]
    pub type Flags = crate::UnityEngine::ModifiableContactPatch_Flags;
}
#[cfg(feature = "UnityEngine+ModifiableContactPatch+Flags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModifiableContactPatch_Flags {
    HasFaceIndices = 1i32,
    HasMaxImpulse = 32i32,
    HasModifiedMassRatios = 8i32,
    HasTargetVelocity = 16i32,
    RegeneratePatches = 64i32,
}
#[cfg(feature = "UnityEngine+ModifiableContactPatch+Flags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ModifiableContactPatch_Flags =>
    "UnityEngine"."ModifiableContactPatch/Flags"
);
