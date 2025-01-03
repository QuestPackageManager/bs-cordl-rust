#[cfg(feature = "MockBeatmapLoader")]
#[repr(C)]
#[derive(Debug)]
pub struct MockBeatmapLoader {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _beatmapLevelsModel: *mut crate::GlobalNamespace::BeatmapLevelsModel,
    pub _beatmapDataLoader: *mut crate::GlobalNamespace::BeatmapDataLoader,
    pub _environmentsListModel: *mut crate::GlobalNamespace::EnvironmentsListModel,
    pub _beatmapCharacteristicCollection: *mut crate::GlobalNamespace::BeatmapCharacteristicCollection,
}
#[cfg(feature = "MockBeatmapLoader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MockBeatmapLoader => ""
    ."MockBeatmapLoader"
);
#[cfg(feature = "MockBeatmapLoader")]
impl std::ops::Deref for crate::GlobalNamespace::MockBeatmapLoader {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MockBeatmapLoader")]
impl std::ops::DerefMut for crate::GlobalNamespace::MockBeatmapLoader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MockBeatmapLoader")]
impl crate::GlobalNamespace::MockBeatmapLoader {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBeatmapData(
        &mut self,
        beatmap: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapKeyNetSerializable,
        >,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut crate::GlobalNamespace::MockBeatmapData,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut crate::GlobalNamespace::MockBeatmapData,
            >,
        > = __cordl_object.invoke("GetBeatmapData", (beatmap, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        beatmapLevelsModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelsModel,
        >,
        beatmapDataLoader: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapDataLoader,
        >,
        environmentsListModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentsListModel,
        >,
        beatmapCharacteristicCollection: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCharacteristicCollection,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
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
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        beatmapLevelsModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelsModel,
        >,
        beatmapDataLoader: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapDataLoader,
        >,
        environmentsListModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentsListModel,
        >,
        beatmapCharacteristicCollection: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCharacteristicCollection,
        >,
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
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MockBeatmapLoader")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::MockBeatmapLoader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MockBeatmapLoader")]
impl AsRef<crate::GlobalNamespace::IMockBeatmapDataProvider>
for crate::GlobalNamespace::MockBeatmapLoader {
    fn as_ref(&self) -> &crate::GlobalNamespace::IMockBeatmapDataProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "MockBeatmapLoader")]
impl AsMut<crate::GlobalNamespace::IMockBeatmapDataProvider>
for crate::GlobalNamespace::MockBeatmapLoader {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IMockBeatmapDataProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "MockBeatmapLoader")]
impl AsRef<crate::System::IDisposable> for crate::GlobalNamespace::MockBeatmapLoader {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "MockBeatmapLoader")]
impl AsMut<crate::System::IDisposable> for crate::GlobalNamespace::MockBeatmapLoader {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
