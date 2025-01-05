#[cfg(feature = "BeatmapDataLoaderVersion4+EventBoxGroupConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct EventBoxGroupConverter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub lightshowSaveData: quest_hook::libil2cpp::Gc<
        crate::BeatmapSaveDataVersion4::LightshowSaveData,
    >,
    pub _lightGroups: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IEnvironmentLightGroups,
    >,
}
#[cfg(feature = "BeatmapDataLoaderVersion4+EventBoxGroupConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatmapDataLoaderVersion4::EventBoxGroupConverter => "BeatmapDataLoaderVersion4"
    ."EventBoxGroupConverter"
);
#[cfg(feature = "BeatmapDataLoaderVersion4+EventBoxGroupConverter")]
impl std::ops::Deref for crate::BeatmapDataLoaderVersion4::EventBoxGroupConverter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion4+EventBoxGroupConverter")]
impl std::ops::DerefMut for crate::BeatmapDataLoaderVersion4::EventBoxGroupConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion4+EventBoxGroupConverter")]
impl crate::BeatmapDataLoaderVersion4::EventBoxGroupConverter {
    pub fn Convert(
        &mut self,
        eventBoxGroup: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion4::EventBoxGroup,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventDataBoxGroup>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapEventDataBoxGroup,
        > = __cordl_object.invoke("Convert", (eventBoxGroup))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertEvents(
        &mut self,
        eventBox: crate::BeatmapSaveDataVersion4::EventBox,
        indexFilter: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IndexFilter>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventDataBox>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapEventDataBox,
        > = __cordl_object.invoke("ConvertEvents", (eventBox, indexFilter))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        lightshowSaveData: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion4::LightshowSaveData,
        >,
        lightGroups: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IEnvironmentLightGroups,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (lightshowSaveData, lightGroups))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        lightshowSaveData: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion4::LightshowSaveData,
        >,
        lightGroups: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IEnvironmentLightGroups,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (lightshowSaveData, lightGroups))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion4+EventBoxGroupConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion4::EventBoxGroupConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
