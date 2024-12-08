#[cfg(feature = "IBeatmapLevelLoader")]
#[repr(C)]
#[derive(Debug)]
pub struct IBeatmapLevelLoader {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IBeatmapLevelLoader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for IBeatmapLevelLoader => ""."IBeatmapLevelLoader"
);
#[cfg(feature = "IBeatmapLevelLoader")]
impl std::ops::Deref for IBeatmapLevelLoader {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IBeatmapLevelLoader")]
impl std::ops::DerefMut for IBeatmapLevelLoader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IBeatmapLevelLoader")]
impl IBeatmapLevelLoader {
    pub fn add_levelDownloadingUpdateEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            crate::GlobalNamespace::BeatmapLevelLoader_LevelDownloadingUpdate,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_levelDownloadingUpdateEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn LoadBeatmapLevelDataAsync(
        &mut self,
        beatmapLevel: *mut BeatmapLevel,
        beatmapLevelDataVersion: BeatmapLevelDataVersion,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<LoadBeatmapLevelDataResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            LoadBeatmapLevelDataResult,
        > = __cordl_object
            .invoke(
                "LoadBeatmapLevelDataAsync",
                (beatmapLevel, beatmapLevelDataVersion, cancellationToken),
            )?;
        Ok(__cordl_ret)
    }
    pub fn remove_levelDownloadingUpdateEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            crate::GlobalNamespace::BeatmapLevelLoader_LevelDownloadingUpdate,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_levelDownloadingUpdateEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn CheckBeatmapLevelDataExistsAsync(
        &mut self,
        beatmapLevel: *mut BeatmapLevel,
        beatmapLevelDataVersion: BeatmapLevelDataVersion,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<bool>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<bool> = __cordl_object
            .invoke(
                "CheckBeatmapLevelDataExistsAsync",
                (beatmapLevel, beatmapLevelDataVersion, cancellationToken),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ClearCache(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearCache", ())?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "IBeatmapLevelLoader")]
impl quest_hook::libil2cpp::ObjectType for IBeatmapLevelLoader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
