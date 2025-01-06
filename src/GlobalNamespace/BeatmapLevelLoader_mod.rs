#[cfg(feature = "BeatmapLevelLoader")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapLevelLoader {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub levelDownloadingUpdateEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            crate::GlobalNamespace::BeatmapLevelLoader_LevelDownloadingUpdate,
        >,
    >,
    pub _loadedBeatmapLevelDataCache: quest_hook::libil2cpp::Gc<
        crate::BGLib::DotnetExtension::Collections::LRUCache_2<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatmapLevelData>,
        >,
    >,
    pub _beatmapLevelDataLoader: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapLevelDataLoader,
    >,
    pub _beatmapDataAssetFileModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IBeatmapDataAssetFileModel,
    >,
    pub _audioClipAsyncLoader: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::AudioClipAsyncLoader,
    >,
}
#[cfg(feature = "BeatmapLevelLoader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatmapLevelLoader => ""
    ."BeatmapLevelLoader"
);
#[cfg(feature = "BeatmapLevelLoader")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapLevelLoader {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelLoader")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapLevelLoader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelLoader")]
impl crate::GlobalNamespace::BeatmapLevelLoader {
    #[cfg(feature = "BeatmapLevelLoader+InitData")]
    pub type InitData = crate::GlobalNamespace::BeatmapLevelLoader_InitData;
    #[cfg(feature = "BeatmapLevelLoader+LevelDownloadingUpdate")]
    pub type LevelDownloadingUpdate = crate::GlobalNamespace::BeatmapLevelLoader_LevelDownloadingUpdate;
    pub fn CheckBeatmapLevelDataExistsAsync(
        &mut self,
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
        beatmapLevelDataVersion: crate::GlobalNamespace::BeatmapLevelDataVersion,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<bool>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<bool>,
        > = __cordl_object
            .invoke(
                "CheckBeatmapLevelDataExistsAsync",
                (beatmapLevel, beatmapLevelDataVersion, cancellationToken),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckBeatmapLevelDataFromAssetBundleExistsAsync(
        &mut self,
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
        beatmapLevelDataVersion: crate::GlobalNamespace::BeatmapLevelDataVersion,
        token: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<bool>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<bool>,
        > = __cordl_object
            .invoke(
                "CheckBeatmapLevelDataFromAssetBundleExistsAsync",
                (beatmapLevel, beatmapLevelDataVersion, token),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckBeatmapLevelDataFromCustomLevels(
        &mut self,
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CheckBeatmapLevelDataFromCustomLevels", (beatmapLevel))?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearCache(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearCache", ())?;
        Ok(__cordl_ret.into())
    }
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
    pub fn DownloadAssetBundleAndGetPath(
        &mut self,
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
        beatmapLevelDataVersion: crate::GlobalNamespace::BeatmapLevelDataVersion,
        token: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::GetAssetBundleFileResult,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::GetAssetBundleFileResult,
            >,
        > = __cordl_object
            .invoke(
                "DownloadAssetBundleAndGetPath",
                (beatmapLevel, beatmapLevelDataVersion, token),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleItemWillBeRemovedFromCache(
        &mut self,
        beatmapLevelId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        beatmapLevel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatmapLevelData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleItemWillBeRemovedFromCache", (beatmapLevelId, beatmapLevel))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleLevelDataAssetDownloadUpdate(
        &mut self,
        update: crate::GlobalNamespace::LevelDataAssetDownloadUpdate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLevelDataAssetDownloadUpdate", (update))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadBeatmapLevelDataAsync(
        &mut self,
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
        beatmapLevelDataVersion: crate::GlobalNamespace::BeatmapLevelDataVersion,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::LoadBeatmapLevelDataResult,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::LoadBeatmapLevelDataResult,
            >,
        > = __cordl_object
            .invoke(
                "LoadBeatmapLevelDataAsync",
                (beatmapLevel, beatmapLevelDataVersion, cancellationToken),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadBeatmapLevelDataFromAssetBundle(
        &mut self,
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
        beatmapLevelDataVersion: crate::GlobalNamespace::BeatmapLevelDataVersion,
        token: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatmapLevelData>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatmapLevelData>,
            >,
        > = __cordl_object
            .invoke(
                "LoadBeatmapLevelDataFromAssetBundle",
                (beatmapLevel, beatmapLevelDataVersion, token),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadBeatmapLevelDataFromCustomLevels(
        &mut self,
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatmapLevelData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatmapLevelData,
        > = __cordl_object
            .invoke("LoadBeatmapLevelDataFromCustomLevels", (beatmapLevel))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        beatmapLevelDataLoader: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelDataLoader,
        >,
        beatmapDataAssetFileModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatmapDataAssetFileModel,
        >,
        audioClipAsyncLoader: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::AudioClipAsyncLoader,
        >,
        initData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelLoader_InitData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    beatmapLevelDataLoader,
                    beatmapDataAssetFileModel,
                    audioClipAsyncLoader,
                    initData,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        beatmapLevelDataLoader: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelDataLoader,
        >,
        beatmapDataAssetFileModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatmapDataAssetFileModel,
        >,
        audioClipAsyncLoader: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::AudioClipAsyncLoader,
        >,
        initData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelLoader_InitData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    beatmapLevelDataLoader,
                    beatmapDataAssetFileModel,
                    audioClipAsyncLoader,
                    initData,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn add_levelDownloadingUpdateEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                crate::GlobalNamespace::BeatmapLevelLoader_LevelDownloadingUpdate,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_levelDownloadingUpdateEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_cachedLoadRequestCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_cachedLoadRequestCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_levelDownloadingUpdateEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                crate::GlobalNamespace::BeatmapLevelLoader_LevelDownloadingUpdate,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_levelDownloadingUpdateEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapLevelLoader")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::BeatmapLevelLoader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapLevelLoader")]
impl AsRef<crate::GlobalNamespace::IBeatmapLevelLoader>
for crate::GlobalNamespace::BeatmapLevelLoader {
    fn as_ref(&self) -> &crate::GlobalNamespace::IBeatmapLevelLoader {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatmapLevelLoader")]
impl AsMut<crate::GlobalNamespace::IBeatmapLevelLoader>
for crate::GlobalNamespace::BeatmapLevelLoader {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IBeatmapLevelLoader {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatmapLevelLoader")]
impl AsRef<crate::System::IDisposable> for crate::GlobalNamespace::BeatmapLevelLoader {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatmapLevelLoader")]
impl AsMut<crate::System::IDisposable> for crate::GlobalNamespace::BeatmapLevelLoader {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatmapLevelLoader+InitData")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapLevelLoader_InitData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub maxCachedBeatmapLevels: i32,
}
#[cfg(feature = "BeatmapLevelLoader+InitData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatmapLevelLoader_InitData =>
    ""."BeatmapLevelLoader/InitData"
);
#[cfg(feature = "BeatmapLevelLoader+InitData")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapLevelLoader_InitData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelLoader+InitData")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapLevelLoader_InitData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelLoader+InitData")]
impl crate::GlobalNamespace::BeatmapLevelLoader_InitData {
    pub fn New(
        maxCachedBeatmapLevels: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (maxCachedBeatmapLevels))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        maxCachedBeatmapLevels: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (maxCachedBeatmapLevels))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapLevelLoader+InitData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapLevelLoader_InitData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapLevelLoader+LevelDownloadingUpdate")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BeatmapLevelLoader_LevelDownloadingUpdate {
    pub levelID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub progress: f32,
    pub downloadingState: crate::GlobalNamespace::LevelDownloadingUpdate_BeatmapLevelLoader_DownloadingState,
}
#[cfg(feature = "BeatmapLevelLoader+LevelDownloadingUpdate")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BeatmapLevelLoader_LevelDownloadingUpdate => ""
    ."BeatmapLevelLoader/LevelDownloadingUpdate"
);
#[cfg(feature = "BeatmapLevelLoader+LevelDownloadingUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::BeatmapLevelLoader_LevelDownloadingUpdate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatmapLevelLoader+LevelDownloadingUpdate")]
impl crate::GlobalNamespace::BeatmapLevelLoader_LevelDownloadingUpdate {
    #[cfg(feature = "BeatmapLevelLoader+LevelDownloadingUpdate+DownloadingState")]
    pub type DownloadingState = crate::GlobalNamespace::LevelDownloadingUpdate_BeatmapLevelLoader_DownloadingState;
    pub fn _ctor(
        &mut self,
        levelID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        progress: f32,
        downloadingState: crate::GlobalNamespace::LevelDownloadingUpdate_BeatmapLevelLoader_DownloadingState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (levelID, progress, downloadingState),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapLevelLoader+LevelDownloadingUpdate+DownloadingState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LevelDownloadingUpdate_BeatmapLevelLoader_DownloadingState {
    #[default]
    Completed = 2i32,
    Downloading = 1i32,
    PreparingToDownload = 0i32,
}
#[cfg(feature = "BeatmapLevelLoader+LevelDownloadingUpdate+DownloadingState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::LevelDownloadingUpdate_BeatmapLevelLoader_DownloadingState => ""
    ."BeatmapLevelLoader/LevelDownloadingUpdate/DownloadingState"
);
