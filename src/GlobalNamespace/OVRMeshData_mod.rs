#[cfg(feature = "OVRMeshData")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRMeshData {
    pub mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    pub material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub baseAttributes: crate::GlobalNamespace::OVRMeshAttributes,
    pub morphTargets: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::GlobalNamespace::OVRMeshAttributes>,
    >,
}
#[cfg(feature = "OVRMeshData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRMeshData => ""."OVRMeshData"
);
#[cfg(feature = "OVRMeshData")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::GlobalNamespace::OVRMeshData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRMeshData")]
impl crate::GlobalNamespace::OVRMeshData {}
