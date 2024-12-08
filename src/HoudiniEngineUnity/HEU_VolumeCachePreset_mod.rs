#[cfg(feature = "HoudiniEngineUnity+HEU_VolumeCachePreset")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_VolumeCachePreset {
    __cordl_parent: crate::System::Object,
    pub _objName: *mut crate::System::String,
    pub _geoName: *mut crate::System::String,
    pub _uiExpanded: bool,
    pub _volumeLayersPresets: *mut crate::System::Collections::Generic::List_1<
        *mut crate::HoudiniEngineUnity::HEU_VolumeLayerPreset,
    >,
    pub _terrainDataPath: *mut crate::System::String,
    pub _tile: i32,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_VolumeCachePreset")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_VolumeCachePreset =>
    "HoudiniEngineUnity"."HEU_VolumeCachePreset"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_VolumeCachePreset")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_VolumeCachePreset {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_VolumeCachePreset")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_VolumeCachePreset {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_VolumeCachePreset")]
impl crate::HoudiniEngineUnity::HEU_VolumeCachePreset {
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
#[cfg(feature = "HoudiniEngineUnity+HEU_VolumeCachePreset")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_VolumeCachePreset {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}