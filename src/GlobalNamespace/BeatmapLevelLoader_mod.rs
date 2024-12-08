#[cfg(feature = "BeatmapLevelLoader")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapLevelLoader {
    __cordl_parent: crate::System::Object,
    pub levelDownloadingUpdateEvent: *mut crate::System::Action_1<
        crate::GlobalNamespace::BeatmapLevelLoader_LevelDownloadingUpdate,
    >,
    pub _loadedBeatmapLevelDataCache: *mut crate::BGLib::DotnetExtension::Collections::LRUCache_2<
        *mut crate::System::String,
        *mut IBeatmapLevelData,
    >,
    pub _beatmapLevelDataLoader: *mut BeatmapLevelDataLoader,
    pub _beatmapDataAssetFileModel: *mut IBeatmapDataAssetFileModel,
    pub _audioClipAsyncLoader: *mut AudioClipAsyncLoader,
}
#[cfg(feature = "BeatmapLevelLoader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BeatmapLevelLoader => ""."BeatmapLevelLoader"
);
#[cfg(feature = "BeatmapLevelLoader")]
impl std::ops::Deref for BeatmapLevelLoader {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelLoader")]
impl std::ops::DerefMut for BeatmapLevelLoader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelLoader")]
impl BeatmapLevelLoader {
    #[cfg(feature = "BeatmapLevelLoader+LevelDownloadingUpdate")]
    pub type LevelDownloadingUpdate = crate::GlobalNamespace::BeatmapLevelLoader_LevelDownloadingUpdate;
    #[cfg(feature = "BeatmapLevelLoader+_CheckBeatmapLevelDataExistsAsync_d__14")]
    pub type _CheckBeatmapLevelDataExistsAsync_d__14 = crate::GlobalNamespace::BeatmapLevelLoader__CheckBeatmapLevelDataExistsAsync_d__14;
    #[cfg(feature = "BeatmapLevelLoader+_LoadBeatmapLevelDataAsync_d__13")]
    pub type _LoadBeatmapLevelDataAsync_d__13 = crate::GlobalNamespace::BeatmapLevelLoader__LoadBeatmapLevelDataAsync_d__13;
    #[cfg(feature = "BeatmapLevelLoader+InitData")]
    pub type InitData = crate::GlobalNamespace::BeatmapLevelLoader_InitData;
    #[cfg(feature = "BeatmapLevelLoader+_DownloadAssetBundleAndGetPath_d__19")]
    pub type _DownloadAssetBundleAndGetPath_d__19 = crate::GlobalNamespace::BeatmapLevelLoader__DownloadAssetBundleAndGetPath_d__19;
    #[cfg(
        feature = "BeatmapLevelLoader+_CheckBeatmapLevelDataFromAssetBundleExistsAsync_d__16"
    )]
    pub type _CheckBeatmapLevelDataFromAssetBundleExistsAsync_d__16 = crate::GlobalNamespace::BeatmapLevelLoader__CheckBeatmapLevelDataFromAssetBundleExistsAsync_d__16;
    #[cfg(feature = "BeatmapLevelLoader+_LoadBeatmapLevelDataFromAssetBundle_d__18")]
    pub type _LoadBeatmapLevelDataFromAssetBundle_d__18 = crate::GlobalNamespace::BeatmapLevelLoader__LoadBeatmapLevelDataFromAssetBundle_d__18;
    pub fn CheckBeatmapLevelDataFromAssetBundleExistsAsync(
        &mut self,
        beatmapLevel: *mut BeatmapLevel,
        beatmapLevelDataVersion: BeatmapLevelDataVersion,
        token: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<bool>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<bool> = __cordl_object
            .invoke(
                "CheckBeatmapLevelDataFromAssetBundleExistsAsync",
                (beatmapLevel, beatmapLevelDataVersion, token),
            )?;
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
    pub fn LoadBeatmapLevelDataFromAssetBundle(
        &mut self,
        beatmapLevel: *mut BeatmapLevel,
        beatmapLevelDataVersion: BeatmapLevelDataVersion,
        token: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<*mut IBeatmapLevelData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            *mut IBeatmapLevelData,
        > = __cordl_object
            .invoke(
                "LoadBeatmapLevelDataFromAssetBundle",
                (beatmapLevel, beatmapLevelDataVersion, token),
            )?;
        Ok(__cordl_ret)
    }
    pub fn DownloadAssetBundleAndGetPath(
        &mut self,
        beatmapLevel: *mut BeatmapLevel,
        beatmapLevelDataVersion: BeatmapLevelDataVersion,
        token: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<GetAssetBundleFileResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            GetAssetBundleFileResult,
        > = __cordl_object
            .invoke(
                "DownloadAssetBundleAndGetPath",
                (beatmapLevel, beatmapLevelDataVersion, token),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        beatmapLevelDataLoader: *mut BeatmapLevelDataLoader,
        beatmapDataAssetFileModel: *mut IBeatmapDataAssetFileModel,
        audioClipAsyncLoader: *mut AudioClipAsyncLoader,
        initData: *mut crate::GlobalNamespace::BeatmapLevelLoader_InitData,
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
        Ok(__cordl_ret)
    }
    pub fn CheckBeatmapLevelDataFromCustomLevels(
        &mut self,
        beatmapLevel: *mut BeatmapLevel,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CheckBeatmapLevelDataFromCustomLevels", (beatmapLevel))?;
        Ok(__cordl_ret)
    }
    pub fn HandleLevelDataAssetDownloadUpdate(
        &mut self,
        update: LevelDataAssetDownloadUpdate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLevelDataAssetDownloadUpdate", (update))?;
        Ok(__cordl_ret)
    }
    pub fn get_cachedLoadRequestCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_cachedLoadRequestCount", ())?;
        Ok(__cordl_ret)
    }
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
    pub fn HandleItemWillBeRemovedFromCache(
        &mut self,
        beatmapLevelId: *mut crate::System::String,
        beatmapLevel: *mut IBeatmapLevelData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleItemWillBeRemovedFromCache", (beatmapLevelId, beatmapLevel))?;
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
    pub fn LoadBeatmapLevelDataFromCustomLevels(
        &mut self,
        beatmapLevel: *mut BeatmapLevel,
    ) -> quest_hook::libil2cpp::Result<*mut IBeatmapLevelData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut IBeatmapLevelData = __cordl_object
            .invoke("LoadBeatmapLevelDataFromCustomLevels", (beatmapLevel))?;
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
    pub fn New(
        beatmapLevelDataLoader: *mut BeatmapLevelDataLoader,
        beatmapDataAssetFileModel: *mut IBeatmapDataAssetFileModel,
        audioClipAsyncLoader: *mut AudioClipAsyncLoader,
        initData: *mut crate::GlobalNamespace::BeatmapLevelLoader_InitData,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
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
        Ok(__cordl_object)
    }
}
#[cfg(feature = "BeatmapLevelLoader")]
impl quest_hook::libil2cpp::ObjectType for BeatmapLevelLoader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapLevelLoader+LevelDownloadingUpdate+DownloadingState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LevelDownloadingUpdate_DownloadingState {
    Completed = 2i32,
    Downloading = 1i32,
    PreparingToDownload = 0i32,
}
#[cfg(feature = "BeatmapLevelLoader+LevelDownloadingUpdate+DownloadingState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::LevelDownloadingUpdate_DownloadingState => ""
    ."BeatmapLevelLoader/LevelDownloadingUpdate/DownloadingState"
);
#[cfg(feature = "BeatmapLevelLoader+InitData")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapLevelLoader_InitData {
    __cordl_parent: crate::System::Object,
    pub maxCachedBeatmapLevels: i32,
}
#[cfg(feature = "BeatmapLevelLoader+InitData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatmapLevelLoader_InitData =>
    ""."BeatmapLevelLoader/InitData"
);
#[cfg(feature = "BeatmapLevelLoader+InitData")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapLevelLoader_InitData {
    type Target = crate::System::Object;
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
    pub fn _ctor(
        &mut self,
        maxCachedBeatmapLevels: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (maxCachedBeatmapLevels))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        maxCachedBeatmapLevels: i32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (maxCachedBeatmapLevels))?;
        Ok(__cordl_object)
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
#[derive(Debug, Clone)]
pub struct BeatmapLevelLoader_LevelDownloadingUpdate {
    pub levelID: *mut crate::System::String,
    pub progress: f32,
    pub downloadingState: crate::GlobalNamespace::LevelDownloadingUpdate_DownloadingState,
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
    pub type DownloadingState = crate::GlobalNamespace::LevelDownloadingUpdate_DownloadingState;
    pub fn _ctor(
        &mut self,
        levelID: *mut crate::System::String,
        progress: f32,
        downloadingState: crate::GlobalNamespace::LevelDownloadingUpdate_DownloadingState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (levelID, progress, downloadingState),
        )?;
        Ok(__cordl_ret)
    }
}
