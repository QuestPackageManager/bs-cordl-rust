#[cfg(feature = "BeatmapDataLoaderVersion4+ArcItemConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct ArcItemConverter {
    __cordl_parent: crate::GlobalNamespace::BeatToTimeConverterProvider,
    pub _colorNotes: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::BeatmapSaveDataVersion4::ColorNote,
    >,
    pub _arcs: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::BeatmapSaveDataVersion4::Arc,
    >,
}
#[cfg(feature = "BeatmapDataLoaderVersion4+ArcItemConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatmapDataLoaderVersion4::ArcItemConverter =>
    "BeatmapDataLoaderVersion4"."ArcItemConverter"
);
#[cfg(feature = "BeatmapDataLoaderVersion4+ArcItemConverter")]
impl std::ops::Deref for crate::BeatmapDataLoaderVersion4::ArcItemConverter {
    type Target = crate::GlobalNamespace::BeatToTimeConverterProvider;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion4+ArcItemConverter")]
impl std::ops::DerefMut for crate::BeatmapDataLoaderVersion4::ArcItemConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion4+ArcItemConverter")]
impl crate::BeatmapDataLoaderVersion4::ArcItemConverter {
    pub fn Convert(
        &mut self,
        index: *mut crate::BeatmapSaveDataVersion4::ArcBeatIndex,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::BeatmapObjectData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::BeatmapObjectData = __cordl_object
            .invoke("Convert", (index))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        colorNotes: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::BeatmapSaveDataVersion4::ColorNote,
        >,
        arcs: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::BeatmapSaveDataVersion4::Arc,
        >,
        bpmTimeProcessor: *mut crate::GlobalNamespace::BpmTimeProcessor,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (colorNotes, arcs, bpmTimeProcessor))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        colorNotes: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::BeatmapSaveDataVersion4::ColorNote,
        >,
        arcs: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::BeatmapSaveDataVersion4::Arc,
        >,
        bpmTimeProcessor: *mut crate::GlobalNamespace::BpmTimeProcessor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (colorNotes, arcs, bpmTimeProcessor))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion4+ArcItemConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion4::ArcItemConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
