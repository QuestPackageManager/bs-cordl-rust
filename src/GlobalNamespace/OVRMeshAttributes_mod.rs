#[cfg(feature = "OVRMeshAttributes")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRMeshAttributes {
    pub vertices: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
    pub normals: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
    pub tangents: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
    pub texcoords: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
    pub colors: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
    pub boneWeights: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::BoneWeight,
    >,
}
#[cfg(feature = "OVRMeshAttributes")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for OVRMeshAttributes => ""."OVRMeshAttributes"
);
#[cfg(feature = "OVRMeshAttributes")]
unsafe impl quest_hook::libil2cpp::ThisArgument for OVRMeshAttributes {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRMeshAttributes")]
impl OVRMeshAttributes {}
