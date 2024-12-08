#[cfg(feature = "BeatmapLevelSaveDataVersion4+BpmData")]
#[repr(C)]
#[derive(Debug)]
pub struct BpmData {
    __cordl_parent: crate::System::Object,
    pub si: i32,
    pub ei: i32,
    pub sb: f32,
    pub eb: f32,
}
#[cfg(feature = "BeatmapLevelSaveDataVersion4+BpmData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatmapLevelSaveDataVersion4::BpmData =>
    "BeatmapLevelSaveDataVersion4"."BpmData"
);
#[cfg(feature = "BeatmapLevelSaveDataVersion4+BpmData")]
impl std::ops::Deref for crate::BeatmapLevelSaveDataVersion4::BpmData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelSaveDataVersion4+BpmData")]
impl std::ops::DerefMut for crate::BeatmapLevelSaveDataVersion4::BpmData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelSaveDataVersion4+BpmData")]
impl crate::BeatmapLevelSaveDataVersion4::BpmData {
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
    pub fn get_beat(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_beat", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatmapLevelSaveDataVersion4+BpmData")]
impl quest_hook::libil2cpp::ObjectType for crate::BeatmapLevelSaveDataVersion4::BpmData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
