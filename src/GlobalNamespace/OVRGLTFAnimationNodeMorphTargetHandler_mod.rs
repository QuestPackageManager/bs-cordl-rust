#[cfg(feature = "OVRGLTFAnimationNodeMorphTargetHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRGLTFAnimationNodeMorphTargetHandler {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _MeshData_k__BackingField: crate::GlobalNamespace::OVRMeshData,
    pub Weights: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    pub modified: bool,
}
#[cfg(feature = "OVRGLTFAnimationNodeMorphTargetHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRGLTFAnimationNodeMorphTargetHandler => ""
    ."OVRGLTFAnimationNodeMorphTargetHandler"
);
#[cfg(feature = "OVRGLTFAnimationNodeMorphTargetHandler")]
impl std::ops::Deref for crate::GlobalNamespace::OVRGLTFAnimationNodeMorphTargetHandler {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRGLTFAnimationNodeMorphTargetHandler")]
impl std::ops::DerefMut
for crate::GlobalNamespace::OVRGLTFAnimationNodeMorphTargetHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRGLTFAnimationNodeMorphTargetHandler")]
impl crate::GlobalNamespace::OVRGLTFAnimationNodeMorphTargetHandler {
    pub fn MarkModified(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkModified", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        meshData: crate::GlobalNamespace::OVRMeshData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (meshData))?;
        Ok(__cordl_object.into())
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        meshData: crate::GlobalNamespace::OVRMeshData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (meshData))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MeshData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRMeshData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRMeshData = __cordl_object
            .invoke("get_MeshData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_MeshData(
        &mut self,
        value: crate::GlobalNamespace::OVRMeshData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_MeshData", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRGLTFAnimationNodeMorphTargetHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRGLTFAnimationNodeMorphTargetHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
