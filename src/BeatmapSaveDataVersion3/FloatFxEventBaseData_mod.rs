#[cfg(feature = "BeatmapSaveDataVersion3+FloatFxEventBaseData")]
#[repr(C)]
#[derive(Debug)]
pub struct FloatFxEventBaseData {
    __cordl_parent: crate::System::Object,
    pub b: f32,
    pub p: i32,
    pub v: f32,
    pub i: crate::BeatmapSaveDataCommon::EaseType,
}
#[cfg(feature = "BeatmapSaveDataVersion3+FloatFxEventBaseData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatmapSaveDataVersion3::FloatFxEventBaseData =>
    "BeatmapSaveDataVersion3"."FloatFxEventBaseData"
);
#[cfg(feature = "BeatmapSaveDataVersion3+FloatFxEventBaseData")]
impl std::ops::Deref for crate::BeatmapSaveDataVersion3::FloatFxEventBaseData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+FloatFxEventBaseData")]
impl std::ops::DerefMut for crate::BeatmapSaveDataVersion3::FloatFxEventBaseData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+FloatFxEventBaseData")]
impl crate::BeatmapSaveDataVersion3::FloatFxEventBaseData {
    pub fn New(
        beat: f32,
        usePreviousEventValue: bool,
        value: f32,
        easeType: crate::BeatmapSaveDataCommon::EaseType,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (beat, usePreviousEventValue, value, easeType))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        beat: f32,
        usePreviousEventValue: bool,
        value: f32,
        easeType: crate::BeatmapSaveDataCommon::EaseType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (beat, usePreviousEventValue, value, easeType))?;
        Ok(__cordl_ret)
    }
    pub fn get_beat(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_beat", ())?;
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
    pub fn get_usePreviousEventValue(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_usePreviousEventValue", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_value(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_value", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+FloatFxEventBaseData")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapSaveDataVersion3::FloatFxEventBaseData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
