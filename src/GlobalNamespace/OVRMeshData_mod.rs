#[cfg(feature = "OVRMeshData")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRMeshData {
    pub mesh: *mut crate::UnityEngine::Mesh,
    pub material: *mut crate::UnityEngine::Material,
    pub baseAttributes: OVRMeshAttributes,
    pub morphTargets: *mut quest_hook::libil2cpp::Il2CppArray<OVRMeshAttributes>,
}
#[cfg(feature = "OVRMeshData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for OVRMeshData => ""."OVRMeshData"
);
#[cfg(feature = "OVRMeshData")]
unsafe impl quest_hook::libil2cpp::ThisArgument for OVRMeshData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRMeshData")]
impl OVRMeshData {}
