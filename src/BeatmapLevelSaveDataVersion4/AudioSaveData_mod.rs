#[cfg(feature = "BeatmapLevelSaveDataVersion4+AudioSaveData")]
#[repr(C)]
#[derive(Debug)]
pub struct AudioSaveData {
    __cordl_parent: crate::System::Object,
    pub version: *mut crate::System::String,
    pub songChecksum: *mut crate::System::String,
    pub songSampleCount: i32,
    pub songFrequency: i32,
    pub bpmData: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::BeatmapLevelSaveDataVersion4::BpmData,
    >,
    pub lufsData: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::BeatmapLevelSaveDataVersion4::LufsData,
    >,
}
#[cfg(feature = "BeatmapLevelSaveDataVersion4+AudioSaveData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatmapLevelSaveDataVersion4::AudioSaveData =>
    "BeatmapLevelSaveDataVersion4"."AudioSaveData"
);
#[cfg(feature = "BeatmapLevelSaveDataVersion4+AudioSaveData")]
impl std::ops::Deref for crate::BeatmapLevelSaveDataVersion4::AudioSaveData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelSaveDataVersion4+AudioSaveData")]
impl std::ops::DerefMut for crate::BeatmapLevelSaveDataVersion4::AudioSaveData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelSaveDataVersion4+AudioSaveData")]
impl crate::BeatmapLevelSaveDataVersion4::AudioSaveData {
    pub const kCurrentVersion: &'static str = "4.0.0";
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
}
#[cfg(feature = "BeatmapLevelSaveDataVersion4+AudioSaveData")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapLevelSaveDataVersion4::AudioSaveData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
