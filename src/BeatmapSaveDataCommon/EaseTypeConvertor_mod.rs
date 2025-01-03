#[cfg(feature = "BeatmapSaveDataCommon+EaseTypeConvertor")]
#[repr(C)]
#[derive(Debug)]
pub struct EaseTypeConvertor {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BeatmapSaveDataCommon+EaseTypeConvertor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatmapSaveDataCommon::EaseTypeConvertor =>
    "BeatmapSaveDataCommon"."EaseTypeConvertor"
);
#[cfg(feature = "BeatmapSaveDataCommon+EaseTypeConvertor")]
impl std::ops::Deref for crate::BeatmapSaveDataCommon::EaseTypeConvertor {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataCommon+EaseTypeConvertor")]
impl std::ops::DerefMut for crate::BeatmapSaveDataCommon::EaseTypeConvertor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataCommon+EaseTypeConvertor")]
impl crate::BeatmapSaveDataCommon::EaseTypeConvertor {
    pub fn Convert(
        easeType: crate::GlobalNamespace::EaseType,
    ) -> quest_hook::libil2cpp::Result<crate::BeatmapSaveDataCommon::EaseType> {
        let __cordl_ret: crate::BeatmapSaveDataCommon::EaseType = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Convert", (easeType))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapSaveDataCommon+EaseTypeConvertor")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapSaveDataCommon::EaseTypeConvertor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
