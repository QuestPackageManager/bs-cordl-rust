#[cfg(feature = "BeatmapSaveDataVersion3+LightTranslationEventBoxGroup")]
#[repr(C)]
#[derive(Debug)]
pub struct LightTranslationEventBoxGroup {
    __cordl_parent: crate::BeatmapSaveDataVersion3::EventBoxGroup_1<
        quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion3::LightTranslationEventBox,
        >,
    >,
}
#[cfg(feature = "BeatmapSaveDataVersion3+LightTranslationEventBoxGroup")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatmapSaveDataVersion3::LightTranslationEventBoxGroup => "BeatmapSaveDataVersion3"
    ."LightTranslationEventBoxGroup"
);
#[cfg(feature = "BeatmapSaveDataVersion3+LightTranslationEventBoxGroup")]
impl std::ops::Deref for crate::BeatmapSaveDataVersion3::LightTranslationEventBoxGroup {
    type Target = crate::BeatmapSaveDataVersion3::EventBoxGroup_1<
        quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion3::LightTranslationEventBox,
        >,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+LightTranslationEventBoxGroup")]
impl std::ops::DerefMut
for crate::BeatmapSaveDataVersion3::LightTranslationEventBoxGroup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+LightTranslationEventBoxGroup")]
impl crate::BeatmapSaveDataVersion3::LightTranslationEventBoxGroup {
    pub fn CopyWith(
        &mut self,
        newBeat: crate::System::Nullable_1<f32>,
        newGroupId: crate::System::Nullable_1<i32>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion3::LightTranslationEventBoxGroup,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion3::LightTranslationEventBoxGroup,
        > = __cordl_object.invoke("CopyWith", (newBeat, newGroupId))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        beat: f32,
        groupId: i32,
        eventBoxes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::BeatmapSaveDataVersion3::LightTranslationEventBox,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (beat, groupId, eventBoxes))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        beat: f32,
        groupId: i32,
        eventBoxes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::BeatmapSaveDataVersion3::LightTranslationEventBox,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (beat, groupId, eventBoxes))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+LightTranslationEventBoxGroup")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapSaveDataVersion3::LightTranslationEventBoxGroup {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
