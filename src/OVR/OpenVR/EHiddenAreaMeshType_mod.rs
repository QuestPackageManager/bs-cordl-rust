#[cfg(feature = "OVR+OpenVR+EHiddenAreaMeshType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EHiddenAreaMeshType {
    k_eHiddenAreaMesh_Inverse = 1i32,
    k_eHiddenAreaMesh_LineLoop = 2i32,
    k_eHiddenAreaMesh_Max = 3i32,
    k_eHiddenAreaMesh_Standard = 0i32,
}
#[cfg(feature = "OVR+OpenVR+EHiddenAreaMeshType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::EHiddenAreaMeshType => "OVR.OpenVR"
    ."EHiddenAreaMeshType"
);
