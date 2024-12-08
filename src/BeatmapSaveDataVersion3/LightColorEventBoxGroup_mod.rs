#[cfg(feature = "BeatmapSaveDataVersion3+LightColorEventBoxGroup")]
#[repr(C)]
#[derive(Debug)]
pub struct LightColorEventBoxGroup {
    __cordl_parent: crate::BeatmapSaveDataVersion3::EventBoxGroup_1<
        *mut crate::BeatmapSaveDataVersion3::LightColorEventBox,
    >,
}
#[cfg(feature = "BeatmapSaveDataVersion3+LightColorEventBoxGroup")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatmapSaveDataVersion3::LightColorEventBoxGroup
    => "BeatmapSaveDataVersion3"."LightColorEventBoxGroup"
);
#[cfg(feature = "BeatmapSaveDataVersion3+LightColorEventBoxGroup")]
impl std::ops::Deref for crate::BeatmapSaveDataVersion3::LightColorEventBoxGroup {
    type Target = crate::BeatmapSaveDataVersion3::EventBoxGroup_1<
        *mut crate::BeatmapSaveDataVersion3::LightColorEventBox,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+LightColorEventBoxGroup")]
impl std::ops::DerefMut for crate::BeatmapSaveDataVersion3::LightColorEventBoxGroup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+LightColorEventBoxGroup")]
impl crate::BeatmapSaveDataVersion3::LightColorEventBoxGroup {
    pub fn _ctor(
        &mut self,
        beat: f32,
        groupId: i32,
        eventBoxes: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion3::LightColorEventBox,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (beat, groupId, eventBoxes))?;
        Ok(__cordl_ret)
    }
    pub fn CopyWith(
        &mut self,
        newBeat: crate::System::Nullable_1<f32>,
        newGroupId: crate::System::Nullable_1<i32>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::BeatmapSaveDataVersion3::LightColorEventBoxGroup,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::BeatmapSaveDataVersion3::LightColorEventBoxGroup = __cordl_object
            .invoke("CopyWith", (newBeat, newGroupId))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        beat: f32,
        groupId: i32,
        eventBoxes: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion3::LightColorEventBox,
        >,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (beat, groupId, eventBoxes))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+LightColorEventBoxGroup")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapSaveDataVersion3::LightColorEventBoxGroup {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
