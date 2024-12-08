#[cfg(feature = "BeatmapSaveDataVersion3+ColorBoostEventData")]
#[repr(C)]
#[derive(Debug)]
pub struct ColorBoostEventData {
    __cordl_parent: crate::BeatmapSaveDataVersion3::BeatmapSaveDataItem,
    pub o: bool,
}
#[cfg(feature = "BeatmapSaveDataVersion3+ColorBoostEventData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatmapSaveDataVersion3::ColorBoostEventData =>
    "BeatmapSaveDataVersion3"."ColorBoostEventData"
);
#[cfg(feature = "BeatmapSaveDataVersion3+ColorBoostEventData")]
impl std::ops::Deref for crate::BeatmapSaveDataVersion3::ColorBoostEventData {
    type Target = crate::BeatmapSaveDataVersion3::BeatmapSaveDataItem;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+ColorBoostEventData")]
impl std::ops::DerefMut for crate::BeatmapSaveDataVersion3::ColorBoostEventData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+ColorBoostEventData")]
impl crate::BeatmapSaveDataVersion3::ColorBoostEventData {
    pub fn get_boost(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_boost", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        beat: f32,
        boost: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (beat, boost))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        beat: f32,
        boost: bool,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (beat, boost))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+ColorBoostEventData")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapSaveDataVersion3::ColorBoostEventData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
