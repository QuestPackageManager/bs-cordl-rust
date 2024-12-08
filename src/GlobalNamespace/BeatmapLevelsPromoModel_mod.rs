#[cfg(feature = "BeatmapLevelsPromoModel")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapLevelsPromoModel {
    __cordl_parent: crate::System::Object,
    pub _promotedBeatmapLevelPacksSet: *mut crate::System::Collections::Generic::HashSet_1<
        *mut crate::System::String,
    >,
    pub _updatedBeatmapLevelPacksSet: *mut crate::System::Collections::Generic::HashSet_1<
        *mut crate::System::String,
    >,
    pub _promotedBeatmapLevelsSet: *mut crate::System::Collections::Generic::HashSet_1<
        *mut crate::System::String,
    >,
    pub _updatedBeatmapLevelsSet: *mut crate::System::Collections::Generic::HashSet_1<
        *mut crate::System::String,
    >,
}
#[cfg(feature = "BeatmapLevelsPromoModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BeatmapLevelsPromoModel => ""."BeatmapLevelsPromoModel"
);
#[cfg(feature = "BeatmapLevelsPromoModel")]
impl std::ops::Deref for BeatmapLevelsPromoModel {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelsPromoModel")]
impl std::ops::DerefMut for BeatmapLevelsPromoModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelsPromoModel")]
impl BeatmapLevelsPromoModel {
    pub fn IsBeatmapLevelPackPromoted(
        &mut self,
        beatmapLevelPack: *mut BeatmapLevelPack,
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
        beatmapLevelPack: *mut BeatmapLevelPack,
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
        beatmapLevel: *mut BeatmapLevel,
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
        beatmapLevel: *mut BeatmapLevel,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsBeatmapLevelUpdated", (beatmapLevel))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        promoDataSO: *mut BeatmapLevelsPromoDataSO,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (promoDataSO))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        promoDataSO: *mut BeatmapLevelsPromoDataSO,
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
impl quest_hook::libil2cpp::ObjectType for BeatmapLevelsPromoModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
