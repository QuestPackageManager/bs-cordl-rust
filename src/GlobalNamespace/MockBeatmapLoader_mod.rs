#[cfg(feature = "MockBeatmapLoader")]
#[repr(C)]
#[derive(Debug)]
pub struct MockBeatmapLoader {
    __cordl_parent: crate::System::Object,
    pub _beatmapLevelsModel: *mut BeatmapLevelsModel,
    pub _beatmapDataLoader: *mut BeatmapDataLoader,
    pub _environmentsListModel: *mut EnvironmentsListModel,
    pub _beatmapCharacteristicCollection: *mut BeatmapCharacteristicCollection,
}
#[cfg(feature = "MockBeatmapLoader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MockBeatmapLoader => ""."MockBeatmapLoader"
);
#[cfg(feature = "MockBeatmapLoader")]
impl std::ops::Deref for MockBeatmapLoader {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MockBeatmapLoader")]
impl std::ops::DerefMut for MockBeatmapLoader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MockBeatmapLoader")]
impl MockBeatmapLoader {
    #[cfg(feature = "MockBeatmapLoader+_GetBeatmapData_d__5")]
    pub type _GetBeatmapData_d__5 = crate::GlobalNamespace::MockBeatmapLoader__GetBeatmapData_d__5;
    pub fn _ctor(
        &mut self,
        beatmapLevelsModel: *mut BeatmapLevelsModel,
        beatmapDataLoader: *mut BeatmapDataLoader,
        environmentsListModel: *mut EnvironmentsListModel,
        beatmapCharacteristicCollection: *mut BeatmapCharacteristicCollection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    beatmapLevelsModel,
                    beatmapDataLoader,
                    environmentsListModel,
                    beatmapCharacteristicCollection,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetBeatmapData(
        &mut self,
        beatmap: *mut BeatmapKeyNetSerializable,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<*mut MockBeatmapData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            *mut MockBeatmapData,
        > = __cordl_object.invoke("GetBeatmapData", (beatmap, cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        beatmapLevelsModel: *mut BeatmapLevelsModel,
        beatmapDataLoader: *mut BeatmapDataLoader,
        environmentsListModel: *mut EnvironmentsListModel,
        beatmapCharacteristicCollection: *mut BeatmapCharacteristicCollection,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    beatmapLevelsModel,
                    beatmapDataLoader,
                    environmentsListModel,
                    beatmapCharacteristicCollection,
                ),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "MockBeatmapLoader")]
impl quest_hook::libil2cpp::ObjectType for MockBeatmapLoader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
