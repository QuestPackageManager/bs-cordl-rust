#[cfg(feature = "BeatmapSaveDataVersion3+LightTranslationBaseData")]
#[repr(C)]
#[derive(Debug)]
pub struct LightTranslationBaseData {
    __cordl_parent: crate::System::Object,
    pub b: f32,
    pub p: i32,
    pub e: crate::BeatmapSaveDataCommon::EaseType,
    pub t: f32,
}
#[cfg(feature = "BeatmapSaveDataVersion3+LightTranslationBaseData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatmapSaveDataVersion3::LightTranslationBaseData => "BeatmapSaveDataVersion3"
    ."LightTranslationBaseData"
);
#[cfg(feature = "BeatmapSaveDataVersion3+LightTranslationBaseData")]
impl std::ops::Deref for crate::BeatmapSaveDataVersion3::LightTranslationBaseData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+LightTranslationBaseData")]
impl std::ops::DerefMut for crate::BeatmapSaveDataVersion3::LightTranslationBaseData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+LightTranslationBaseData")]
impl crate::BeatmapSaveDataVersion3::LightTranslationBaseData {
    pub fn get_usePreviousEventTranslationValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_usePreviousEventTranslationValue", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_easeType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::BeatmapSaveDataCommon::EaseType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BeatmapSaveDataCommon::EaseType = __cordl_object
            .invoke("get_easeType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_translation(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_translation", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_beat(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_beat", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        beat: f32,
        usePreviousEventTranslationValue: bool,
        easeType: crate::BeatmapSaveDataCommon::EaseType,
        translation: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (beat, usePreviousEventTranslationValue, easeType, translation),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New(
        beat: f32,
        usePreviousEventTranslationValue: bool,
        easeType: crate::BeatmapSaveDataCommon::EaseType,
        translation: f32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (beat, usePreviousEventTranslationValue, easeType, translation),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+LightTranslationBaseData")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapSaveDataVersion3::LightTranslationBaseData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
