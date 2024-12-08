#[cfg(feature = "BeatmapLevelPackSO")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapLevelPackSO {
    __cordl_parent: PersistentScriptableObject,
    pub _packID: *mut crate::System::String,
    pub _packName: *mut crate::System::String,
    pub _shortPackName: *mut crate::System::String,
    pub _coverImage: *mut crate::UnityEngine::Sprite,
    pub _smallCoverImage: *mut crate::UnityEngine::Sprite,
    pub _contentRating: PlayerSensitivityFlag,
    pub _beatmapLevelCollection: *mut BeatmapLevelCollectionSO,
}
#[cfg(feature = "BeatmapLevelPackSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BeatmapLevelPackSO => ""."BeatmapLevelPackSO"
);
#[cfg(feature = "BeatmapLevelPackSO")]
impl std::ops::Deref for BeatmapLevelPackSO {
    type Target = PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelPackSO")]
impl std::ops::DerefMut for BeatmapLevelPackSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelPackSO")]
impl BeatmapLevelPackSO {
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
    pub fn get_beatmapLevelCollection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut BeatmapLevelCollectionSO> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut BeatmapLevelCollectionSO = __cordl_object
            .invoke("get_beatmapLevelCollection", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_collectionName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_collectionName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_contentRating(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<PlayerSensitivityFlag> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: PlayerSensitivityFlag = __cordl_object
            .invoke("get_contentRating", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_coverImage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Sprite> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Sprite = __cordl_object
            .invoke("get_coverImage", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_packID(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_packID", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_packName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_packName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_shortPackName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_shortPackName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_smallCoverImage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Sprite> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Sprite = __cordl_object
            .invoke("get_smallCoverImage", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatmapLevelPackSO")]
impl quest_hook::libil2cpp::ObjectType for BeatmapLevelPackSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
