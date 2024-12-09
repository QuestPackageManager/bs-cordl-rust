#[cfg(feature = "BeatmapLevelsPromoModel")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapLevelsPromoModel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _promotedBeatmapLevelPacksSet: *mut crate::System::Collections::Generic::HashSet_1<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
    pub _updatedBeatmapLevelPacksSet: *mut crate::System::Collections::Generic::HashSet_1<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
    pub _promotedBeatmapLevelsSet: *mut crate::System::Collections::Generic::HashSet_1<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
    pub _updatedBeatmapLevelsSet: *mut crate::System::Collections::Generic::HashSet_1<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
}
#[cfg(feature = "BeatmapLevelsPromoModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatmapLevelsPromoModel => ""
    ."BeatmapLevelsPromoModel"
);
#[cfg(feature = "BeatmapLevelsPromoModel")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapLevelsPromoModel {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelsPromoModel")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapLevelsPromoModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelsPromoModel")]
impl crate::GlobalNamespace::BeatmapLevelsPromoModel {
    pub fn IsBeatmapLevelPackPromoted(
        &mut self,
        beatmapLevelPack: *mut crate::GlobalNamespace::BeatmapLevelPack,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsBeatmapLevelPackPromoted", (beatmapLevelPack))?;
        Ok(__cordl_ret)
    }
    pub fn IsBeatmapLevelPackUpdated(
        &mut self,
        beatmapLevelPack: *mut crate::GlobalNamespace::BeatmapLevelPack,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsBeatmapLevelPackUpdated", (beatmapLevelPack))?;
        Ok(__cordl_ret)
    }
    pub fn IsBeatmapLevelPromoted(
        &mut self,
        beatmapLevel: *mut crate::GlobalNamespace::BeatmapLevel,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsBeatmapLevelPromoted", (beatmapLevel))?;
        Ok(__cordl_ret)
    }
    pub fn IsBeatmapLevelUpdated(
        &mut self,
        beatmapLevel: *mut crate::GlobalNamespace::BeatmapLevel,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsBeatmapLevelUpdated", (beatmapLevel))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        promoDataSO: *mut crate::GlobalNamespace::BeatmapLevelsPromoDataSO,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (promoDataSO))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        promoDataSO: *mut crate::GlobalNamespace::BeatmapLevelsPromoDataSO,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (promoDataSO))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatmapLevelsPromoModel")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapLevelsPromoModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
