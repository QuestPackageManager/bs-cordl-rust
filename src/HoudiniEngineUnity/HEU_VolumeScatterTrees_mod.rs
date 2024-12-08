#[cfg(feature = "HoudiniEngineUnity+HEU_VolumeScatterTrees")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_VolumeScatterTrees {
    __cordl_parent: crate::System::Object,
    pub _treePrototypInfos: *mut crate::System::Collections::Generic::List_1<
        *mut crate::HoudiniEngineUnity::HEU_TreePrototypeInfo,
    >,
    pub _colors: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color32>,
    pub _heightScales: *mut quest_hook::libil2cpp::Il2CppArray<f32>,
    pub _lightmapColors: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::Color32,
    >,
    pub _positions: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
    pub _prototypeIndices: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub _rotations: *mut quest_hook::libil2cpp::Il2CppArray<f32>,
    pub _widthScales: *mut quest_hook::libil2cpp::Il2CppArray<f32>,
    pub _terrainTiles: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_VolumeScatterTrees")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_VolumeScatterTrees =>
    "HoudiniEngineUnity"."HEU_VolumeScatterTrees"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_VolumeScatterTrees")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_VolumeScatterTrees {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_VolumeScatterTrees")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_VolumeScatterTrees {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_VolumeScatterTrees")]
impl crate::HoudiniEngineUnity::HEU_VolumeScatterTrees {
    pub fn IsEquivalentTo(
        &mut self,
        other: *mut crate::HoudiniEngineUnity::HEU_VolumeScatterTrees,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsEquivalentTo", (other))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_VolumeScatterTrees")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_VolumeScatterTrees {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}