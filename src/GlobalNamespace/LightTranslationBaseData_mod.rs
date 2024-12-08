#[cfg(feature = "LightTranslationBaseData")]
#[repr(C)]
#[derive(Debug)]
pub struct LightTranslationBaseData {
    __cordl_parent: crate::System::Object,
    pub beat: f32,
    pub usePreviousEventTranslationValue: bool,
    pub easeType: EaseType,
    pub translation: f32,
}
#[cfg(feature = "LightTranslationBaseData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for LightTranslationBaseData => ""
    ."LightTranslationBaseData"
);
#[cfg(feature = "LightTranslationBaseData")]
impl std::ops::Deref for LightTranslationBaseData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LightTranslationBaseData")]
impl std::ops::DerefMut for LightTranslationBaseData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LightTranslationBaseData")]
impl LightTranslationBaseData {
    pub fn _ctor(
        &mut self,
        beat: f32,
        usePreviousEventTranslationValue: bool,
        easeType: EaseType,
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
        easeType: EaseType,
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
#[cfg(feature = "LightTranslationBaseData")]
impl quest_hook::libil2cpp::ObjectType for LightTranslationBaseData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
