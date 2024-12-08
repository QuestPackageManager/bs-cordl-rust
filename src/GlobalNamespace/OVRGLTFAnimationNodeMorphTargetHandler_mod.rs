#[cfg(feature = "OVRGLTFAnimationNodeMorphTargetHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRGLTFAnimationNodeMorphTargetHandler {
    __cordl_parent: crate::System::Object,
    pub _MeshData_k__BackingField: OVRMeshData,
    pub Weights: *mut quest_hook::libil2cpp::Il2CppArray<f32>,
    pub modified: bool,
}
#[cfg(feature = "OVRGLTFAnimationNodeMorphTargetHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for OVRGLTFAnimationNodeMorphTargetHandler => ""
    ."OVRGLTFAnimationNodeMorphTargetHandler"
);
#[cfg(feature = "OVRGLTFAnimationNodeMorphTargetHandler")]
impl std::ops::Deref for OVRGLTFAnimationNodeMorphTargetHandler {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRGLTFAnimationNodeMorphTargetHandler")]
impl std::ops::DerefMut for OVRGLTFAnimationNodeMorphTargetHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRGLTFAnimationNodeMorphTargetHandler")]
impl OVRGLTFAnimationNodeMorphTargetHandler {
    pub fn get_MeshData(&mut self) -> quest_hook::libil2cpp::Result<OVRMeshData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: OVRMeshData = __cordl_object.invoke("get_MeshData", ())?;
        Ok(__cordl_ret)
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
        Ok(__cordl_ret)
    }
    pub fn MarkModified(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkModified", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_MeshData(
        &mut self,
        value: OVRMeshData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_MeshData", (value))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        meshData: OVRMeshData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (meshData))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        meshData: OVRMeshData,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (meshData))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "OVRGLTFAnimationNodeMorphTargetHandler")]
impl quest_hook::libil2cpp::ObjectType for OVRGLTFAnimationNodeMorphTargetHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
