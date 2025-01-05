#[cfg(feature = "BeatmapDataLoaderVersion4+NjsEventConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct NjsEventConverter {
    __cordl_parent: crate::GlobalNamespace::BeatToTimeConverter,
    pub _noteJumpMovementSpeedEvents: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::BeatmapSaveDataVersion4::NoteJumpMovementSpeedEvent,
        >,
    >,
}
#[cfg(feature = "BeatmapDataLoaderVersion4+NjsEventConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatmapDataLoaderVersion4::NjsEventConverter =>
    "BeatmapDataLoaderVersion4"."NjsEventConverter"
);
#[cfg(feature = "BeatmapDataLoaderVersion4+NjsEventConverter")]
impl std::ops::Deref for crate::BeatmapDataLoaderVersion4::NjsEventConverter {
    type Target = crate::GlobalNamespace::BeatToTimeConverter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion4+NjsEventConverter")]
impl std::ops::DerefMut for crate::BeatmapDataLoaderVersion4::NjsEventConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion4+NjsEventConverter")]
impl crate::BeatmapDataLoaderVersion4::NjsEventConverter {
    pub fn Convert(
        &mut self,
        index: quest_hook::libil2cpp::Gc<crate::BeatmapSaveDataVersion4::BeatIndex>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteJumpSpeedEventData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::NoteJumpSpeedEventData,
        > = __cordl_object.invoke("Convert", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        noteJumpMovementSpeedEvents: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::BeatmapSaveDataVersion4::NoteJumpMovementSpeedEvent,
            >,
        >,
        bpmTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatToTimeConverter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (noteJumpMovementSpeedEvents, bpmTimeProcessor))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        noteJumpMovementSpeedEvents: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::BeatmapSaveDataVersion4::NoteJumpMovementSpeedEvent,
            >,
        >,
        bpmTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatToTimeConverter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (noteJumpMovementSpeedEvents, bpmTimeProcessor))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion4+NjsEventConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion4::NjsEventConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
