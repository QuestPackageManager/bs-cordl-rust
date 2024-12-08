#[cfg(feature = "SongPackMasksModelItem")]
#[repr(C)]
#[derive(Debug)]
pub struct SongPackMasksModelItem {
    __cordl_parent: crate::System::Object,
    pub _type: SongPackDataType,
    pub _serializedName: *mut crate::System::String,
    pub _beatmapLevelPackId: *mut crate::System::String,
    pub _includeTags: crate::GlobalNamespace::PackDefinitionSO_Tags,
    pub _excludeTags: crate::GlobalNamespace::PackDefinitionSO_Tags,
}
#[cfg(feature = "SongPackMasksModelItem")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for SongPackMasksModelItem => ""."SongPackMasksModelItem"
);
#[cfg(feature = "SongPackMasksModelItem")]
impl std::ops::Deref for SongPackMasksModelItem {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SongPackMasksModelItem")]
impl std::ops::DerefMut for SongPackMasksModelItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SongPackMasksModelItem")]
impl SongPackMasksModelItem {
    #[cfg(feature = "SongPackMasksModelItem+__c")]
    pub type __c = crate::GlobalNamespace::SongPackMasksModelItem___c;
    pub fn GetSongPackMask(
        &mut self,
        beatmapLevelsModel: *mut BeatmapLevelsModel,
    ) -> quest_hook::libil2cpp::Result<SongPackMask> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: SongPackMask = __cordl_object
            .invoke("GetSongPackMask", (beatmapLevelsModel))?;
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
    pub fn get_containsMultiplePacks(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_containsMultiplePacks", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_serializedName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_serializedName", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "SongPackMasksModelItem")]
impl quest_hook::libil2cpp::ObjectType for SongPackMasksModelItem {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
