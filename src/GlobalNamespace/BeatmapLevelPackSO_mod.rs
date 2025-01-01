#[cfg(feature = "BeatmapLevelPackSO")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapLevelPackSO {
    __cordl_parent: crate::GlobalNamespace::PersistentScriptableObject,
    pub _packID: *mut quest_hook::libil2cpp::Il2CppString,
    pub _packName: *mut quest_hook::libil2cpp::Il2CppString,
    pub _shortPackName: *mut quest_hook::libil2cpp::Il2CppString,
    pub _coverImage: *mut crate::UnityEngine::Sprite,
    pub _smallCoverImage: *mut crate::UnityEngine::Sprite,
    pub _packBuyOption: crate::GlobalNamespace::PackBuyOption,
    pub _contentRating: crate::GlobalNamespace::PlayerSensitivityFlag,
    pub _beatmapLevelCollection: *mut crate::GlobalNamespace::BeatmapLevelCollectionSO,
}
#[cfg(feature = "BeatmapLevelPackSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatmapLevelPackSO => ""
    ."BeatmapLevelPackSO"
);
#[cfg(feature = "BeatmapLevelPackSO")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapLevelPackSO {
    type Target = crate::GlobalNamespace::PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelPackSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapLevelPackSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelPackSO")]
impl crate::GlobalNamespace::BeatmapLevelPackSO {
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
    pub fn get_beatmapLevelCollection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelCollectionSO>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelCollectionSO,
        > = __cordl_object.invoke("get_beatmapLevelCollection", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_collectionName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_collectionName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_contentRating(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::PlayerSensitivityFlag> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::PlayerSensitivityFlag = __cordl_object
            .invoke("get_contentRating", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_coverImage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite> = __cordl_object
            .invoke("get_coverImage", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_packBuyOption(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::PackBuyOption> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::PackBuyOption = __cordl_object
            .invoke("get_packBuyOption", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_packID(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_packID", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_packName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_packName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_shortPackName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_shortPackName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_smallCoverImage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite> = __cordl_object
            .invoke("get_smallCoverImage", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapLevelPackSO")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::BeatmapLevelPackSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
