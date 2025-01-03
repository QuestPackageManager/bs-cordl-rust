#[cfg(feature = "SongPackMasksModelItem")]
#[repr(C)]
#[derive(Debug)]
pub struct SongPackMasksModelItem {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _type: crate::GlobalNamespace::SongPackDataType,
    pub _serializedName: *mut quest_hook::libil2cpp::Il2CppString,
    pub _beatmapLevelPackId: *mut quest_hook::libil2cpp::Il2CppString,
    pub _includeTags: crate::GlobalNamespace::PackDefinitionSO_Tags,
    pub _excludeTags: crate::GlobalNamespace::PackDefinitionSO_Tags,
}
#[cfg(feature = "SongPackMasksModelItem")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SongPackMasksModelItem => ""
    ."SongPackMasksModelItem"
);
#[cfg(feature = "SongPackMasksModelItem")]
impl std::ops::Deref for crate::GlobalNamespace::SongPackMasksModelItem {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SongPackMasksModelItem")]
impl std::ops::DerefMut for crate::GlobalNamespace::SongPackMasksModelItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SongPackMasksModelItem")]
impl crate::GlobalNamespace::SongPackMasksModelItem {
    pub fn GetSongPackMask(
        &mut self,
        beatmapLevelsModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelsModel,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::SongPackMask> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::SongPackMask = __cordl_object
            .invoke("GetSongPackMask", (beatmapLevelsModel))?;
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
    pub fn get_containsMultiplePacks(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_containsMultiplePacks", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_serializedName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_serializedName", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SongPackMasksModelItem")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SongPackMasksModelItem {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
