#[cfg(feature = "cordl_class_OculusBeatmapDataAssetFileModel")]
#[repr(C)]
#[derive(Debug)]
pub struct OculusBeatmapDataAssetFileModel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub levelDataAssetDownloadUpdateEvent: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::LevelDataAssetDownloadUpdate,
    >,
    pub _downloadedAssetBundleFiles: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    >,
    pub _downloadingAssetBundlesQueue: quest_hook::libil2cpp::Gc<u64, f32>,
    pub _assetIdToDownloadingData: quest_hook::libil2cpp::Gc<
        u64,
        crate::GlobalNamespace::OculusBeatmapDataAssetFileModel_AssetBundleDownloadingData,
    >,
    pub _reloadAssetDetailsSemaphoreSlim: quest_hook::libil2cpp::Gc<
        crate::System::Threading::SemaphoreSlim,
    >,
    pub _assetFileToAssetDetails: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::AssetDetails>,
    >,
    pub _oculusPlatformAdditionalContentModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::OculusPlatformAdditionalContentModel,
    >,
}
#[cfg(feature = "cordl_class_OculusBeatmapDataAssetFileModel")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OculusBeatmapDataAssetFileModel {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OculusBeatmapDataAssetFileModel";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "OculusBeatmapDataAssetFileModel")]
impl std::ops::Deref for crate::GlobalNamespace::OculusBeatmapDataAssetFileModel {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OculusBeatmapDataAssetFileModel")]
impl std::ops::DerefMut for crate::GlobalNamespace::OculusBeatmapDataAssetFileModel {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OculusBeatmapDataAssetFileModel")]
impl crate::GlobalNamespace::OculusBeatmapDataAssetFileModel {
    pub const kMaxTimeoutBeforeFail: f32 = 15f32;
    #[cfg(feature = "OculusBeatmapDataAssetFileModel+AssetBundleDownloadingData")]
    pub type AssetBundleDownloadingData = crate::GlobalNamespace::OculusBeatmapDataAssetFileModel_AssetBundleDownloadingData;
    pub fn CancelDownload(
        &mut self,
        assetId: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (u64),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("CancelDownload")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CancelDownload", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (assetId))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetAssetBundleFileForBeatmapLevelAsync(
        &mut self,
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
        beatmapLevelDataVersion: crate::GlobalNamespace::BeatmapLevelDataVersion,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GetAssetBundleFileResult>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::BeatmapLevel,
                            >,
                            crate::GlobalNamespace::BeatmapLevelDataVersion,
                            crate::System::Threading::CancellationToken,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::GetAssetBundleFileResult,
                        >,
                        3usize,
                    >("GetAssetBundleFileForBeatmapLevelAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetAssetBundleFileForBeatmapLevelAsync", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GetAssetBundleFileResult,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (beatmapLevel, beatmapLevelDataVersion, cancellationToken),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetDownloadAssetBundleFileAsync(
        &mut self,
        levelId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        assetDetails: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::AssetDetails,
        >,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GetAssetBundleFileResult>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Oculus::Platform::Models::AssetDetails,
                            >,
                            crate::System::Threading::CancellationToken,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::GetAssetBundleFileResult,
                        >,
                        3usize,
                    >("GetDownloadAssetBundleFileAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetDownloadAssetBundleFileAsync", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GetAssetBundleFileResult,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (levelId, assetDetails, cancellationToken))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleAssetFileDownloadUpdate(
        &mut self,
        msg: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::Oculus::Platform::Models::AssetFileDownloadUpdate,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Gc<
                                crate::Oculus::Platform::Models::AssetFileDownloadUpdate,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("HandleAssetFileDownloadUpdate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HandleAssetFileDownloadUpdate", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (msg))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn MarkDownloadCompleted(
        &mut self,
        assetId: u64,
        isError: bool,
        downloadingData: crate::GlobalNamespace::OculusBeatmapDataAssetFileModel_AssetBundleDownloadingData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u64,
                            bool,
                            crate::GlobalNamespace::OculusBeatmapDataAssetFileModel_AssetBundleDownloadingData,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("MarkDownloadCompleted")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "MarkDownloadCompleted", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (assetId, isError, downloadingData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        oculusPlatformAdditionalContentModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OculusPlatformAdditionalContentModel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (oculusPlatformAdditionalContentModel))?;
        Ok(__cordl_object.into())
    }
    pub fn ReloadAssetDetailsForAllLevelsAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<bool>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::System::Threading::CancellationToken),
                        quest_hook::libil2cpp::Gc<bool>,
                        1usize,
                    >("ReloadAssetDetailsForAllLevelsAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ReloadAssetDetailsForAllLevelsAsync", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<bool> = unsafe {
            cordl_method_info.invoke_unchecked(self, (cancellationToken))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RemoveFromDownloadingDataStructures(
        &mut self,
        assetId: u64,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (u64),
                        bool,
                        1usize,
                    >("RemoveFromDownloadingDataStructures")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RemoveFromDownloadingDataStructures", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (assetId))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Tick(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Tick")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Tick",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryDeleteAssetBundleFileForBeatmapLevelAsync(
        &mut self,
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
        beatmapLevelDataVersion: crate::GlobalNamespace::BeatmapLevelDataVersion,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<bool>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::BeatmapLevel,
                            >,
                            crate::GlobalNamespace::BeatmapLevelDataVersion,
                            crate::System::Threading::CancellationToken,
                        ),
                        quest_hook::libil2cpp::Gc<bool>,
                        3usize,
                    >("TryDeleteAssetBundleFileForBeatmapLevelAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TryDeleteAssetBundleFileForBeatmapLevelAsync", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<bool> = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (beatmapLevel, beatmapLevelDataVersion, cancellationToken),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _Tick_g__GetAssetTime_12_0(
        &mut self,
        id: u64,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(u64), f32, 1usize>("<Tick>g__GetAssetTime|12_0")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "<Tick>g__GetAssetTime|12_0", 1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe {
            cordl_method_info.invoke_unchecked(self, (id))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        oculusPlatformAdditionalContentModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OculusPlatformAdditionalContentModel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::OculusPlatformAdditionalContentModel,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (oculusPlatformAdditionalContentModel))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_levelDataAssetDownloadUpdateEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelDataAssetDownloadUpdate,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::LevelDataAssetDownloadUpdate,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("add_levelDataAssetDownloadUpdateEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "add_levelDataAssetDownloadUpdateEvent", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn remove_levelDataAssetDownloadUpdateEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelDataAssetDownloadUpdate,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::LevelDataAssetDownloadUpdate,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("remove_levelDataAssetDownloadUpdateEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "remove_levelDataAssetDownloadUpdateEvent", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_OculusBeatmapDataAssetFileModel")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OculusBeatmapDataAssetFileModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OculusBeatmapDataAssetFileModel")]
impl AsRef<crate::GlobalNamespace::IBeatmapDataAssetFileModel>
for crate::GlobalNamespace::OculusBeatmapDataAssetFileModel {
    fn as_ref(&self) -> &crate::GlobalNamespace::IBeatmapDataAssetFileModel {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OculusBeatmapDataAssetFileModel")]
impl AsMut<crate::GlobalNamespace::IBeatmapDataAssetFileModel>
for crate::GlobalNamespace::OculusBeatmapDataAssetFileModel {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IBeatmapDataAssetFileModel {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OculusBeatmapDataAssetFileModel")]
impl AsRef<crate::Zenject::ITickable>
for crate::GlobalNamespace::OculusBeatmapDataAssetFileModel {
    fn as_ref(&self) -> &crate::Zenject::ITickable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OculusBeatmapDataAssetFileModel")]
impl AsMut<crate::Zenject::ITickable>
for crate::GlobalNamespace::OculusBeatmapDataAssetFileModel {
    fn as_mut(&mut self) -> &mut crate::Zenject::ITickable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "cordl_class_OculusBeatmapDataAssetFileModel+AssetBundleDownloadingData"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OculusBeatmapDataAssetFileModel_AssetBundleDownloadingData {
    pub levelId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub assetBundlePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub downloadAssetBundleFileTCS: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GetAssetBundleFileResult,
    >,
}
#[cfg(
    feature = "cordl_class_OculusBeatmapDataAssetFileModel+AssetBundleDownloadingData"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OculusBeatmapDataAssetFileModel_AssetBundleDownloadingData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OculusBeatmapDataAssetFileModel/AssetBundleDownloadingData";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(
    feature = "cordl_class_OculusBeatmapDataAssetFileModel+AssetBundleDownloadingData"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::OculusBeatmapDataAssetFileModel_AssetBundleDownloadingData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_OculusBeatmapDataAssetFileModel+AssetBundleDownloadingData"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::OculusBeatmapDataAssetFileModel_AssetBundleDownloadingData {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(
    feature = "cordl_class_OculusBeatmapDataAssetFileModel+AssetBundleDownloadingData"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::OculusBeatmapDataAssetFileModel_AssetBundleDownloadingData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(
    feature = "cordl_class_OculusBeatmapDataAssetFileModel+AssetBundleDownloadingData"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::OculusBeatmapDataAssetFileModel_AssetBundleDownloadingData {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(
    feature = "cordl_class_OculusBeatmapDataAssetFileModel+AssetBundleDownloadingData"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OculusBeatmapDataAssetFileModel_AssetBundleDownloadingData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OculusBeatmapDataAssetFileModel+AssetBundleDownloadingData")]
impl crate::GlobalNamespace::OculusBeatmapDataAssetFileModel_AssetBundleDownloadingData {
    pub fn _ctor(
        &mut self,
        levelId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        assetBundlePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::System::Threading::CancellationToken,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (levelId, assetBundlePath, cancellationToken))?
        };
        Ok(__cordl_ret.into())
    }
}
