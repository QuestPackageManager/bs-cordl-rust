#[cfg(feature = "BeatmapLevelsPromoModel")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapLevelsPromoModel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _promotedBeatmapLevelPacksSet: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::HashSet_1<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
    >,
    pub _updatedBeatmapLevelPacksSet: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::HashSet_1<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
    >,
    pub _promotedBeatmapLevelsSet: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::HashSet_1<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
    >,
    pub _updatedBeatmapLevelsSet: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::HashSet_1<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
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
        beatmapLevelPack: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelPack,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsBeatmapLevelPackPromoted", (beatmapLevelPack))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsBeatmapLevelPackUpdated(
        &mut self,
        beatmapLevelPack: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelPack,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsBeatmapLevelPackUpdated", (beatmapLevelPack))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsBeatmapLevelPromoted(
        &mut self,
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsBeatmapLevelPromoted", (beatmapLevel))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsBeatmapLevelUpdated(
        &mut self,
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsBeatmapLevelUpdated", (beatmapLevel))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        promoDataSO: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelsPromoDataSO,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (promoDataSO))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        promoDataSO: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelsPromoDataSO,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (promoDataSO))?;
        Ok(__cordl_ret.into())
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
