#[cfg(feature = "UnityEngine+XR+MeshGenerationResult")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct MeshGenerationResult {
    pub _MeshId_k__BackingField: crate::UnityEngine::XR::MeshId,
    pub _Mesh_k__BackingField: *mut crate::UnityEngine::Mesh,
    pub _MeshCollider_k__BackingField: *mut crate::UnityEngine::MeshCollider,
    pub _Status_k__BackingField: crate::UnityEngine::XR::MeshGenerationStatus,
    pub _Attributes_k__BackingField: crate::UnityEngine::XR::MeshVertexAttributes,
    pub _Timestamp_k__BackingField: u64,
    pub _Position_k__BackingField: crate::UnityEngine::Vector3,
    pub _Rotation_k__BackingField: crate::UnityEngine::Quaternion,
    pub _Scale_k__BackingField: crate::UnityEngine::Vector3,
}
#[cfg(feature = "UnityEngine+XR+MeshGenerationResult")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::XR::MeshGenerationResult =>
    "UnityEngine.XR"."MeshGenerationResult"
);
#[cfg(feature = "UnityEngine+XR+MeshGenerationResult")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::XR::MeshGenerationResult {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+XR+MeshGenerationResult")]
impl crate::UnityEngine::XR::MeshGenerationResult {
    pub fn get_MeshId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::XR::MeshId> {
        let __cordl_ret: crate::UnityEngine::XR::MeshId = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_MeshId",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Position(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Position",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Status(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::XR::MeshGenerationStatus> {
        let __cordl_ret: crate::UnityEngine::XR::MeshGenerationStatus = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Status",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Attributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::XR::MeshVertexAttributes> {
        let __cordl_ret: crate::UnityEngine::XR::MeshVertexAttributes = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Attributes",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Mesh(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Mesh> {
        let __cordl_ret: *mut crate::UnityEngine::Mesh = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Mesh",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals_Object0(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals_MeshGenerationResult1(
        &mut self,
        other: crate::UnityEngine::XR::MeshGenerationResult,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Rotation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_ret: crate::UnityEngine::Quaternion = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Rotation",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_MeshCollider(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::MeshCollider> {
        let __cordl_ret: *mut crate::UnityEngine::MeshCollider = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_MeshCollider",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Scale(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Scale",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
