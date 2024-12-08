#[cfg(feature = "BeatmapObjectExecutionRating")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapObjectExecutionRating {
    __cordl_parent: crate::System::Object,
    pub _time_k__BackingField: f32,
}
#[cfg(feature = "BeatmapObjectExecutionRating")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BeatmapObjectExecutionRating => ""
    ."BeatmapObjectExecutionRating"
);
#[cfg(feature = "BeatmapObjectExecutionRating")]
impl std::ops::Deref for BeatmapObjectExecutionRating {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapObjectExecutionRating")]
impl std::ops::DerefMut for BeatmapObjectExecutionRating {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapObjectExecutionRating")]
impl BeatmapObjectExecutionRating {
    pub fn get_time(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_time", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        _cordl_time: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_time))?;
        Ok(__cordl_ret)
    }
    pub fn New(_cordl_time: f32) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_time))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "BeatmapObjectExecutionRating")]
impl quest_hook::libil2cpp::ObjectType for BeatmapObjectExecutionRating {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
