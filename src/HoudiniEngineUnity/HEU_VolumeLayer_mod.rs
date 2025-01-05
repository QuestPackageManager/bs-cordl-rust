#[cfg(feature = "HoudiniEngineUnity+HEU_VolumeLayer")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_VolumeLayer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _layerName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _part: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_PartData>,
    pub _strength: f32,
    pub _uiExpanded: bool,
    pub _tile: i32,
    pub _xLength: i32,
    pub _yLength: i32,
    pub _hasLayerAttributes: bool,
    pub _terrainLayer: quest_hook::libil2cpp::Gc<crate::UnityEngine::TerrainLayer>,
    pub _layerType: crate::HoudiniEngineUnity::HFLayerType,
    pub _detailPrototype: quest_hook::libil2cpp::Gc<
        crate::HoudiniEngineUnity::HEU_DetailPrototype,
    >,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_VolumeLayer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_VolumeLayer =>
    "HoudiniEngineUnity"."HEU_VolumeLayer"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_VolumeLayer")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_VolumeLayer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_VolumeLayer")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_VolumeLayer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_VolumeLayer")]
impl crate::HoudiniEngineUnity::HEU_VolumeLayer {
    pub fn IsEquivalentTo(
        &mut self,
        other: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_VolumeLayer>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsEquivalentTo", (other))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_VolumeLayer")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::HEU_VolumeLayer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_VolumeLayer")]
impl AsRef<
    crate::HoudiniEngineUnity::IEquivable_1<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_VolumeLayer>,
    >,
> for crate::HoudiniEngineUnity::HEU_VolumeLayer {
    fn as_ref(
        &self,
    ) -> &crate::HoudiniEngineUnity::IEquivable_1<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_VolumeLayer>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_VolumeLayer")]
impl AsMut<
    crate::HoudiniEngineUnity::IEquivable_1<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_VolumeLayer>,
    >,
> for crate::HoudiniEngineUnity::HEU_VolumeLayer {
    fn as_mut(
        &mut self,
    ) -> &mut crate::HoudiniEngineUnity::IEquivable_1<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_VolumeLayer>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
