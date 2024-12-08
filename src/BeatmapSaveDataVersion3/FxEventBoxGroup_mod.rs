#[cfg(feature = "BeatmapSaveDataVersion3+FxEventBoxGroup")]
#[repr(C)]
#[derive(Debug)]
pub struct FxEventBoxGroup {
    __cordl_parent: crate::BeatmapSaveDataVersion3::EventBoxGroup_1<
        *mut crate::BeatmapSaveDataVersion3::FxEventBox,
    >,
    pub t: crate::BeatmapSaveDataVersion3::FxEventType,
}
#[cfg(feature = "BeatmapSaveDataVersion3+FxEventBoxGroup")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatmapSaveDataVersion3::FxEventBoxGroup =>
    "BeatmapSaveDataVersion3"."FxEventBoxGroup"
);
#[cfg(feature = "BeatmapSaveDataVersion3+FxEventBoxGroup")]
impl std::ops::Deref for crate::BeatmapSaveDataVersion3::FxEventBoxGroup {
    type Target = crate::BeatmapSaveDataVersion3::EventBoxGroup_1<
        *mut crate::BeatmapSaveDataVersion3::FxEventBox,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+FxEventBoxGroup")]
impl std::ops::DerefMut for crate::BeatmapSaveDataVersion3::FxEventBoxGroup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+FxEventBoxGroup")]
impl crate::BeatmapSaveDataVersion3::FxEventBoxGroup {
    pub fn get_type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::BeatmapSaveDataVersion3::FxEventType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BeatmapSaveDataVersion3::FxEventType = __cordl_object
            .invoke("get_type", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        beat: f32,
        groupId: i32,
        _cordl_type: crate::BeatmapSaveDataVersion3::FxEventType,
        eventBoxes: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion3::FxEventBox,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (beat, groupId, _cordl_type, eventBoxes))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        beat: f32,
        groupId: i32,
        _cordl_type: crate::BeatmapSaveDataVersion3::FxEventType,
        eventBoxes: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion3::FxEventBox,
        >,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (beat, groupId, _cordl_type, eventBoxes))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+FxEventBoxGroup")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapSaveDataVersion3::FxEventBoxGroup {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
