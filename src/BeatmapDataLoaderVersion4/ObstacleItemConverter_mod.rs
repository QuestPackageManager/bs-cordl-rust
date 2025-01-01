#[cfg(feature = "BeatmapDataLoaderVersion4+ObstacleItemConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct ObstacleItemConverter {
    __cordl_parent: crate::GlobalNamespace::BeatToTimeConverter,
    pub _obstacles: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::BeatmapSaveDataVersion4::Obstacle,
    >,
}
#[cfg(feature = "BeatmapDataLoaderVersion4+ObstacleItemConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatmapDataLoaderVersion4::ObstacleItemConverter
    => "BeatmapDataLoaderVersion4"."ObstacleItemConverter"
);
#[cfg(feature = "BeatmapDataLoaderVersion4+ObstacleItemConverter")]
impl std::ops::Deref for crate::BeatmapDataLoaderVersion4::ObstacleItemConverter {
    type Target = crate::GlobalNamespace::BeatToTimeConverter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion4+ObstacleItemConverter")]
impl std::ops::DerefMut for crate::BeatmapDataLoaderVersion4::ObstacleItemConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion4+ObstacleItemConverter")]
impl crate::BeatmapDataLoaderVersion4::ObstacleItemConverter {
    pub fn Convert(
        &mut self,
        index: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion4::BeatmapBeatIndex,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapObjectData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapObjectData,
        > = __cordl_object.invoke("Convert", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        obstacles: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::BeatmapSaveDataVersion4::Obstacle>,
        >,
        bpmTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BpmTimeProcessor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (obstacles, bpmTimeProcessor))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        obstacles: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::BeatmapSaveDataVersion4::Obstacle>,
        >,
        bpmTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BpmTimeProcessor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (obstacles, bpmTimeProcessor))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion4+ObstacleItemConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion4::ObstacleItemConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
