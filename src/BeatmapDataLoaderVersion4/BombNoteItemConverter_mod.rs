#[cfg(feature = "BeatmapDataLoaderVersion4+BombNoteItemConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct BombNoteItemConverter {
    __cordl_parent: BeatToTimeConverterProvider,
    pub _bombNotes: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::BeatmapSaveDataVersion4::BombNote,
    >,
}
#[cfg(feature = "BeatmapDataLoaderVersion4+BombNoteItemConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatmapDataLoaderVersion4::BombNoteItemConverter
    => "BeatmapDataLoaderVersion4"."BombNoteItemConverter"
);
#[cfg(feature = "BeatmapDataLoaderVersion4+BombNoteItemConverter")]
impl std::ops::Deref for crate::BeatmapDataLoaderVersion4::BombNoteItemConverter {
    type Target = BeatToTimeConverterProvider;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion4+BombNoteItemConverter")]
impl std::ops::DerefMut for crate::BeatmapDataLoaderVersion4::BombNoteItemConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion4+BombNoteItemConverter")]
impl crate::BeatmapDataLoaderVersion4::BombNoteItemConverter {
    pub fn Convert(
        &mut self,
        index: *mut crate::BeatmapSaveDataVersion4::BeatmapBeatIndex,
    ) -> quest_hook::libil2cpp::Result<*mut BeatmapObjectData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut BeatmapObjectData = __cordl_object
            .invoke("Convert", (index))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        bombNotes: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::BeatmapSaveDataVersion4::BombNote,
        >,
        bpmTimeProcessor: *mut BpmTimeProcessor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bombNotes, bpmTimeProcessor))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        bombNotes: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::BeatmapSaveDataVersion4::BombNote,
        >,
        bpmTimeProcessor: *mut BpmTimeProcessor,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bombNotes, bpmTimeProcessor))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion4+BombNoteItemConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion4::BombNoteItemConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
